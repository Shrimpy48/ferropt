mod cost;
mod evolve;
mod layout;
mod simple_cost;

use evolve::*;
use layout::*;

use rayon::iter;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::time::Instant;

use indicatif::{HumanDuration, MultiProgress, ProgressBar};

const TRIALS: usize = 21;

fn main() -> io::Result<()> {
    let f = File::open("initial.json")?;
    let layout = serde_json::from_reader(f)?;
    eprintln!("{}", serde_json::to_string_pretty(&layout)?);

    let corpus = read_corpus()?;

    let mut best = 0.;

    for n in [100, 1_000, 5_000] {
        let (l, score) = run_trials(n, &corpus, &layout);
        if score > best {
            best = score;
            let f = File::create("test.json")?;
            serde_json::to_writer_pretty(f, &l)?;
        }
    }

    Ok(())
}

fn run_trials(n: u32, corpus: &[String], layout: &Layout) -> (Layout, f64) {
    let start = Instant::now();

    let multiprog = MultiProgress::new();
    multiprog.set_move_cursor(true);

    let bars: Vec<_> = std::iter::repeat_with(|| multiprog.add(ProgressBar::new(n.into())))
        .take(TRIALS)
        .collect();

    for bar in bars.iter() {
        bar.set_position(0);
    }

    let (_, results): (_, Vec<_>) = rayon::join(
        || multiprog.join_and_clear().unwrap(),
        || {
            iter::repeatn(layout, TRIALS)
                .zip(bars)
                .map(|(l, bar)| {
                    let res = optimise(n, l.clone(), corpus, |i| bar.set_position(i.into()));
                    bar.finish();
                    res
                })
                .collect()
        },
    );

    // let mut bar = ProgressBar::new(n.into());
    // let results = vec![optimise(n, layout.clone(), corpus, |i| {
    //     bar.set_position(i.into())
    // })];
    // bar.finish();

    // let results = vec![optimise(n, layout.clone(), corpus, |_i| {})];

    let mean = results.iter().map(|(_, i)| i).sum::<f64>() / TRIALS as f64;
    let var = results.iter().map(|(_, i)| (i - mean).powi(2)).sum::<f64>() / TRIALS as f64;
    let std_dev = var.sqrt();

    let best = results
        .into_iter()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap();

    eprintln!(
        "N = {:6}: μ = {:6.3}, σ = {:6.3}, best = {:6.3} in {}",
        n,
        mean,
        std_dev,
        best.1,
        HumanDuration(start.elapsed()),
    );

    best
}
