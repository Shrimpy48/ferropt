use rand::{thread_rng, Rng};
use rayon::prelude::*;

use ahash::AHashMap;

use std::io::prelude::*;

use std::path::Path;
use std::{cmp, fs, io};

use std::fs::{read_dir, File};

use crate::layout::{DEFAULT_LAYOUT, Key, KeyCost, Layer, Layout, NUM_KEYS, NUM_LAYERS, NextKeyCost};

const CORPUS_DIR: &str = "corpus";

#[rustfmt::skip]
const KEY_COST: KeyCost = Layer([
    30, 24, 20, 22, 32,   32, 22, 20, 24, 30,
    16, 13, 11, 10, 29,   29, 10, 11, 13, 16,
    32, 26, 23, 16, 30,   30, 16, 23, 26, 32,
                16, 11,   11, 16,
]);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Finger {
    Pinky,
    Ring,
    Middle,
    Index,
    Thumb,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Hand {
    Left,
    Right,
}

fn finger_for_pos(row: usize, col: usize) -> (Hand, Finger) {
    if row == 3 {
        match col {
            0 | 1 => (Hand::Left, Finger::Thumb),
            2 | 3 => (Hand::Right, Finger::Thumb),
            _ => panic!("invalid column {} for row {}", col, row),
        }
    } else {
        match col {
            0 => (Hand::Left, Finger::Pinky),
            1 => (Hand::Left, Finger::Ring),
            2 => (Hand::Left, Finger::Middle),
            3 | 4 => (Hand::Left, Finger::Index),
            5 | 6 => (Hand::Right, Finger::Index),
            7 => (Hand::Right, Finger::Middle),
            8 => (Hand::Right, Finger::Ring),
            9 => (Hand::Right, Finger::Pinky),
            _ => panic!("invalid column {} for row {}", col, row),
        }
    }
}

fn vert_penalty(f: Finger) -> u8 {
    match f {
        Finger::Middle => 2,
        Finger::Index => 3,
        Finger::Ring => 5,
        Finger::Pinky => 7,
        Finger::Thumb => 10,
    }
}

const OUTWARD_PENALTY: u8 = 1;

fn next_key_cost(i: usize, j: usize) -> u8 {
    // NEXT_KEY_COST[i][j]
    let r0 = i / 10;
    let c0 = i % 10;
    let r1 = j / 10;
    let c1 = j % 10;
    let row_dist = if r0 <= r1 { r1 - r0 } else { r0 - r1 };
    let (h0, f0) = finger_for_pos(r0, c0);
    let (h1, f1) = finger_for_pos(r1, c1);
    if (h0, f0) == (h1, f1) {
        // Same finger.
        let col_dist = if c0 <= c1 { c1 - c0 } else { c0 - c1 };
        let horiz_penalty = match f0 {
            Finger::Middle => 6,
            Finger::Index => 5,
            Finger::Ring => 8,
            Finger::Pinky => 12,
            Finger::Thumb => 3,
        };
        let strength_penalty = match f0 {
            Finger::Index => 2,
            Finger::Middle => 2,
            Finger::Ring => 4,
            Finger::Pinky => 6,
            Finger::Thumb => 3,
        };
        let sq_dist =
            vert_penalty(f0) as usize * row_dist * row_dist + horiz_penalty * col_dist * col_dist;
        if sq_dist == 0 {
            strength_penalty
        } else {
            // Inefficient, but the integer log isn't on stable yet.
            strength_penalty + (sq_dist as f64).log2() as u8
        }
    } else if h0 == h1 {
        // Same hand, different finger.
        if f0 == Finger::Thumb {
            // Thumb to finger.
            return match (c0, c1, r1) {
                (1 | 2, 4 | 5, 0 | 1) => 2,
                (1 | 2, 4 | 5, 2) => 3,
                (1 | 2, 3 | 6, 2) => 2,
                (0 | 3, 4 | 5, 0 | 1) => 3,
                (0 | 3, 4 | 5, 2) => 5,
                (0 | 3, 3 | 6, 2) => 2,
                _ => OUTWARD_PENALTY,
            };
        } else if f1 == Finger::Thumb {
            // Finger to thumb.
            return match (c1, c0, r0) {
                (1 | 2, 4 | 5, 0 | 1) => 2,
                (1 | 2, 4 | 5, 2) => 3,
                (1 | 2, 3 | 6, 2) => 2,
                (0 | 3, 4 | 5, 0 | 1) => 3,
                (0 | 3, 4 | 5, 2) => 5,
                (0 | 3, 3 | 6, 2) => 2,
                _ => 0,
            };
        };
        // Finger to finger.
        let outward = match h0 {
            Hand::Left => c1 < c0,
            Hand::Right => c1 > c0,
        };
        let stretch = c0 == 4 || c0 == 5 || c1 == 4 || c1 == 5;
        let dist = ((row_dist * vert_penalty(f1) as usize) as f64).log2() as u8;
        (if outward { OUTWARD_PENALTY } else { 0 }) + (if stretch { 2 } else { 0 }) + dist
    } else {
        // Different hand.
        2
    }
}

fn hold_key_cost(i: usize, j: usize) -> u8 {
    // Approximate by next key cost.
    2 + next_key_cost(i, j)
}

struct TypingModel<'nkc> {
    char_idx: AHashMap<char, (usize, usize)>,
    next_key_cost: &'nkc NextKeyCost,
    // Assumes the layer keys are on the home layer.
    layer_idx: [usize; NUM_LAYERS],
    total_cost: u64,
    count: u64,
    state: Option<(usize, usize)>,
}

impl<'nkc> TypingModel<'nkc> {
    fn new(next_key_cost: &'nkc NextKeyCost, layout: &Layout) -> Self {
        Self {
            char_idx: layout
                .iter()
                .enumerate()
                .flat_map(|(i, l)| {
                    l.iter().enumerate().filter_map(move |(j, k)| match k {
                        Key::Char(c) => Some((*c, (i, j))),
                        _ => None,
                    })
                })
                .collect(),
            next_key_cost,
            layer_idx: layout
                .iter()
                .flat_map(|l| {
                    l.iter().enumerate().filter_map(move |(j, k)| match k {
                        Key::Layer(n) => Some((*n, j)),
                        _ => None,
                    })
                })
                .fold([0; 3], |mut a, (n, j)| {
                    a[n] = j;
                    a
                }),
            total_cost: 0,
            count: 0,
            state: None,
        }
    }

    fn type_char(&mut self, c: char) {
        let idx = self.char_idx.get(&c).copied();
        match (self.state, idx) {
            (None, Some((j0, j1))) => {
                if j0 != 0 {
                    // Switch to the correct layer.
                    let l = self.layer_idx[j0];
                    self.total_cost += KEY_COST[l] as u64;
                    self.total_cost += KEY_COST[j1] as u64 + self.next_key_cost[l][j1] as u64;
                } else {
                    self.total_cost += KEY_COST[j1] as u64;
                }
                self.count += 1;
            }
            (Some((i0, i1)), Some((j0, j1))) => {
                if j0 != 0 {
                    let l = self.layer_idx[j0];
                    if j0 != i0 {
                        // Switch to the correct layer.
                        self.total_cost += KEY_COST[l] as u64 + self.next_key_cost[i1][l] as u64;
                        self.total_cost += KEY_COST[j1] as u64 + self.next_key_cost[l][j1] as u64;
                    } else {
                        // Apply a penalty for holding the layer key.
                        self.total_cost += hold_key_cost(l, j1) as u64;
                        self.total_cost += KEY_COST[j1] as u64 + self.next_key_cost[i1][j1] as u64;
                    }
                } else {
                    self.total_cost += KEY_COST[j1] as u64 + self.next_key_cost[i1][j1] as u64;
                }
                self.count += 1;
            }
            _ => {}
        }
        self.state = idx;
    }
}

pub fn file_costs<R: Read>(
    next_key_cost: &NextKeyCost,
    layouts: &[Layout],
    mut reader: R,
) -> io::Result<Vec<u64>> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;

    let mut models: Vec<_> = layouts
        .iter()
        .map(|l| TypingModel::new(next_key_cost, l))
        .collect();

    for c in string.chars() {
        for m in models.iter_mut() {
            m.type_char(c);
        }
    }

    Ok(models.into_iter().map(|m| m.total_cost).collect())
}

fn costs(next_key_cost: &NextKeyCost, layouts: &[Layout]) -> io::Result<Vec<u64>> {
    read_dir(CORPUS_DIR)?
        .map(|entry| {
            let path = entry?.path();
            let handle = File::open(path)?;
            file_costs(next_key_cost, layouts, handle)
        })
        .try_fold(vec![0; layouts.len()], |acc, costs| {
            Ok(acc
                .into_iter()
                .zip(costs?.into_iter())
                .map(|(x, y)| x + y)
                .collect())
        })
}

pub fn file_cost<R: Read>(
    next_key_cost: &NextKeyCost,
    layout: &Layout,
    mut reader: R,
) -> io::Result<(u64, u64)> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;

    let mut model = TypingModel::new(next_key_cost, layout);

    for c in string.chars() {
        model.type_char(c);
    }

    Ok((model.total_cost, model.count))
}

pub fn string_cost(next_key_cost: &NextKeyCost, layout: &Layout, string: &str) -> (u64, u64) {
    let mut model = TypingModel::new(next_key_cost, layout);

    for c in string.chars() {
        model.type_char(c);
    }

    (model.total_cost, model.count)
}

fn cost(next_key_cost: &NextKeyCost, corpus: &[String], layout: &Layout) -> f64 {
    let (t, c) = corpus
        .par_iter()
        .map(|s| string_cost(next_key_cost, layout, s))
        .reduce(|| (0, 0), |(a0, a1), (b0, b1)| (a0 + b0, a1 + b1));

    t as f64 / c as f64 + layout.hamming_dist(&DEFAULT_LAYOUT) as f64 / (NUM_KEYS * NUM_LAYERS) as f64 * 0.5
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

fn read_corpus() -> io::Result<Vec<String>> {
    // read_dir(CORPUS_DIR)?
    //     .map(|entry| fs::read_to_string(entry?.path()))
    //     .collect::<io::Result<Vec<_>>>()
    let mut out = Vec::new();
    read_corpus_impl(&mut out, CORPUS_DIR)?;
    Ok(out)
}

#[derive(Debug, Clone, Copy)]
enum Mutation {
    SwapKeys {
        i0: usize,
        i1: usize,
        j0: usize,
        j1: usize,
    },
    SwapPaired {
        l0: usize,
        l1: usize,
        i: usize,
        j: usize,
    },
}

impl Mutation {
    fn gen<R: Rng>(mut rng: R, layout: &Layout) -> Self {
        let mut i0 = rng.gen_range(0..NUM_LAYERS);
        let mut i1 = rng.gen_range(0..NUM_KEYS);
        if i1 >= 30 {
            // Move thumb keys around.
            let j1 = 30 + rng.gen_range(0..4);
            return Self::SwapKeys {
                i0: 0,
                i1,
                j0: 0,
                j1,
            };
        }
        if let Key::Char(c) = layout[i0][i1] {
            if c.is_ascii_alphabetic() {
                // Move shifted and unshifted version of the key together,
                // Keeping them on the same layers.
                let j = rng.gen_range(0..30);
                // Assume the alphabetic keys are on layers 0 and 1.
                debug_assert!([0, 1].contains(&i0));
                return Self::SwapPaired {
                    l0: 0,
                    l1: 1,
                    i: i1,
                    j,
                };
            }
        }
        let mut j0;
        let mut j1;
        // let j1 = rng.gen_range(0..layout[j0].0.len());
        // Avoid alphabetic characters when performing single key swaps,
        // to keep them on their layers and in sync with the shifted versions.
        loop {
            j0 = rng.gen_range(0..NUM_LAYERS);
            j1 = rng.gen_range(0..NUM_KEYS - 4);
            if let Key::Char(c) = layout[j0][j1] {
                if c.is_ascii_alphabetic() {
                    continue;
                }
            }
            break;
        }
        // Ensure i0 <= j0 for ease of application.
        if i0 > j0 {
            (i0, j0) = (j0, i0);
            // Necessary to preserve the alphabetic key checks.
            (i1, j1) = (j1, i1);
        }
        Self::SwapKeys { i0, i1, j0, j1 }
    }

    fn apply(self, layout: &mut Layout) {
        match self {
            Self::SwapKeys { i0, i1, j0, j1 } => {
                debug_assert!(if let Key::Char(c) = layout[i0][i1] {
                    !c.is_ascii_alphabetic()
                } else {
                    true
                });
                debug_assert!(if let Key::Char(c) = layout[j0][j1] {
                    !c.is_ascii_alphabetic()
                } else {
                    true
                });
                if i0 == j0 {
                    layout[i0].0.swap(i1, j1);
                    return;
                }
                // Split the layers so we can safely have mutable references
                // to two parts of it.
                let (left, right) = layout.layers.split_at_mut(i0 + 1);
                std::mem::swap(
                    &mut left.last_mut().unwrap()[i1],
                    &mut right[j0 - i0 - 1][j1],
                );
            }
            Self::SwapPaired { l0, l1, i, j } => {
                layout[l0].0.swap(i, j);
                layout[l1].0.swap(i, j);
            }
        }
    }

    fn undo(self, layout: &mut Layout) {
        self.apply(layout)
    }
}

const N: u32 = 20000;
const T0: f64 = 30.;
const K: f64 = 10.;
const P0: f64 = 1.;

/// Optimise the layout using simulated annealing.
pub fn optimise(mut layout: Layout) -> io::Result<Layout> {
    assert!(layout.is_satisfactory());

    let corpus = read_corpus()?;

    let next_key_cost = Layer(
        (0..NUM_KEYS)
            .map(|i| {
                Layer(
                    (0..NUM_KEYS)
                        .map(|j| next_key_cost(i, j))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );

    let mut rng = thread_rng();
    let initial_energy = cost(&next_key_cost, &corpus, &layout);
    let mut energy = initial_energy;
    eprintln!("energy = {}", energy);
    for i in 0..N {
        let mutation = Mutation::gen(&mut rng, &layout);
        mutation.apply(&mut layout);
        // The mutation shouldn't change this invariant.
        debug_assert!(layout.is_satisfactory());
        let new_energy = cost(&next_key_cost, &corpus, &layout);
        match new_energy.partial_cmp(&energy).unwrap() {
            cmp::Ordering::Less => {}
            cmp::Ordering::Equal => {}
            cmp::Ordering::Greater => {
                let temperature = T0 * (-K * i as f64 / N as f64).exp();
                let p = P0 * ((energy - new_energy) / temperature).exp();
                if !rng.gen_bool(p) {
                    mutation.undo(&mut layout);
                    continue;
                }
            }
        }
        energy = new_energy;
        eprintln!("iteration {}, energy = {}", i, energy);
    }
    eprintln!("improvement: {}", initial_energy - energy);
    Ok(layout)
}
