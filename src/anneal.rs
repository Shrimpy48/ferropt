use rand::{thread_rng, Rng};

use std::cmp;

use crate::cost::CostModel;

use crate::layout::{AnnotatedLayout, Key, Layout, Win1252Char, NUM_KEYS};

#[derive(Debug, Clone, Copy)]
enum Mutation {
    SwapKeys {
        layer_a: u8,
        layer_b: u8,
        pos_a: u8,
        pos_b: u8,
    },
    // SwapPaired {
    //     l0: u8,
    //     l1: u8,
    //     i: u8,
    //     j: u8,
    // },
}

impl Mutation {
    fn gen<R: Rng>(mut rng: R, layout: &AnnotatedLayout) -> Self {
        let layer = rng.gen_range(0..layout.num_layers());
        let i = rng.gen_range(0..NUM_KEYS);
        if matches!(layout.layout()[layer][i], Key::Layer(_) | Key::Shift) {
            assert_eq!(layer, 0);
            // Keep layer switch keys on home layer
            // to ensure every layer can be accessed.
            let j = rng.gen_range(0..NUM_KEYS);
            Self::SwapKeys {
                layer_a: 0,
                layer_b: 0,
                pos_a: i,
                pos_b: j,
            }
        } else {
            let layer2 = rng.gen_range(0..layout.num_layers());
            let j = loop {
                let j = rng.gen_range(0..NUM_KEYS);
                if !matches!(layout.layout()[layer2][j], Key::Layer(_) | Key::Shift) {
                    break j;
                }
            };
            Self::SwapKeys {
                layer_a: layer,
                layer_b: layer2,
                pos_a: i,
                pos_b: j,
            }
        }
    }

    fn apply(self, layout: &mut AnnotatedLayout) {
        match self {
            Self::SwapKeys {
                layer_a: l0,
                pos_a: i,
                layer_b: l1,
                pos_b: j,
            } => layout.swap((l0, i), (l1, j)),
            // Self::SwapPaired { l0, l1, i, j } => {
            //     layout.swap((l0, i), (l0, j));
            //     layout.swap((l1, i), (l1, j));
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
pub fn optimise<M: CostModel>(
    cost_model: M,
    n: u32,
    layout: Layout,
    corpus: &[Vec<Win1252Char>],
    mut progress_callback: impl FnMut(u32),
) -> (Layout, f64) {
    let mut layout: AnnotatedLayout = layout.into();
    let mut rng = thread_rng();
    let initial_energy = cost_model.cost(corpus, &layout);
    let t0 = initial_energy;
    let mut energy = initial_energy;
    for i in 0..n {
        progress_callback(i);
        let mutation = Mutation::gen(&mut rng, &layout);
        mutation.apply(&mut layout);
        let new_energy = cost_model.cost(corpus, &layout);
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
    (layout.into(), initial_energy - energy)
}
