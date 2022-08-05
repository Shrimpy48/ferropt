use enum_map::enum_map;

use std::num::NonZeroU64;

use crate::layout::{finger_for_pos, AnnotatedLayout, Digit, TypingEvent};

use super::{log_norm, CostModel};

pub struct Model;

impl Default for Model {
    fn default() -> Self {
        Self
    }
}

impl CostModel for Model {
    fn cost_of_typing(&self, keys: impl Iterator<Item = TypingEvent>) -> (u64, u64) {
        let mut last_used = enum_map! {
            Digit::LeftPinky => LastUsedEntry {
                at: None,
                row: 1,
                col: 0,
            },
            Digit::LeftRing => LastUsedEntry {
                at: None,
                row: 1,
                col: 1,
            },
            Digit::LeftMiddle => LastUsedEntry {
                at: None,
                row: 1,
                col: 2,
            },
            Digit::LeftIndex => LastUsedEntry {
                at: None,
                row: 1,
                col: 3,
            },
            Digit::RightIndex => LastUsedEntry {
                at: None,
                row: 1,
                col: 6,
            },
            Digit::RightMiddle => LastUsedEntry {
                at: None,
                row: 1,
                col: 7,
            },
            Digit::RightRing => LastUsedEntry {
                at: None,
                row: 1,
                col: 8,
            },
            Digit::RightPinky => LastUsedEntry {
                at: None,
                row: 1,
                col: 9,
            },
            Digit::LeftThumb => LastUsedEntry {
                at: None,
                row: 3,
                col: 1,
            },
            Digit::RightThumb => LastUsedEntry {
                at: None,
                row: 3,
                col: 2,
            },
        };

        let mut count = 0;
        let mut total_cost = 0;
        for (i, event) in (1u64..).zip(keys) {
            match event {
                TypingEvent::Tap { pos, for_char } => {
                    let r = pos / 10;
                    let c = pos % 10;
                    let digit = finger_for_pos(r, c);
                    let dr = last_used[digit].row.abs_diff(r);
                    let dc = last_used[digit].col.abs_diff(c);
                    let weighted_sq_dist = dr.pow(2) + (2 * dc).pow(2);
                    if let Some(at) = last_used[digit].at {
                        let x: u64 = 10 * (1 + log_norm(weighted_sq_dist) as u64) / (i - at.get());
                        total_cost += x;
                    } else {
                        total_cost += 1 + log_norm(weighted_sq_dist) as u64;
                    }
                    assert!(i > 0);
                    last_used[digit].at = NonZeroU64::new(i);
                    last_used[digit].row = r;
                    last_used[digit].col = c;
                    if for_char {
                        count += 1;
                    }
                }
                _ => {}
            }
        }
        (total_cost, count)
    }
    fn layout_cost(&self, _layout: &AnnotatedLayout) -> f64 {
        0.
    }
}

#[derive(Debug, Clone, Copy)]
struct LastUsedEntry {
    at: Option<NonZeroU64>,
    row: u8,
    col: u8,
}
