use enum_map::enum_map;

use std::num::NonZeroU64;

use crate::{
    cost::log_norm,
    evolve::{AnnotatedLayout, TypingEvent},
    layout::{finger_for_pos, Digit, Finger, Hand},
};

#[derive(Debug, Clone, Copy)]
struct LastUsedEntry {
    at: Option<NonZeroU64>,
    row: usize,
    col: usize,
}

pub fn cost_of_typing(keys: impl Iterator<Item = TypingEvent>) -> (u64, u64) {
    use Finger::*;
    use Hand::*;
    let mut last_used = enum_map! {
        Digit {
            hand: Left,
            finger: Pinky,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 0,
        },
        Digit {
            hand: Left,
            finger: Ring,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 1,
        },
        Digit {
            hand: Left,
            finger: Middle,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 2,
        },
        Digit {
            hand: Left,
            finger: Index,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 3,
        },
        Digit {
            hand: Right,
            finger: Index,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 6,
        },
        Digit {
            hand: Right,
            finger: Middle,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 7,
        },
        Digit {
            hand: Right,
            finger: Ring,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 8,
        },
        Digit {
            hand: Right,
            finger: Pinky,
        } => LastUsedEntry {
            at: None,
            row: 1,
            col: 9,
        },
        Digit {
            hand: Left,
            finger: Thumb,
        } => LastUsedEntry {
            at: None,
            row: 3,
            col: 1,
        },
        Digit {
            hand: Right,
            finger: Thumb,
        } => LastUsedEntry {
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

pub fn layout_cost(_layout: &AnnotatedLayout) -> f64 {
    0.
}
