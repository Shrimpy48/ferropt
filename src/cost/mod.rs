use crate::layout::{keys, oneshot, AnnotatedLayout, TypingEvent, Win1252Char};

pub mod heuristic;
pub mod measured;
pub mod simple;

pub trait CostModel {
    fn cost_of_typing(&self, keys: impl Iterator<Item = TypingEvent>) -> f64;

    fn layout_cost(&self, layout: &AnnotatedLayout) -> f64;

    fn string_cost(&self, layout: &AnnotatedLayout, string: &[Win1252Char]) -> (f64, u64) {
        // let keys = keys(&layout.char_idx, string.chars());
        // let events = key_seq(layout.layer_idx, layout.shift_idx, keys);

        let events = oneshot(keys(layout, string.iter().copied()));

        (self.cost_of_typing(events), string.len() as u64)
    }

    fn cost(&self, corpus: &[Vec<Win1252Char>], layout: &AnnotatedLayout) -> f64 {
        let (total, count) = corpus
            .iter()
            .map(|s| self.string_cost(layout, s))
            .fold((0., 0), |a, b| (a.0 + b.0, a.1 + b.1));
        debug_assert!(total.is_finite());

        let typing_cost = total / count as f64;
        typing_cost + self.layout_cost(layout)
    }
}

impl<M: CostModel + ?Sized> CostModel for &M {
    fn cost_of_typing(&self, keys: impl Iterator<Item = TypingEvent>) -> f64 {
        (*self).cost_of_typing(keys)
    }

    fn layout_cost(&self, layout: &AnnotatedLayout) -> f64 {
        (*self).layout_cost(layout)
    }

    fn string_cost(&self, layout: &AnnotatedLayout, string: &[Win1252Char]) -> (f64, u64) {
        (*self).string_cost(layout, string)
    }

    fn cost(&self, corpus: &[Vec<Win1252Char>], layout: &AnnotatedLayout) -> f64 {
        (*self).cost(corpus, layout)
    }
}

pub(crate) fn log_norm(x: u8) -> u8 {
    // To distinguish between 0 and 1;
    let x = x + 1;
    // Calculate the integer log2, rounded down.
    let shift = u8::BITS - x.leading_zeros();
    assert!(shift > 0);
    assert!(shift - 1 <= u8::MAX.into());
    (shift - 1) as u8
}
