use encoding_rs::WINDOWS_1252;
use enum_map::EnumMap;
use rand::{thread_rng, Rng};

use std::collections::VecDeque;
use std::iter::FusedIterator;
use std::path::{Path, PathBuf};
use std::{cmp, fs, io};

use crate::cost::cost_of_typing;
use crate::cost::layout_cost;

use crate::layout::{Key, Layout, Win1252Char, NUM_KEYS};

#[derive(Clone)]
pub struct Keys<'l, I> {
    layout: &'l AnnotatedLayout,
    chars: I,
    cur_layer: u8,
    cur_shifted: bool,
    buf: VecDeque<TypingEvent>,
}

impl<'l, I> Keys<'l, I>
where
    I: Iterator<Item = Win1252Char>,
{
    fn extend_buf_to(&mut self, n: usize) {
        while self.buf.len() <= n {
            if !self.handle_next() {
                break;
            }
        }
    }

    fn handle_next(&mut self) -> bool {
        if let Some(c) = self.chars.next() {
            match self.layout.char_idx[c] {
                Some(CharIdxEntry {
                    layer,
                    pos,
                    shifted,
                }) => {
                    // A typable character.
                    if self.cur_layer != 0 && layer != self.cur_layer {
                        self.buf.push_back(TypingEvent::Release(
                            self.layout.layer_idx[self.cur_layer as usize],
                        ));
                        self.cur_layer = 0;
                    }
                    if self.cur_shifted && !shifted {
                        self.buf
                            .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = false;
                    }

                    if shifted && !self.cur_shifted {
                        // The shift key is on the home layer.
                        if self.cur_layer != 0 {
                            self.buf.push_back(TypingEvent::Release(
                                self.layout.layer_idx[self.cur_layer as usize],
                            ));
                            self.cur_layer = 0;
                        }
                        self.buf
                            .push_back(TypingEvent::Hold(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = true;
                    }
                    if layer != 0 && self.cur_layer != layer {
                        self.buf
                            .push_back(TypingEvent::Hold(self.layout.layer_idx[layer as usize]));
                        self.cur_layer = layer;
                    }

                    self.buf.push_back(TypingEvent::Tap {
                        pos,
                        for_char: true,
                    });
                    true
                }
                None => {
                    // An untypable character.
                    if self.cur_layer != 0 {
                        self.buf.push_back(TypingEvent::Release(
                            self.layout.layer_idx[self.cur_layer as usize],
                        ));
                        self.cur_layer = 0;
                    }
                    if self.cur_shifted {
                        self.buf
                            .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = false;
                    }
                    self.buf.push_back(TypingEvent::Unknown);
                    true
                }
            }
        } else {
            // No more characters.
            if self.cur_layer != 0 {
                self.buf.push_back(TypingEvent::Release(
                    self.layout.layer_idx[self.cur_layer as usize],
                ));
                self.cur_layer = 0;
            }
            if self.cur_shifted {
                self.buf
                    .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                self.cur_shifted = false;
            }
            false
        }
    }
}

impl<'l, I> Iterator for Keys<'l, I>
where
    I: Iterator<Item = Win1252Char>,
{
    type Item = TypingEvent;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.buf.pop_front() {
            return Some(e);
        }

        self.handle_next();
        self.buf.pop_front()
    }

    /// At least one keypress will be yielded for each character in the input,
    /// either a `Tap` of the corresponding key or `Unknown`.
    /// More may be generated to switch layers etc.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, _) = self.chars.size_hint();
        (l + self.buf.len(), None)
    }
}
impl<'l, I> FusedIterator for Keys<'l, I> where I: FusedIterator + Iterator<Item = Win1252Char> {}

impl<'l, I> LookaheadIterator for Keys<'l, I>
where
    I: Iterator<Item = Win1252Char>,
{
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item> {
        self.extend_buf_to(n);
        self.buf.get(n)
    }

    fn remove_nth(&mut self, n: usize) -> Option<Self::Item> {
        self.extend_buf_to(n);
        self.buf.remove(n)
    }
}

pub trait LookaheadIterator: Iterator {
    /// Peek at the nth element of the iterator.
    ///
    /// This will _not_ consume any elements from the iterator,
    /// but may consume from an underlying one.
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item>;

    /// Pop the nth element of the iterator.
    ///
    /// This will consume _only_ the nth element, not any earlier ones.
    fn remove_nth(&mut self, n: usize) -> Option<Self::Item>;
}

#[derive(Clone)]
pub struct NPeekable<I: Iterator> {
    inner: I,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator> NPeekable<I> {
    fn extend_buf_to(&mut self, n: usize) {
        if n >= self.buf.len() {
            self.buf
                .extend(self.inner.by_ref().take(n + 1 - self.buf.len()));
        }
    }
}

impl<I: Iterator> Iterator for NPeekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let elem @ Some(_) = self.buf.pop_front() {
            return elem;
        }

        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.inner.size_hint();
        let buf_len = self.buf.len();
        (
            lower.saturating_add(buf_len),
            upper.map(|u| u.saturating_add(buf_len)),
        )
    }

    fn count(self) -> usize {
        self.buf.len() + self.inner.count()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.inner.last().or_else(|| self.buf.pop_back())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.buf.len() {
            self.buf.drain(..=n).next_back()
        } else {
            let out = self.inner.nth(n - self.buf.len());
            self.buf.clear();
            out
        }
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let accum = self.buf.into_iter().fold(init, &mut f);
        self.inner.fold(accum, f)
    }
}

impl<I: Iterator> LookaheadIterator for NPeekable<I> {
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item> {
        self.extend_buf_to(n);
        self.buf.get(n)
    }

    fn remove_nth(&mut self, n: usize) -> Option<Self::Item> {
        self.extend_buf_to(n);
        self.buf.remove(n)
    }
}

#[derive(Clone)]
pub struct Oneshot<I> {
    events: I,
}

impl<I> Iterator for Oneshot<I>
where
    I: LookaheadIterator + Iterator<Item = TypingEvent>,
{
    type Item = TypingEvent;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.events.next() {
                Some(TypingEvent::Hold(pos)) => {
                    let mut used = false;
                    for i in 0.. {
                        match self.events.peek_nth(i).copied() {
                            None => panic!("{pos} held but not released"),
                            Some(TypingEvent::Tap { .. } | TypingEvent::Unknown) => {
                                if used {
                                    // The hold is used for multiple keys, so should stay a hold.
                                    return Some(TypingEvent::Hold(pos));
                                }
                                used = true;
                            }
                            Some(TypingEvent::Release(pos2)) if pos == pos2 => {
                                match used {
                                    false => {
                                        // The hold is not actually doing anything, so remove it.
                                        // This will break if modifiers require other modifiers to be held,
                                        // for eg. if they are not on the home layer.
                                        self.events.remove_nth(i);
                                    }
                                    true => {
                                        // The hold is used for 1 tap, so should be oneshot.
                                        self.events.remove_nth(i);
                                        return Some(TypingEvent::Tap {
                                            pos,
                                            for_char: false,
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                e => {
                    return e;
                }
            }
        }
    }

    /// At most one event will be yielded for each event in the input.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, u) = self.events.size_hint();
        (0, u)
    }
}
impl<I> FusedIterator for Oneshot<I> where
    I: FusedIterator + LookaheadIterator + Iterator<Item = TypingEvent>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypingEvent {
    Tap { pos: u8, for_char: bool },
    Hold(u8),
    Release(u8),
    Unknown,
}

pub fn keys<I: IntoIterator<Item = Win1252Char>>(
    layout: &AnnotatedLayout,
    chars: I,
) -> Keys<'_, I::IntoIter> {
    Keys {
        layout,
        chars: chars.into_iter(),
        cur_layer: 0,
        cur_shifted: false,
        buf: VecDeque::new(),
    }
}

pub fn oneshot<I>(events: I) -> Oneshot<I::IntoIter>
where
    I: IntoIterator<Item = TypingEvent>,
    I::IntoIter: LookaheadIterator,
{
    Oneshot {
        events: events.into_iter(),
    }
}

pub fn lookahead<I>(iter: I) -> NPeekable<I::IntoIter>
where
    I: IntoIterator,
{
    NPeekable {
        inner: iter.into_iter(),
        buf: VecDeque::new(),
    }
}

pub fn string_cost(layout: &AnnotatedLayout, string: &[Win1252Char]) -> (u64, u64) {
    // let keys = keys(&layout.char_idx, string.chars());
    // let events = key_seq(layout.layer_idx, layout.shift_idx, keys);

    let events = oneshot(keys(layout, string.iter().copied()));

    cost_of_typing(events)
}

/// Interpret a String as a Vec of bytes encoded using Windows_1252, where each byte represents one char.
/// If any of the chars in the String are not encodable, returns None.
pub fn to_bytes(string: String) -> Option<Vec<Win1252Char>> {
    let (out, _, had_errors) = WINDOWS_1252.encode(&string);

    // SAFETY: Win1252_char is a repr(transparent) wrapper of u8
    // and the bytes came from WINDOWS_1252::encode, so they are valid.
    (!had_errors).then_some(unsafe { std::mem::transmute(out.to_vec()) })
}

// Assumes there is only one intended way of typing each character,
// and that all typable characters have a single-byte representation.
pub type CharIdx = EnumMap<Win1252Char, Option<CharIdxEntry>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharIdxEntry {
    pub layer: u8,
    pub pos: u8,
    pub shifted: bool,
}

#[derive(Debug, Clone)]
pub struct AnnotatedLayout {
    layout: Layout,
    char_idx: CharIdx,
    layer_idx: Vec<u8>,
    shift_idx: Option<u8>,
}

impl AnnotatedLayout {
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn char_idx(&self) -> &CharIdx {
        &self.char_idx
    }

    pub fn layer_idx(&self) -> &[u8] {
        &self.layer_idx
    }

    pub fn shift_idx(&self) -> &Option<u8> {
        &self.shift_idx
    }

    pub fn num_layers(&self) -> u8 {
        self.layout.layers.len() as u8
    }
}

impl From<Layout> for AnnotatedLayout {
    fn from(layout: Layout) -> Self {
        let mut char_idx: CharIdx = (0..)
            .zip(layout.iter())
            .flat_map(|(i, l)| {
                (0..).zip(l.iter()).filter_map(move |(j, k)| {
                    k.typed_char(true).map(|c| {
                        (
                            c,
                            Some(CharIdxEntry {
                                layer: i,
                                pos: j,
                                shifted: true,
                            }),
                        )
                    })
                })
            })
            .collect();
        char_idx.extend((0..).zip(layout.iter()).flat_map(|(i, l)| {
            (0..).zip(l.iter()).filter_map(move |(j, k)| {
                k.typed_char(false).map(|c| {
                    (
                        c,
                        Some(CharIdxEntry {
                            layer: i,
                            pos: j,
                            shifted: false,
                        }),
                    )
                })
            })
        }));

        let layer_idx = (0..)
            .zip(layout[0].iter())
            .filter_map(move |(j, k)| match k {
                Key::Layer(n) => Some((*n, j)),
                _ => None,
            })
            .fold(vec![0; layout.layers.len()], |mut a, (n, j)| {
                a[n] = j;
                a
            });
        let shift_idx = (0..)
            .zip(layout[0].iter())
            .find_map(|(i, k)| matches!(k, Key::Shift).then_some(i));
        Self {
            layout,
            char_idx,
            layer_idx,
            shift_idx,
        }
    }
}

impl From<AnnotatedLayout> for Layout {
    fn from(layout: AnnotatedLayout) -> Self {
        layout.layout
    }
}

fn cost(corpus: &[Vec<Win1252Char>], layout: &AnnotatedLayout) -> f64 {
    let (t, c) = corpus
        .iter()
        .map(|s| string_cost(layout, s))
        .fold((0, 0), |(a0, a1), (b0, b1)| (a0 + b0, a1 + b1));
    // let (t, c) = corpus
    //     .par_iter()
    //     .map(|s| string_cost(&char_idx, layer_idx, next_key_cost, held_key_cost, s))
    //     .reduce(|| (0, 0), |(a0, a1), (b0, b1)| (a0 + b0, a1 + b1));

    t as f64 / c as f64 + layout_cost(layout)
}

fn read_corpus_impl<P: AsRef<Path>>(corpus: &mut Vec<Vec<Win1252Char>>, path: P) -> io::Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in path.read_dir()? {
            read_corpus_impl(corpus, entry?.path())?;
        }
    } else {
        let string = fs::read_to_string(path)?;
        corpus.push(
            to_bytes(string).unwrap_or_else(|| panic!("unable to encode {}", path.display())),
        );
    }

    Ok(())
}

fn read_named_corpus_impl<P: AsRef<Path>>(
    corpus: &mut Vec<(PathBuf, Vec<Win1252Char>)>,
    path: P,
) -> io::Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in path.read_dir()? {
            read_named_corpus_impl(corpus, entry?.path())?;
        }
    } else {
        let string = fs::read_to_string(path)?;
        corpus.push((
            path.to_owned(),
            to_bytes(string).unwrap_or_else(|| panic!("unable to encode {}", path.display())),
        ));
    }

    Ok(())
}

pub fn read_corpus<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<Win1252Char>>> {
    let mut out = Vec::new();
    read_corpus_impl(&mut out, path)?;
    Ok(out)
}

pub fn read_named_corpus<P: AsRef<Path>>(path: P) -> io::Result<Vec<(PathBuf, Vec<Win1252Char>)>> {
    let mut out = Vec::new();
    read_named_corpus_impl(&mut out, path)?;
    Ok(out)
}

#[derive(Debug, Clone, Copy)]
enum Mutation {
    SwapKeys { l0: u8, l1: u8, i: u8, j: u8 },
    // SwapPaired {
    //     l0: u8,
    //     l1: u8,
    //     i: u8,
    //     j: u8,
    // },
}

impl Mutation {
    fn gen<R: Rng>(mut rng: R, layout: &AnnotatedLayout) -> Self {
        let layer = rng.gen_range(0..layout.num_layers());
        let i = rng.gen_range(0..NUM_KEYS);
        if matches!(layout.layout[layer][i], Key::Layer(_) | Key::Shift) {
            assert_eq!(layer, 0);
            // Keep layer switch keys on home layer
            // to ensure every layer can be accessed.
            let j = rng.gen_range(0..NUM_KEYS);
            Self::SwapKeys { l0: 0, l1: 0, i, j }
        } else {
            let layer2 = rng.gen_range(0..layout.num_layers());
            let j = loop {
                let j = rng.gen_range(0..NUM_KEYS);
                if !matches!(layout.layout[layer2][j], Key::Layer(_) | Key::Shift) {
                    break j;
                }
            };
            Self::SwapKeys {
                l0: layer,
                l1: layer2,
                i,
                j,
            }
        }
    }

    fn apply(self, layout: &mut AnnotatedLayout) {
        match self {
            Self::SwapKeys { l0, i, l1, j } => {
                let a = layout.layout[l0][i];
                let b = layout.layout[l1][j];

                if let Some(c) = a.typed_char(false) {
                    let entry = layout.char_idx[c].as_mut().unwrap();
                    assert!(!entry.shifted);
                    assert_eq!(entry.layer, l0);
                    assert_eq!(entry.pos, i);
                    entry.layer = l1;
                    entry.pos = j;
                }
                if let Some(c) = b.typed_char(false) {
                    let entry = layout.char_idx[c].as_mut().unwrap();
                    assert!(!entry.shifted);
                    assert_eq!(entry.layer, l1);
                    assert_eq!(entry.pos, j);
                    entry.layer = l0;
                    entry.pos = i;
                }
                if let Some(c) = a.typed_char(true) {
                    let entry = layout.char_idx[c].as_mut().unwrap();
                    if entry.shifted {
                        assert_eq!(entry.layer, l0);
                        assert_eq!(entry.pos, i);
                        entry.layer = l1;
                        entry.pos = j;
                    }
                }
                if let Some(c) = b.typed_char(true) {
                    let entry = layout.char_idx[c].as_mut().unwrap();
                    if entry.shifted {
                        assert_eq!(entry.layer, l1);
                        assert_eq!(entry.pos, j);
                        entry.layer = l0;
                        entry.pos = i;
                    }
                }
                if let Key::Layer(layer) = a {
                    assert_eq!(l0, 0);
                    assert_eq!(layout.layer_idx[layer], i);
                    layout.layer_idx[layer] = j;
                }
                if let Key::Layer(layer) = b {
                    assert_eq!(l1, 0);
                    assert_eq!(layout.layer_idx[layer], j);
                    layout.layer_idx[layer] = i;
                }
                if let Key::Shift = a {
                    assert_eq!(l0, 0);
                    assert_eq!(layout.shift_idx, Some(i));
                    layout.shift_idx = Some(j);
                }
                if let Key::Shift = b {
                    assert_eq!(l1, 0);
                    assert_eq!(layout.shift_idx, Some(j));
                    layout.shift_idx = Some(i);
                }

                if l0 == l1 {
                    layout.layout[l0].0.swap(i as usize, j as usize);
                } else {
                    let (layer_low, layer_high, pos_low, pos_high);
                    if l0 > l1 {
                        (layer_low, pos_low, layer_high, pos_high) = (l1, j, l0, i);
                    } else {
                        (layer_low, pos_low, layer_high, pos_high) = (l0, i, l1, j);
                    }
                    assert!(layer_low < layer_high);
                    // Split the layers so we can safely have mutable references
                    // to two parts of it.
                    let (left, right) = layout.layout.layers.split_at_mut(layer_low as usize + 1);
                    assert_eq!(left.len(), layer_low as usize + 1);
                    std::mem::swap(
                        &mut left.last_mut().unwrap()[pos_low],
                        &mut right[(layer_high - layer_low) as usize - 1][pos_high],
                    );
                }

                assert_eq!(b, layout.layout[l0][i]);
                assert_eq!(a, layout.layout[l1][j]);
            } // Self::SwapPaired { l0, l1, i, j } => {
              //     layout[l0].0.swap(i, j);
              //     layout[l1].0.swap(i, j);
              // }
        }
    }

    fn undo(self, layout: &mut AnnotatedLayout) {
        self.apply(layout)
    }
}

// const N: u32 = 50000;
// const T0: f64 = 20.;
const K: f64 = 10.;
const P0: f64 = 1.;

/// Optimise the layout using simulated annealing.
pub fn optimise(
    n: u32,
    layout: Layout,
    corpus: &[Vec<Win1252Char>],
    mut progress_callback: impl FnMut(u32),
) -> (Layout, f64) {
    let mut layout: AnnotatedLayout = layout.into();
    let mut rng = thread_rng();
    let initial_energy = cost(corpus, &layout);
    let t0 = initial_energy;
    let mut energy = initial_energy;
    for i in 0..n {
        progress_callback(i);
        let mutation = Mutation::gen(&mut rng, &layout);
        mutation.apply(&mut layout);
        let new_energy = cost(corpus, &layout);
        match new_energy.partial_cmp(&energy).unwrap() {
            cmp::Ordering::Less | cmp::Ordering::Equal => {}
            cmp::Ordering::Greater => {
                let temperature = t0 * (-K * i as f64 / n as f64).exp();
                let p = P0 * ((energy - new_energy) / temperature).exp();
                if !rng.gen_bool(p) {
                    mutation.undo(&mut layout);
                    continue;
                }
            }
        }
        energy = new_energy;
        // eprintln!("iteration {}, energy = {}", i, energy);
    }
    // eprintln!("improvement: {}", initial_energy - energy);
    (layout.layout, initial_energy - energy)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn keys_helloworld() {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let string = "Hello, WORLD! (~1)".to_owned();
        let expected = {
            use TypingEvent::*;
            vec![
                Hold(30),
                Tap {
                    pos: 15,
                    for_char: true,
                },
                Release(30),
                Tap {
                    pos: 2,
                    for_char: true,
                },
                Tap {
                    pos: 18,
                    for_char: true,
                },
                Tap {
                    pos: 18,
                    for_char: true,
                },
                Tap {
                    pos: 8,
                    for_char: true,
                },
                Tap {
                    pos: 27,
                    for_char: true,
                },
                Tap {
                    pos: 31,
                    for_char: true,
                },
                Hold(30),
                Tap {
                    pos: 1,
                    for_char: true,
                },
                Tap {
                    pos: 8,
                    for_char: true,
                },
                Tap {
                    pos: 3,
                    for_char: true,
                },
                Tap {
                    pos: 18,
                    for_char: true,
                },
                Tap {
                    pos: 12,
                    for_char: true,
                },
                Release(30),
                Hold(32),
                Tap {
                    pos: 15,
                    for_char: true,
                },
                Release(32),
                Tap {
                    pos: 31,
                    for_char: true,
                },
                Hold(32),
                Tap {
                    pos: 13,
                    for_char: true,
                },
                Release(32),
                Hold(30),
                Hold(32),
                Tap {
                    pos: 12,
                    for_char: true,
                },
                Release(32),
                Release(30),
                Hold(33),
                Tap {
                    pos: 1,
                    for_char: true,
                },
                Release(33),
                Hold(32),
                Tap {
                    pos: 16,
                    for_char: true,
                },
                Release(32),
            ]
        };

        let actual: Vec<_> = keys(&layout.into(), to_bytes(string).unwrap()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn oneshot_helloworld() {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let string = "Hello, WORLD! (~1)".to_owned();
        let expected = {
            use TypingEvent::*;
            vec![
                Tap {
                    pos: 30,
                    for_char: false,
                },
                Tap {
                    pos: 15,
                    for_char: true,
                },
                Tap {
                    pos: 2,
                    for_char: true,
                },
                Tap {
                    pos: 18,
                    for_char: true,
                },
                Tap {
                    pos: 18,
                    for_char: true,
                },
                Tap {
                    pos: 8,
                    for_char: true,
                },
                Tap {
                    pos: 27,
                    for_char: true,
                },
                Tap {
                    pos: 31,
                    for_char: true,
                },
                Hold(30),
                Tap {
                    pos: 1,
                    for_char: true,
                },
                Tap {
                    pos: 8,
                    for_char: true,
                },
                Tap {
                    pos: 3,
                    for_char: true,
                },
                Tap {
                    pos: 18,
                    for_char: true,
                },
                Tap {
                    pos: 12,
                    for_char: true,
                },
                Release(30),
                Tap {
                    pos: 32,
                    for_char: false,
                },
                Tap {
                    pos: 15,
                    for_char: true,
                },
                Tap {
                    pos: 31,
                    for_char: true,
                },
                Tap {
                    pos: 32,
                    for_char: false,
                },
                Tap {
                    pos: 13,
                    for_char: true,
                },
                Tap {
                    pos: 30,
                    for_char: false,
                },
                Tap {
                    pos: 32,
                    for_char: false,
                },
                Tap {
                    pos: 12,
                    for_char: true,
                },
                Tap {
                    pos: 33,
                    for_char: false,
                },
                Tap {
                    pos: 1,
                    for_char: true,
                },
                Tap {
                    pos: 32,
                    for_char: false,
                },
                Tap {
                    pos: 16,
                    for_char: true,
                },
            ]
        };

        let actual: Vec<_> = oneshot(keys(&layout.into(), to_bytes(string).unwrap())).collect();

        assert_eq!(expected, actual);
    }
}
