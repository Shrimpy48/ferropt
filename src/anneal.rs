use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

use std::cmp;

use crate::cost::CostModel;
use crate::layout::{
    AnnotatedLayout, Key, Layout, Win1252Char, LOWER_ALPHA, NUMBERS, NUM_KEYS, NUM_LAYOUTS,
    UPPER_ALPHA,
};

lazy_static! {
    /// Keys which the optimiser may not move.
    static ref PINNED_KEYS: Vec<(u8, u8)> = vec![(0, 31)];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PinnedTo {
    Layer,
    Key,
    Position,
}

// WARNING: number layout switching does NOT respect pinning.
fn pinned(layout: &AnnotatedLayout, layer: u8, pos: u8) -> Option<PinnedTo> {
    let out = if PINNED_KEYS.contains(&(layer, pos)) {
        Some(PinnedTo::Position)
    } else if matches!(layout.layout()[layer][pos], Key::Layer(_) | Key::Shift) {
        Some(PinnedTo::Layer)
    } else if let Some(c) = layout.layout()[layer][pos].typed_char(false) {
        if LOWER_ALPHA.contains(&c) || UPPER_ALPHA.contains(&c) {
            Some(PinnedTo::Position)
        } else {
            None
        }
    } else {
        None
    };

    if layout.num_layers() == 1 {
        if out == Some(PinnedTo::Layer) {
            return None;
        }
        if out == Some(PinnedTo::Key) {
            return Some(PinnedTo::Position);
        }
    }
    out
}

#[derive(Debug, Clone, Copy)]
enum Mutation {
    SwapKeys {
        layer_a: u8,
        layer_b: u8,
        pos_a: u8,
        pos_b: u8,
    },
    SwapNumLayout {
        layout_a: u8,
        layout_b: u8,
    },
    // SwapPaired {
    //     l0: u8,
    //     l1: u8,
    //     i: u8,
    //     j: u8,
    // },
}

impl Mutation {
    fn gen_impl<R: Rng>(rng: &mut R, layout: &AnnotatedLayout) -> Self {
        let mut i = 0;
        let (layer_a, pos_a, a_pinned_to) = loop {
            let layer = rng.gen_range(0..layout.num_layers());
            let pos = rng.gen_range(0..NUM_KEYS);
            let pinned_to = pinned(layout, layer, pos);
            if pinned_to != Some(PinnedTo::Position) {
                break (layer, pos, pinned_to);
            }
            i += 1;
            if i > 1000 {
                panic!("stuck picking A");
            }
        };
        let is_digit = layout.layout()[layer_a][pos_a]
            .typed_char(false)
            .map(|c| NUMBERS.contains(&c))
            .unwrap_or(false);
        if is_digit {
            let mut i = 0;
            let layout_b = loop {
                let layout_b = rng.gen_range(0..NUM_LAYOUTS.len() as u8);
                if layout_b != layout.num_layout() {
                    break layout_b;
                }
                i += 1;
                if i > 1000 {
                    panic!("stuck picking num layout");
                }
            };
            Self::SwapNumLayout {
                layout_a: layout.num_layout(),
                layout_b,
            }
        } else {
            match a_pinned_to {
                Some(PinnedTo::Layer) => {
                    let mut i = 0;
                    let pos_b = loop {
                        let pos = rng.gen_range(0..NUM_KEYS);
                        let is_digit = layout.layout()[layer_a][pos]
                            .typed_char(false)
                            .map(|c| NUMBERS.contains(&c))
                            .unwrap_or(false);
                        if !matches!(
                            pinned(layout, layer_a, pos),
                            Some(PinnedTo::Key | PinnedTo::Position)
                        ) && !is_digit
                        {
                            break pos;
                        }
                        i += 1;
                        if i > 1000 {
                            panic!("stuck picking B on same layer");
                        }
                    };
                    Self::SwapKeys {
                        layer_a,
                        layer_b: layer_a,
                        pos_a,
                        pos_b,
                    }
                }
                Some(PinnedTo::Key) => {
                    let mut i = 0;
                    let layer_b = loop {
                        let layer = rng.gen_range(0..layout.num_layers());
                        let is_digit = layout.layout()[layer][pos_a]
                            .typed_char(false)
                            .map(|c| NUMBERS.contains(&c))
                            .unwrap_or(false);
                        if !matches!(
                            pinned(layout, layer, pos_a),
                            Some(PinnedTo::Layer | PinnedTo::Position)
                        ) && !is_digit
                        {
                            break layer;
                        }
                        i += 1;
                        if i > 1000 {
                            panic!("stuck picking B on same key");
                        }
                    };
                    Self::SwapKeys {
                        layer_a,
                        layer_b,
                        pos_a,
                        pos_b: pos_a,
                    }
                }
                Some(PinnedTo::Position) => unreachable!(),
                None => {
                    let mut i = 0;
                    let (layer_b, pos_b) = loop {
                        let layer = rng.gen_range(0..layout.num_layers());
                        let pos = rng.gen_range(0..NUM_KEYS);
                        let is_digit = layout.layout()[layer][pos]
                            .typed_char(false)
                            .map(|c| NUMBERS.contains(&c))
                            .unwrap_or(false);
                        if pinned(layout, layer, pos).is_none() && !is_digit {
                            break (layer, pos);
                        }
                        i += 1;
                        if i > 1000 {
                            panic!("stuck picking B");
                        }
                    };
                    Self::SwapKeys {
                        layer_a,
                        layer_b,
                        pos_a,
                        pos_b,
                    }
                }
            }
        }
    }

    fn gen<R: Rng>(rng: &mut R, layout: &AnnotatedLayout) -> Self {
        loop {
            let change = Self::gen_impl(rng, layout);
            match change {
                Self::SwapNumLayout { layout_a, layout_b } => {
                    if layout_a != layout_b {
                        break change;
                    }
                }
                Self::SwapKeys {
                    layer_a,
                    pos_a,
                    layer_b,
                    pos_b,
                } => {
                    #[cfg(debug_assertions)]
                    {
                        if layer_a != layer_b {
                            assert!(!matches!(
                                pinned(layout, layer_a, pos_a),
                                Some(PinnedTo::Layer | PinnedTo::Position)
                            ));
                            assert!(!matches!(
                                pinned(layout, layer_b, pos_b),
                                Some(PinnedTo::Layer | PinnedTo::Position)
                            ));
                        }
                        if pos_a != pos_b {
                            assert!(!matches!(
                                pinned(layout, layer_a, pos_a),
                                Some(PinnedTo::Key | PinnedTo::Position)
                            ));
                            assert!(!matches!(
                                pinned(layout, layer_b, pos_b),
                                Some(PinnedTo::Key | PinnedTo::Position)
                            ));
                        }
                    }
                    if layout.layout()[layer_a][pos_a] != layout.layout()[layer_b][pos_b] {
                        break change;
                    }
                }
            }
        }
    }

    fn apply(self, layout: &mut AnnotatedLayout) {
        match self {
            Self::SwapKeys {
                layer_a,
                pos_a,
                layer_b,
                pos_b,
            } => layout.swap((layer_a, pos_a), (layer_b, pos_b)),
            Self::SwapNumLayout { layout_a, layout_b } => {
                assert!(layout.num_layout() == layout_a);
                layout.switch_to_num_layout(layout_b);
            } // Self::SwapPaired { l0, l1, i, j } => {
              //     layout.swap((l0, i), (l0, j));
              //     layout.swap((l1, i), (l1, j));
              // }
        }
    }

    fn undo(self, layout: &mut AnnotatedLayout) {
        match self {
            Self::SwapKeys { .. } => self.apply(layout),
            Self::SwapNumLayout { layout_a, layout_b } => {
                assert!(layout.num_layout() == layout_b);
                layout.switch_to_num_layout(layout_a);
            }
        }
    }
}

const P0: f64 = 1.;

/// Optimise the layout using simulated annealing with a fixed number of iterations.
/// Returns the optimised layout and the percentage cost improvement.
pub fn optimise_fixed<M: CostModel>(
    cost_model: M,
    n: u32,
    k: f64,
    temp_scale: f64,
    layout: Layout,
    corpus: &[Vec<Win1252Char>],
    mut progress_callback: impl FnMut(u32),
) -> (Layout, f64) {
    let mut layout: AnnotatedLayout = layout.into();
    let mut rng = thread_rng();
    let initial_energy = cost_model.cost(corpus, &layout);
    let t0 = initial_energy * temp_scale;
    let mut energy = initial_energy;
    for i in 0..n {
        progress_callback(i);
        let mutation = Mutation::gen(&mut rng, &layout);
        mutation.apply(&mut layout);
        let new_energy = cost_model.cost(corpus, &layout);
        match new_energy.partial_cmp(&energy).unwrap() {
            cmp::Ordering::Less | cmp::Ordering::Equal => {}
            cmp::Ordering::Greater => {
                let temperature = t0 * (-k * i as f64 / n as f64).exp();
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
    (
        layout.into(),
        100. * (initial_energy - energy) / initial_energy,
    )
}

/// Optimise the layout using simulated annealing until a local minimum is reached.
/// Returns the optimised layout and the percentage cost improvement.
pub fn optimise_until_stable<M: CostModel>(
    cost_model: M,
    max_unchanged: u32,
    hl: f64,
    temp_scale: f64,
    layout: Layout,
    corpus: &[Vec<Win1252Char>],
) -> (Layout, f64) {
    let mut layout: AnnotatedLayout = layout.into();
    let mut rng = thread_rng();
    let initial_energy = cost_model.cost(corpus, &layout);
    let t0 = initial_energy * temp_scale;
    let mut energy = initial_energy;
    let mut unchanged_count = 0;
    for i in 0.. {
        let mutation = Mutation::gen(&mut rng, &layout);
        mutation.apply(&mut layout);
        let new_energy = cost_model.cost(corpus, &layout);
        match new_energy.partial_cmp(&energy).unwrap() {
            cmp::Ordering::Less => {
                unchanged_count = 0;
            }
            cmp::Ordering::Equal => {
                unchanged_count += 1;
            }
            cmp::Ordering::Greater => {
                let temperature = t0 * (-i as f64 / hl).exp2();
                let p = P0 * ((energy - new_energy) / temperature).exp();
                if !rng.gen_bool(p) {
                    mutation.undo(&mut layout);
                    unchanged_count += 1;
                    if unchanged_count >= max_unchanged {
                        break;
                    }
                    continue;
                } else {
                    unchanged_count = 0;
                }
            }
        }
        energy = new_energy;
        // eprintln!("iteration {}, energy = {}", i, energy);
    }
    // eprintln!("improvement: {}", initial_energy - energy);
    (
        layout.into(),
        100. * (initial_energy - energy) / initial_energy,
    )
}

/// Optimise the layout using simulated annealing until a local minimum is reached.
/// Returns the optimised layout and the percentage cost improvement.
pub fn optimise_log<M: CostModel>(
    cost_model: M,
    max_unchanged: u32,
    hl: f64,
    temp_scale: f64,
    layout: Layout,
    corpus: &[Vec<Win1252Char>],
    mut log_writer: impl std::io::Write,
) -> (Layout, f64) {
    let mut layout: AnnotatedLayout = layout.into();
    let mut rng = thread_rng();
    let initial_energy = cost_model.cost(corpus, &layout);
    let t0 = initial_energy * temp_scale;
    let mut energy = initial_energy;
    let mut unchanged_count = 0;
    writeln!(log_writer, "iteration,temperature,energy").unwrap();
    for i in 0.. {
        let temperature = t0 * (-i as f64 / hl).exp2();
        // let temperature = t0 * (1. - i as f64 / (hl * 2.)).max(0.);
        writeln!(log_writer, "{i},{temperature},{energy}").unwrap();
        let mutation = Mutation::gen(&mut rng, &layout);
        mutation.apply(&mut layout);
        let new_energy = cost_model.cost(corpus, &layout);
        match new_energy.partial_cmp(&energy).unwrap() {
            cmp::Ordering::Less => {
                unchanged_count = 0;
            }
            cmp::Ordering::Equal => {
                unchanged_count += 1;
            }
            cmp::Ordering::Greater => {
                let p = P0 * ((energy - new_energy) / temperature).exp();
                if !rng.gen_bool(p) {
                    mutation.undo(&mut layout);
                    unchanged_count += 1;
                    if unchanged_count >= max_unchanged {
                        break;
                    }
                    continue;
                } else {
                    unchanged_count = 0;
                }
            }
        }
        energy = new_energy;
        // eprintln!("iteration {}, energy = {}", i, energy);
    }
    // eprintln!("improvement: {}", initial_energy - energy);
    (
        layout.into(),
        100. * (initial_energy - energy) / initial_energy,
    )
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn mutation_apply_undo_inverses() {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let mut layout: AnnotatedLayout = layout.into();

        let mut rng = thread_rng();

        for _ in 0..1000 {
            let start = layout.clone();
            let mutation = Mutation::gen(&mut rng, &layout);
            mutation.apply(&mut layout);
            mutation.undo(&mut layout);
            assert_eq!(
                start.layout(),
                layout.layout(),
                "mutation {mutation:?} not undone correctly"
            );
        }
    }

    #[test]
    fn mutation_apply_undo_shuffled() {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let mut layout: AnnotatedLayout = layout.into();

        let mut rng = thread_rng();

        for _ in 0..1000 {
            let mutation = Mutation::gen(&mut rng, &layout);
            mutation.apply(&mut layout);
        }

        for _ in 0..1000 {
            let start = layout.clone();
            let mutation = Mutation::gen(&mut rng, &layout);
            mutation.apply(&mut layout);
            mutation.undo(&mut layout);
            assert_eq!(
                start.layout(),
                layout.layout(),
                "mutation {mutation:?} not undone correctly"
            );
        }
    }

    #[test]
    fn mutation_apply_undo_optimised() {
        let f = File::open("optimised_noshifted.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let mut layout: AnnotatedLayout = layout.into();

        let mut rng = thread_rng();

        for _ in 0..1000 {
            let start = layout.clone();
            let mutation = Mutation::gen(&mut rng, &layout);
            mutation.apply(&mut layout);
            mutation.undo(&mut layout);
            assert_eq!(
                start.layout(),
                layout.layout(),
                "mutation {mutation:?} not undone correctly"
            );
        }
    }
}
