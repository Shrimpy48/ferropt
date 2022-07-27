use rand::{thread_rng, Rng};

use ahash::AHashMap;

use std::collections::VecDeque;
use std::iter::FusedIterator;
use std::path::Path;
use std::{cmp, fs, io};

use crate::simple_cost::{cost_of_typing, layout_cost};

use crate::layout::{Key, Layout, NUM_KEYS};

const CORPUS_DIR: &str = "corpus";

// TODO: use one-shot layers/shift where possible.
#[derive(Clone)]
struct Keys<'l, I> {
    layout: &'l AnnotatedLayout,
    chars: I,
    cur_layer: usize,
    cur_shifted: bool,
    buf: VecDeque<TypingEvent>,
}

impl<'l, I> Iterator for Keys<'l, I>
where
    I: Iterator<Item = char>,
{
    type Item = TypingEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.buf.pop_front() {
            return Some(e);
        }

        if let Some(c) = self.chars.next() {
            match self.layout.char_idx.get(&c).copied() {
                Some(CharIdxEntry {
                    layer,
                    pos,
                    shifted,
                }) => {
                    // A typable character.
                    if self.cur_layer != 0 && layer != self.cur_layer {
                        self.buf
                            .push_back(TypingEvent::Release(self.layout.layer_idx[self.cur_layer]));
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
                                self.layout.layer_idx[self.cur_layer],
                            ));
                            self.cur_layer = 0;
                        }
                        self.buf
                            .push_back(TypingEvent::Hold(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = true;
                    }
                    if layer != 0 && self.cur_layer != layer {
                        self.buf
                            .push_back(TypingEvent::Hold(self.layout.layer_idx[layer]));
                        self.cur_layer = layer;
                    }

                    self.buf.push_back(TypingEvent::Tap {
                        pos,
                        for_char: true,
                    });
                    self.buf.pop_front()
                }
                None => {
                    // An untypable character.
                    if self.cur_layer != 0 {
                        self.buf
                            .push_back(TypingEvent::Release(self.layout.layer_idx[self.cur_layer]));
                        self.cur_layer = 0;
                    }
                    if self.cur_shifted {
                        self.buf
                            .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = false;
                    }
                    self.buf.push_back(TypingEvent::Unknown);
                    self.buf.pop_front()
                }
            }
        } else {
            // No more characters.
            if self.cur_layer != 0 {
                self.buf
                    .push_back(TypingEvent::Release(self.layout.layer_idx[self.cur_layer]));
                self.cur_layer = 0;
            }
            if self.cur_shifted {
                self.buf
                    .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                self.cur_shifted = false;
            }
            self.buf.pop_front()
        }
    }

    /// At least one keypress will be yielded for each character in the input,
    /// either a `Tap` of the corresponding key or `Unknown`.
    /// More may be generated to switch layers etc.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, _) = self.chars.size_hint();
        (l, None)
    }
}
impl<'l, I> FusedIterator for Keys<'l, I> where I: FusedIterator + Iterator<Item = char> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypingEvent {
    Tap { pos: usize, for_char: bool },
    Hold(usize),
    Release(usize),
    Unknown,
}

// fn keys(char_idx: &CharIdx, chars: impl Iterator<Item = char>) -> Vec<(usize, bool, Vec<usize>)> {
//     let mut out = vec![(0, false, Vec::new())];
//     for pos in chars.map(|c| char_idx.get(&c).copied()) {
//         let (cur_layer, cur_shifted, cur_keys) = out.last_mut().unwrap();
//         match pos {
//             Some(CharIdxEntry {
//                 layer,
//                 pos,
//                 shifted,
//             }) => {
//                 if layer == *cur_layer && shifted == *cur_shifted {
//                     cur_keys.push(pos);
//                 } else {
//                     out.push((layer, shifted, vec![pos]));
//                 }
//             }
//             None if !cur_keys.is_empty() => {
//                 // Add an empty Vec to indicate the unknown character.
//                 out.push((0, false, Vec::new()));
//                 out.push((0, false, Vec::new()));
//             }
//             None => {}
//         }
//     }
//     if out.last().unwrap().2.is_empty() {
//         out.pop();
//     }
//     out
// }

fn keys<I: IntoIterator<Item = char>>(layout: &AnnotatedLayout, chars: I) -> Keys<'_, I::IntoIter> {
    Keys {
        layout,
        chars: chars.into_iter(),
        cur_layer: 0,
        cur_shifted: false,
        buf: VecDeque::new(),
    }
}

// Assumes the layer and shift keys are on the home layer.
// fn key_seq(
//     layer_idx: [usize; NUM_LAYERS],
//     shift_idx: Option<usize>,
//     key_groups: impl IntoIterator<Item = (usize, bool, Vec<usize>)>,
// ) -> impl Iterator<Item = TypingEvent> {
//     let mut out = Vec::new();
//     let mut cur_layer = 0;
//     let mut cur_shifted = false;
//     for (layer, shifted, keys) in key_groups {
//         if keys.is_empty() {
//             out.push(TypingEvent::Unknown);
//             if cur_layer != 0 {
//                 out.push(TypingEvent::Release(layer_idx[cur_layer]));
//                 cur_layer = 0;
//             }
//             if cur_shifted {
//                 out.push(TypingEvent::Release(shift_idx.unwrap()));
//                 cur_shifted = false;
//             }
//             continue;
//         }
//         if cur_layer != 0 && layer != cur_layer {
//             out.push(TypingEvent::Release(layer_idx[cur_layer]));
//             cur_layer = 0;
//         }
//         if cur_shifted && !shifted {
//             out.push(TypingEvent::Release(shift_idx.unwrap()));
//             cur_shifted = false;
//         }
//         if keys.len() == 1 {
//             if shifted && !cur_shifted {
//                 if cur_layer != 0 {
//                     out.push(TypingEvent::Release(layer_idx[cur_layer]));
//                     cur_layer = 0;
//                 }
//                 out.push(TypingEvent::Tap {
//                     pos: shift_idx.unwrap(),
//                     for_char: false,
//                 });
//                 cur_shifted = false;
//             }
//             if layer != 0 && layer != cur_layer {
//                 out.push(TypingEvent::Tap {
//                     pos: layer_idx[layer],
//                     for_char: false,
//                 });
//                 cur_layer = 0;
//             }
//             out.push(TypingEvent::Tap {
//                 pos: keys[0],
//                 for_char: true,
//             });
//         } else {
//             if shifted && !cur_shifted {
//                 if cur_layer != 0 {
//                     out.push(TypingEvent::Release(layer_idx[cur_layer]));
//                     cur_layer = 0;
//                 }
//                 out.push(TypingEvent::Hold(shift_idx.unwrap()));
//                 cur_shifted = true;
//             }
//             if layer != 0 && layer != cur_layer {
//                 out.push(TypingEvent::Hold(layer_idx[layer]));
//                 cur_layer = layer;
//             }
//             for key in keys {
//                 out.push(TypingEvent::Tap {
//                     pos: key,
//                     for_char: true,
//                 });
//             }
//         }
//     }
//     if cur_layer != 0 {
//         out.push(TypingEvent::Release(layer_idx[cur_layer]));
//     }
//     if cur_shifted {
//         out.push(TypingEvent::Release(shift_idx.unwrap()));
//     }
//     out.into_iter()
// }

pub fn string_cost(layout: &AnnotatedLayout, string: &str) -> (u64, u64) {
    // let keys = keys(&layout.char_idx, string.chars());
    // let events = key_seq(layout.layer_idx, layout.shift_idx, keys);

    let events = keys(layout, string.chars());

    cost_of_typing(events)
}

// Assumes there is only one intended way of typing each character.
pub type CharIdx = AHashMap<char, CharIdxEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharIdxEntry {
    pub layer: usize,
    pub pos: usize,
    pub shifted: bool,
}

#[derive(Debug, Clone)]
pub struct AnnotatedLayout {
    layout: Layout,
    char_idx: CharIdx,
    layer_idx: Vec<usize>,
    shift_idx: Option<usize>,
}

impl AnnotatedLayout {
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn char_idx(&self) -> &CharIdx {
        &self.char_idx
    }

    pub fn layer_idx(&self) -> &[usize] {
        &self.layer_idx
    }

    pub fn shift_idx(&self) -> &Option<usize> {
        &self.shift_idx
    }

    pub fn num_layers(&self) -> usize {
        self.layout.layers.len()
    }
}

impl From<Layout> for AnnotatedLayout {
    fn from(layout: Layout) -> Self {
        let mut char_idx: AHashMap<_, _> = layout
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.iter().enumerate().filter_map(move |(j, k)| {
                    k.typed_char(true).map(|c| {
                        (
                            c,
                            CharIdxEntry {
                                layer: i,
                                pos: j,
                                shifted: true,
                            },
                        )
                    })
                })
            })
            .collect();
        char_idx.extend(layout.iter().enumerate().flat_map(|(i, l)| {
            l.iter().enumerate().filter_map(move |(j, k)| {
                k.typed_char(false).map(|c| {
                    (
                        c,
                        CharIdxEntry {
                            layer: i,
                            pos: j,
                            shifted: false,
                        },
                    )
                })
            })
        }));

        let layer_idx = layout[0]
            .iter()
            .enumerate()
            .filter_map(move |(j, k)| match k {
                Key::Layer(n) => Some((*n, j)),
                _ => None,
            })
            .fold(vec![0; layout.layers.len()], |mut a, (n, j)| {
                a[n] = j;
                a
            });
        let shift_idx = layout[0]
            .iter()
            .enumerate()
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

fn cost(corpus: &[String], layout: &AnnotatedLayout) -> f64 {
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

fn read_corpus_impl<P: AsRef<Path>>(corpus: &mut Vec<String>, path: P) -> io::Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in path.read_dir()? {
            read_corpus_impl(corpus, entry?.path())?;
        }
    } else {
        corpus.push(fs::read_to_string(path)?);
    }

    Ok(())
}

pub fn read_corpus() -> io::Result<Vec<String>> {
    let mut out = Vec::new();
    read_corpus_impl(&mut out, CORPUS_DIR)?;
    Ok(out)
}

#[derive(Debug, Clone, Copy)]
enum Mutation {
    SwapKeys {
        l0: usize,
        l1: usize,
        i: usize,
        j: usize,
    },
    // SwapPaired {
    //     l0: usize,
    //     l1: usize,
    //     i: usize,
    //     j: usize,
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
            Self::SwapKeys {
                mut l0,
                mut i,
                mut l1,
                mut j,
            } => {
                let a = layout.layout[l0][i];
                let b = layout.layout[l1][j];

                if let Some(c) = a.typed_char(false) {
                    let entry = layout.char_idx.get_mut(&c).unwrap();
                    assert_eq!(entry.layer, l0);
                    assert_eq!(entry.pos, i);
                    assert!(!entry.shifted);
                    entry.layer = l1;
                    entry.pos = j;
                }
                if let Some(c) = b.typed_char(false) {
                    let entry = layout.char_idx.get_mut(&c).unwrap();
                    assert_eq!(entry.layer, l1);
                    assert_eq!(entry.pos, j);
                    assert!(!entry.shifted);
                    entry.layer = l0;
                    entry.pos = i;
                }
                if let Some(c) = a.typed_char(true) {
                    let entry = layout.char_idx.get_mut(&c).unwrap();
                    if entry.layer == l0 && entry.pos == i && entry.shifted {
                        entry.layer = l1;
                        entry.pos = j;
                    }
                }
                if let Some(c) = b.typed_char(true) {
                    let entry = layout.char_idx.get_mut(&c).unwrap();
                    if entry.layer == l1 && entry.pos == j && entry.shifted {
                        entry.layer = l0;
                        entry.pos = i;
                    }
                }
                if let Key::Layer(layer) = a {
                    assert_eq!(layout.layer_idx[layer], i);
                    layout.layer_idx[layer] = j;
                }
                if let Key::Layer(layer) = b {
                    assert_eq!(layout.layer_idx[layer], j);
                    layout.layer_idx[layer] = i;
                }
                if let Key::Shift = a {
                    assert_eq!(layout.shift_idx, Some(i));
                    layout.shift_idx = Some(j);
                }
                if let Key::Shift = b {
                    assert_eq!(layout.shift_idx, Some(j));
                    layout.shift_idx = Some(i);
                }

                if l0 == l1 {
                    layout.layout[l0].0.swap(i, j);
                    return;
                }
                if l0 > l1 {
                    (l0, i, l1, j) = (l1, j, l0, i);
                }
                // Split the layers so we can safely have mutable references
                // to two parts of it.
                let (left, right) = layout.layout.layers.split_at_mut(l0 + 1);
                std::mem::swap(&mut left.last_mut().unwrap()[i], &mut right[l1 - l0 - 1][j]);
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
    corpus: &[String],
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
        let string = "Hello, WORLD! (~1)";
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

        let actual: Vec<_> = keys(&layout.into(), string.chars()).collect();

        assert_eq!(expected, actual);
    }
}
