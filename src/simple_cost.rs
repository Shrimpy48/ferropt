use enum_map::EnumMap;

use crate::{
    evolve::{AnnotatedLayout, TypingEvent},
    layout::finger_for_pos,
};

pub fn cost_of_typing(keys: impl Iterator<Item = TypingEvent>) -> (u64, u64) {
    let mut last_used = EnumMap::default();

    let mut count = 0;
    let mut total_cost = 0;
    for (i, event) in (1u64..).zip(keys) {
        match event {
            TypingEvent::Tap { pos, for_char } => {
                let r = pos / 10;
                let c = pos % 10;
                let digit = finger_for_pos(r, c);
                if let Some(l) = last_used[digit] {
                    let x: u64 = 120 / (i - l);
                    total_cost += x;
                } else {
                    total_cost += 1;
                }
                last_used[digit] = Some(i);
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
