use lazy_static::lazy_static;

use crate::{
    evolve::{CharIdx, TypingEvent},
    layout::{finger_for_pos, Digit, Finger, Hand, Key, Layer, Layout, NUM_KEYS, NUM_LAYERS},
};

#[rustfmt::skip]
const KEY_COST: KeyCost = Layer([
    30, 24, 20, 22, 32,   32, 22, 20, 24, 30,
    16, 13, 11, 10, 29,   29, 10, 11, 13, 16,
    32, 26, 23, 16, 30,   30, 16, 23, 26, 32,
                16, 11,   11, 16,
]);

lazy_static! {
    static ref NEXT_KEY_COST: NextKeyCost = Layer(
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
    static ref HELD_KEY_COST: NextKeyCost = Layer(
        (0..NUM_KEYS)
            .map(|i| {
                Layer(
                    (0..NUM_KEYS)
                        .map(|j| held_key_cost(i, j))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );
}

pub type KeyCost = Layer<u8>;
pub type NextKeyCost = Layer<Layer<u8>>;

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
    let r0 = i / 10;
    let c0 = i % 10;
    let r1 = j / 10;
    let c1 = j % 10;
    let row_dist = if r0 <= r1 { r1 - r0 } else { r0 - r1 };
    let Digit {
        hand: h0,
        finger: f0,
    } = finger_for_pos(r0, c0);
    let Digit {
        hand: h1,
        finger: f1,
    } = finger_for_pos(r1, c1);
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
            Finger::Index => 6,
            Finger::Middle => 6,
            Finger::Ring => 12,
            Finger::Pinky => 18,
            Finger::Thumb => 9,
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

fn held_key_cost(i: usize, j: usize) -> u8 {
    let r0 = i / 10;
    let c0 = i % 10;
    let r1 = j / 10;
    let c1 = j % 10;
    let row_dist = if r0 <= r1 { r1 - r0 } else { r0 - r1 };
    let Digit {
        hand: h0,
        finger: f0,
    } = finger_for_pos(r0, c0);
    let Digit {
        hand: h1,
        finger: f1,
    } = finger_for_pos(r1, c1);
    let strength_penalty = match f0 {
        Finger::Index => 6,
        Finger::Middle => 6,
        Finger::Ring => 8,
        Finger::Pinky => 10,
        Finger::Thumb => 6,
    };
    if (h0, f0) == (h1, f1) {
        // Same finger.
        100
    } else if h0 == h1 {
        // Same hand, different finger.
        if f0 == Finger::Thumb {
            // Thumb to finger.
            return strength_penalty
                + match (c0, c1, r1) {
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
            return strength_penalty
                + match (c1, c0, r0) {
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
        (if outward { OUTWARD_PENALTY } else { 0 })
            + (if stretch { 2 } else { 0 })
            + dist
            + strength_penalty
    } else {
        // Different hand.
        strength_penalty
    }
}

pub fn cost_of_typing(keys: impl Iterator<Item = TypingEvent>) -> (u64, u64) {
    let mut held_keys = Vec::new();
    let mut prev_key = None;
    let mut total_cost = 0;
    let mut count = 0;
    for event in keys {
        match event {
            TypingEvent::Tap { pos, for_char } => {
                total_cost += KEY_COST[pos] as u64;
                for held in held_keys.iter() {
                    total_cost += HELD_KEY_COST[*held][pos] as u64;
                }
                if let Some(prev) = prev_key {
                    total_cost += NEXT_KEY_COST[prev][pos] as u64;
                }
                if for_char {
                    count += 1;
                }
                prev_key = Some(pos);
            }
            TypingEvent::Hold(pos) => {
                held_keys.push(pos);
                prev_key = None;
            }
            TypingEvent::Release(pos) => {
                let idx = held_keys
                    .iter()
                    .enumerate()
                    .find_map(|(i, k)| (*k == pos).then_some(i))
                    .unwrap();
                held_keys.swap_remove(idx);
            }
            TypingEvent::Unknown => {
                prev_key = None;
            }
        }
    }
    (total_cost, count)
}

fn similarity_cost(layout: &Layout, _char_idx: &CharIdx) -> f64 {
    0.
    // layout.hamming_dist(&DEFAULT_LAYOUT) as f64 / (NUM_KEYS * NUM_LAYERS) as f64 * 0.5
}

fn memorability_cost(_layout: &Layout, char_idx: &CharIdx) -> f64 {
    0.
}
// fn memorability_cost(char_idx: &AHashMap<char, (usize, usize)>) -> f64 {
//     let ordered_pairs = [['(', ')'], ['{', '}'], ['[', ']'], ['<', '>']];
//     let ordered_pair_penalty: f64 = ordered_pairs
//         .into_iter()
//         .map(|[l, r]| {
//             let (i0, i1) = char_idx[&l];
//             let (j0, j1) = char_idx[&r];
//             if i0 != j0 {
//                 if i1 != j1 {
//                     // Different key and layer.
//                     6.
//                 } else {
//                     // Same key, different layer.
//                     2.
//                 }
//             } else {
//                 // Same layer.
//                 let ri = i1 / 10;
//                 let ci = i1 % 10;
//                 let rj = j1 / 10;
//                 let cj = j1 % 10;
//                 if ri == rj {
//                     // Same row.
//                     if (ri == 3 && ci < 2 && cj == 3 - ci) || (ci < 5 && cj == 9 - ci) {
//                         // Mirrored between sides.
//                         0.
//                     } else if cj == ci + 1 {
//                         // Next to each other.
//                         0.
//                     } else if ci < cj && (cj < 5 || 5 <= ci) {
//                         // In order on the same side.
//                         1.
//                     } else {
//                         // Somewhere else in the same row.
//                         4.
//                     }
//                 } else if ci == cj {
//                     // Same column.
//                     1.
//                 } else {
//                     // Different row and column.
//                     4.
//                 }
//             }
//         })
//         .sum();
//     let similar_pairs = [
//         ['+', '-'],
//         ['\'', '"'],
//         ['*', '/'],
//         ['*', '&'],
//         ['\\', '/'],
//         ['!', '?'],
//         ['.', ','],
//         ['$', '£'],
//         ['-', '_'],
//         ['-', '~'],
//         ['/', '%'],
//         ['\'', '`'],
//         [';', ':'],
//         ['+', '*'],
//     ];
//     let similar_pair_penalty: f64 = similar_pairs
//         .into_iter()
//         .map(|[i, j]| {
//             let (i0, i1) = char_idx[&i];
//             let (j0, j1) = char_idx[&j];
//             if i0 != j0 {
//                 if i1 != j1 {
//                     // Different key and layer.
//                     4.
//                 } else {
//                     // Same key, different layer.
//                     1.
//                 }
//             } else {
//                 // Same layer.
//                 let ri = i1 / 10;
//                 let ci = i1 % 10;
//                 let rj = j1 / 10;
//                 let cj = j1 % 10;
//                 if ri == rj {
//                     // Same row.
//                     if (ri == 3 && cj == 3 - ci) || (cj == 9 - ci) {
//                         // Mirrored between sides.
//                         0.
//                     } else if cj == ci + 1 || ci == cj + 1 {
//                         // Next to each other.
//                         0.
//                     } else {
//                         // Somewhere else in the same row.
//                         2.
//                     }
//                 } else if ci == cj {
//                     // Same column.
//                     0.
//                 } else {
//                     // Different row and column.
//                     2.
//                 }
//             }
//         })
//         .sum();
//     0.01 * ordered_pair_penalty + 0.002 * similar_pair_penalty
// }

pub fn layout_cost(layout: &Layout, char_idx: &CharIdx) -> f64 {
    similarity_cost(layout, char_idx) + memorability_cost(layout, char_idx)
}
