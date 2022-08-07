use lazy_static::lazy_static;

use crate::layout::{
    finger_for_pos, AnnotatedLayout, CharIdx, Finger, Hand, Layer, TypingEvent, Win1252Char,
    LOWER_ALPHA, NUM_KEYS, UPPER_ALPHA,
};

use super::{log_norm, CostModel};

pub struct Model;

impl Default for Model {
    fn default() -> Self {
        Self
    }
}

impl CostModel for Model {
    fn cost_of_typing(&self, keys: impl Iterator<Item = TypingEvent>) -> (u64, u64) {
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
    fn layout_cost(&self, layout: &AnnotatedLayout) -> f64 {
        similarity_cost(layout) + memorability_cost(layout)
    }
}

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

fn next_key_cost(i: u8, j: u8) -> u8 {
    let r0 = i / 10;
    let c0 = i % 10;
    let r1 = j / 10;
    let c1 = j % 10;
    let row_dist = if r0 <= r1 { r1 - r0 } else { r0 - r1 };
    let d0 = finger_for_pos(r0, c0);
    let h0 = d0.hand();
    let f0 = d0.finger();
    let d1 = finger_for_pos(r1, c1);
    let h1 = d1.hand();
    let f1 = d1.finger();
    if d0 == d1 {
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
            Finger::Middle => 7,
            Finger::Ring => 12,
            Finger::Pinky => 18,
            Finger::Thumb => 10,
        };
        let sq_dist = vert_penalty(f0) * row_dist * row_dist + horiz_penalty * col_dist * col_dist;
        if sq_dist == 0 {
            strength_penalty
        } else {
            strength_penalty + log_norm(sq_dist)
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
        let dist = log_norm(row_dist * vert_penalty(f1));
        (if outward { OUTWARD_PENALTY } else { 0 }) + (if stretch { 2 } else { 0 }) + dist
    } else {
        // Different hand.
        2
    }
}

fn held_key_cost(i: u8, j: u8) -> u8 {
    let r0 = i / 10;
    let c0 = i % 10;
    let r1 = j / 10;
    let c1 = j % 10;
    let row_dist = if r0 <= r1 { r1 - r0 } else { r0 - r1 };
    let d0 = finger_for_pos(r0, c0);
    let h0 = d0.hand();
    let f0 = d0.finger();
    let d1 = finger_for_pos(r1, c1);
    let h1 = d1.hand();
    let f1 = d1.finger();
    let strength_penalty = match f0 {
        Finger::Index => 6,
        Finger::Middle => 6,
        Finger::Ring => 8,
        Finger::Pinky => 10,
        Finger::Thumb => 6,
    };
    if d0 == d1 {
        // Same finger.
        u8::MAX
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
        let dist = log_norm(row_dist * vert_penalty(f1));
        (if outward { OUTWARD_PENALTY } else { 0 })
            + (if stretch { 2 } else { 0 })
            + dist
            + strength_penalty
    } else {
        // Different hand.
        strength_penalty
    }
}

fn similarity_cost(_layout: &AnnotatedLayout) -> f64 {
    0.
    // layout.hamming_dist(&DEFAULT_LAYOUT) as f64 / (NUM_KEYS * NUM_LAYERS) as f64 * 0.5
}

lazy_static! {
    static ref ORDERED_PAIRS: [[Win1252Char; 2]; 4] =
        [["(", ")"], ["{", "}"], ["[", "]"], ["<", ">"],].map(|p| p.map(|s| s.try_into().unwrap()));
    static ref SIMILAR_PAIRS: [[Win1252Char; 2]; 17] = [
        ["+", "-"],
        ["*", "/"],
        ["+", "*"],
        ["-", "/"],
        ["/", "%"],
        ["\\", "/"],
        ["\\", "|"],
        ["/", "|"],
        ["\"", "'"],
        ["*", "&"],
        ["!", "?"],
        [".", ","],
        ["$", "£"],
        ["-", "_"],
        ["-", "~"],
        ["'", "`"],
        [";", ":"],
    ]
    .map(|p| p.map(|s| s.try_into().unwrap()));
    static ref MATHS_SYMBOLS: [Win1252Char; 13] =
        ["+", "-", "*", "/", "%", "=", "!", "@", "<", ">", "^", "&", "|",]
            .map(|s| s.try_into().unwrap());
    static ref BRACKETS: [Win1252Char; 8] =
        ["(", ")", "{", "}", "[", "]", "<", ">"].map(|s| s.try_into().unwrap());
    static ref QUOTES: [Win1252Char; 3] = ["'", "\"", "`"].map(|s| s.try_into().unwrap());
    static ref PUNCTUATION: [Win1252Char; 9] =
        [",", ".", ";", ":", "!", "?", "\"", "'", "-"].map(|s| s.try_into().unwrap());
    static ref LINES: [Win1252Char; 6] =
        ["-", "_", "\\", "|", "/", "~"].map(|s| s.try_into().unwrap());
}

// fn memorability_cost(_layout: &Layout, char_idx: &CharIdx) -> f64 {
//     0.
// }
fn memorability_cost(layout: &AnnotatedLayout) -> f64 {
    let ordered_pair_penalty: f64 = ORDERED_PAIRS
        .into_iter()
        .filter_map(|[l, r]| {
            let l = layout.char_idx()[l]?;
            let r = layout.char_idx()[r]?;
            Some(if l.layer != r.layer || l.shifted != r.shifted {
                if l.pos != r.pos {
                    // Different key and layer.
                    6.
                } else {
                    // Same key, different layer or shiftedness.
                    2.
                }
            } else {
                // Same layer and shiftedness.
                let l_row = l.pos / 10;
                let l_col = l.pos % 10;
                let r_row = r.pos / 10;
                let r_col = r.pos % 10;
                if l_row == r_row {
                    // Same row.
                    if (l_row == 3 && l_col < 2 && r_col == 3 - l_col)
                        || (l_col < 5 && r_col == 9 - l_col)
                        || (r_col == l_col + 1)
                    {
                        // Mirrored between sides or next to each other.
                        0.
                    } else if l_col < r_col && (r_col < 5 || 5 <= l_col) {
                        // In order on the same side.
                        1.
                    } else {
                        // Somewhere else in the same row.
                        4.
                    }
                } else if l_col == r_col {
                    // Same column.
                    1.
                } else {
                    // Different row and column.
                    4.
                }
            })
        })
        .sum();
    let similar_pair_penalty: f64 = SIMILAR_PAIRS
        .into_iter()
        .filter_map(|[a, b]| {
            let a = layout.char_idx()[a]?;
            let b = layout.char_idx()[b]?;
            Some(if a.layer != b.layer || a.shifted != b.shifted {
                if a.pos != b.pos {
                    // Different key and layer.
                    4.
                } else {
                    // Same key, different layer or shiftedness.
                    1.
                }
            } else {
                // Same layer and shiftedness.
                let a_row = a.pos / 10;
                let a_col = a.pos % 10;
                let b_row = b.pos / 10;
                let b_col = b.pos % 10;
                if a_row == b_row {
                    // Same row.
                    if (a_row == 3 && b_col == 3 - a_col)
                        || (b_col == 9 - a_col)
                        || b_col == a_col + 1
                        || a_col == b_col + 1
                    {
                        // Mirrored between sides or next to each other.
                        0.
                    } else {
                        // Somewhere else in the same row.
                        2.
                    }
                } else if a_col == b_col {
                    // Same column.
                    0.
                } else {
                    // Different row and column.
                    2.
                }
            })
        })
        .sum();
    // let space_penalty = match layout.char_idx()[*SPACE].unwrap().pos {
    //     31 => 0.,
    //     32 => 1.,
    //     _ => 3.,
    // };
    let space_penalty = 0.;
    let shift_penalty = match layout.shift_idx() {
        None | Some(30 | 31 | 32 | 33) => 0.,
        _ => 2.,
    };
    let mut layer_penalty = 0.;
    for l in layout.layer_idx().iter().skip(1) {
        layer_penalty += match l {
            30 | 31 | 32 | 33 => 0.,
            _ => 2.,
        }
    }
    0.01 * ordered_pair_penalty
        + 0.002 * similar_pair_penalty
        + 0.01 * layer_variation(layout.char_idx(), *LOWER_ALPHA)
        + 0.01 * layer_variation(layout.char_idx(), *UPPER_ALPHA)
        // + 0.01 * layer_variation(layout.char_idx(), *NUMBERS)
        + 0.002 * layer_variation(layout.char_idx(), *MATHS_SYMBOLS)
        + 0.002 * layer_variation(layout.char_idx(), *BRACKETS)
        + 0.002 * layer_variation(layout.char_idx(), *QUOTES)
        + 0.002 * layer_variation(layout.char_idx(), *PUNCTUATION)
        + 0.002 * layer_variation(layout.char_idx(), *LINES)
        + 0.1 * space_penalty
        + 0.1 * shift_penalty
        + 0.1 * layer_penalty
    // + num_layout_penalty(layout)
}

fn relative_pos(r: usize, mut c: usize) -> (usize, usize) {
    if r == 3 {
        c += 3;
    }
    if c >= 5 {
        c += 1;
    }
    (r, c)
}

fn hamming_dist<T: PartialEq>(xs: &[T], ys: &[T]) -> usize {
    xs.iter().zip(ys).filter(|(x, y)| x != y).count()
}

// fn num_layout_penalty(layout: &AnnotatedLayout) -> f64 {
//     if layer_variation(layout.char_idx(), *NUMBERS) != 0. {
//         return layer_variation(layout.char_idx(), *NUMBERS) * 10.;
//     }
//     let positions: Vec<_> = NUMBERS
//         .into_iter()
//         .filter_map(|c| Some(layout.char_idx()[c]?.pos))
//         .collect();
//     if positions.len() != 10 {
//         return 0.;
//     }
//     NUM_LAYOUTS
//         .iter()
//         .map(|l| hamming_dist(l, &positions))
//         .min()
//         .unwrap() as f64
//     // let rl = positions.iter().copied().map(|(r, _)| r).min().unwrap();
//     // let cl = positions.iter().copied().map(|(_, c)| c).min().unwrap();
//     // let rh = positions.iter().copied().map(|(r, _)| r).max().unwrap();
//     // let ch = positions.iter().copied().map(|(_, c)| c).max().unwrap();
//     // let mut out = 0.;
//     // let w = 1 + ch - cl;
//     // let h = 1 + rh - rl;
//     // match (w, h) {
//     //     (4, 3) => {
//     //         // A numpad-style layout.
//     //         let cl19 = positions[1..]
//     //             .iter()
//     //             .copied()
//     //             .map(|(_, c)| c)
//     //             .min()
//     //             .unwrap();
//     //         out += positions[1].1.abs_diff(cl19) as f64;
//     //         for w in positions[1..].windows(2) {
//     //             if let [a, b] = w {
//     //                 if b.0 == a.0 {
//     //                     out += 0.1 * (a.1 + 1).abs_diff(b.1) as f64;
//     //                 } else {
//     //                     out += b.1.abs_diff(cl19) as f64;
//     //                 }
//     //             } else {
//     //                 unreachable!();
//     //             }
//     //         }
//     //     }
//     //     (5, 2) => {
//     //         // A block layout.
//     //         out += positions[0].1.abs_diff(cl) as f64;
//     //         for w in positions.windows(2) {
//     //             if let [a, b] = w {
//     //                 if b.0 == a.0 {
//     //                     out += 0.1 * (a.1 + 1).abs_diff(b.1) as f64;
//     //                 } else {
//     //                     out += b.1.abs_diff(cl) as f64;
//     //                 }
//     //             } else {
//     //                 unreachable!();
//     //             }
//     //         }
//     //     }
//     //     (11, 1) => {
//     //         // A single row.
//     //         out += positions[0].1.abs_diff(cl) as f64;
//     //         for w in positions.windows(2) {
//     //             if let [a, b] = w {
//     //                 if b.0 == a.0 {
//     //                     out += 0.1 * (a.1 + 1).abs_diff(b.1) as f64;
//     //                 } else {
//     //                     out += b.1.abs_diff(cl) as f64;
//     //                 }
//     //             } else {
//     //                 unreachable!();
//     //             }
//     //         }
//     //     }
//     //     _ => {
//     //         out += 2. * ((w * h - 10) as f64).sqrt();
//     //         for w in positions.windows(2) {
//     //             if let [a, b] = w {
//     //                 if b.0 == a.0 {
//     //                     out += 0.1 * (a.1 + 1).abs_diff(b.1) as f64;
//     //                 } else {
//     //                     out += 0.05;
//     //                 }
//     //             } else {
//     //                 unreachable!();
//     //             }
//     //         }
//     //     }
//     // }
//     // out
// }

fn layer_variation(char_idx: &CharIdx, chars: impl IntoIterator<Item = Win1252Char>) -> f64 {
    let layers: Vec<_> = chars
        .into_iter()
        .filter_map(|c| {
            let at = char_idx[c]?;
            Some((at.layer, at.shifted))
        })
        .collect();
    if layers.is_empty() {
        return 0.;
    }
    let mut num_different = 0;
    for (i, &a) in layers.iter().enumerate() {
        num_different += layers[i + 1..].iter().filter(|&&b| b != a).count();
    }
    num_different as f64 / layers.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_norm_small() {
        let inputs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected = [0, 1, 1, 2, 2, 2, 2, 3, 3, 3];
        assert_eq!(inputs.map(log_norm), expected);
    }
}
