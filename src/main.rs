mod evolve;
mod layout;

use evolve::*;
use layout::*;

use rayon::iter;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::time::Instant;

const TRIALS: usize = 16;

fn main() -> io::Result<()> {
    let f = File::open("initial.json")?;
    let layout = serde_json::from_reader(f)?;
    eprintln!("{}", serde_json::to_string_pretty(&layout)?);

    let corpus = read_corpus()?;

    let mut best = 0.;

    for n in [100, 1_000, 2_500, 5_000, 10_000, 25_000, 50_000] {
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
    let results: Vec<_> = iter::repeatn(layout, TRIALS)
        .map(|l| optimise(n, l.clone(), corpus))
        .collect();

    let mean = results.iter().map(|(_, i)| i).sum::<f64>() / TRIALS as f64;
    let var = results.iter().map(|(_, i)| (i - mean).powi(2)).sum::<f64>() / TRIALS as f64;
    let std_dev = var.sqrt();

    let best = results
        .into_iter()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap();

    eprintln!(
        "N = {:6}: μ = {:6.3}, σ = {:6.3}, best = {:6.3} in {:.1?}",
        n,
        mean,
        std_dev,
        best.1,
        start.elapsed(),
    );

    best
}
