use encoding_rs::WINDOWS_1252;
use lazy_static::lazy_static;

use crate::{
    evolve::{AnnotatedLayout, CharIdx, TypingEvent},
    layout::{finger_for_pos, Digit, Finger, Hand, Layer, NUM_KEYS},
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
            Finger::Middle => 7,
            Finger::Ring => 12,
            Finger::Pinky => 18,
            Finger::Thumb => 10,
        };
        let sq_dist =
            vert_penalty(f0) as usize * row_dist * row_dist + horiz_penalty * col_dist * col_dist;
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
        let dist = log_norm(row_dist * vert_penalty(f1) as usize);
        (if outward { OUTWARD_PENALTY } else { 0 }) + (if stretch { 2 } else { 0 }) + dist
    } else {
        // Different hand.
        2
    }
}

pub(crate) fn log_norm(x: usize) -> u8 {
    // To distinguish between 0 and 1;
    let x = x + 1;
    // Calculate the integer log2, rounded down.
    let shift = usize::BITS - x.leading_zeros();
    assert!(shift > 0);
    assert!(shift - 1 <= u8::MAX.into());
    (shift - 1) as u8
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
        let dist = log_norm(row_dist * vert_penalty(f1) as usize);
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

fn similarity_cost(_layout: &AnnotatedLayout) -> f64 {
    0.
    // layout.hamming_dist(&DEFAULT_LAYOUT) as f64 / (NUM_KEYS * NUM_LAYERS) as f64 * 0.5
}

lazy_static! {
    static ref ORDERED_PAIRS: [[u8; 2]; 4] =
        [["(", ")"], ["{", "}"], ["[", "]"], ["<", ">"],].map(|p| p.map(|s| {
            let (out, _, had_errors) = WINDOWS_1252.encode(s);
            assert!(!had_errors);
            assert!(out.len() == 1);
            out[0]
        }));
    static ref SIMILAR_PAIRS: [[u8; 2]; 17] = [
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
    .map(|p| p.map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    }));
    static ref SPACE: u8 = {
        let (out, _, had_errors) = WINDOWS_1252.encode(" ");
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref LOWER_ALPHA: [u8; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ]
    .map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    });
    static ref UPPER_ALPHA: [u8; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ]
    .map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    });
    static ref NUMBERS: [u8; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    });
    static ref MATHS_SYMBOLS: [u8; 13] =
        ["+", "-", "*", "/", "%", "=", "!", "@", "<", ">", "^", "&", "|",].map(|s| {
            let (out, _, had_errors) = WINDOWS_1252.encode(s);
            assert!(!had_errors);
            assert!(out.len() == 1);
            out[0]
        });
    static ref BRACKETS: [u8; 8] = ["(", ")", "{", "}", "[", "]", "<", ">"].map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    });
    static ref QUOTES: [u8; 3] = ["'", "\"", "`"].map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    });
    static ref PUNCTUATION: [u8; 9] = [",", ".", ";", ":", "!", "?", "\"", "'", "-"].map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    });
    static ref LINES: [u8; 6] = ["-", "_", "\\", "|", "/", "~"].map(|s| {
        let (out, _, had_errors) = WINDOWS_1252.encode(s);
        assert!(!had_errors);
        assert!(out.len() == 1);
        out[0]
    });
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
    let space_penalty = match layout.char_idx()[*SPACE].unwrap().pos {
        31 => 0.,
        32 => 1.,
        _ => 3.,
    };
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
        + 0.01 * layer_variation(layout.char_idx(), *NUMBERS)
        + 0.002 * layer_variation(layout.char_idx(), *MATHS_SYMBOLS)
        + 0.002 * layer_variation(layout.char_idx(), *BRACKETS)
        + 0.002 * layer_variation(layout.char_idx(), *QUOTES)
        + 0.002 * layer_variation(layout.char_idx(), *PUNCTUATION)
        + 0.002 * layer_variation(layout.char_idx(), *LINES)
        + 0.1 * space_penalty
        + 0.1 * shift_penalty
        + 0.1 * layer_penalty
}

fn layer_variation(char_idx: &CharIdx, chars: impl IntoIterator<Item = u8>) -> f64 {
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

pub fn layout_cost(layout: &AnnotatedLayout) -> f64 {
    similarity_cost(layout) + memorability_cost(layout)
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
