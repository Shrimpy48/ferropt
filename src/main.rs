use ferropt::evolve::*;
use ferropt::layout::*;

use clap::Parser;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use rayon::iter;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    quiet: bool,
    #[clap(short = 'n', long, value_parser, default_value_t = 10_000)]
    iterations: u32,
    #[clap(short, long, value_parser, default_value_t = 5)]
    trials: u32,
    #[clap(short, long, value_parser, default_value = "corpus")]
    corpus: PathBuf,
    #[clap(short = 'l', long, value_parser, default_value = "initial.json")]
    initial_layout: PathBuf,
    #[clap(short, long, value_parser, default_value = "optimised.json")]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(feature = "dhat-ad-hoc")]
    let _profiler = dhat::Profiler::new_ad_hoc();

    let args = Args::parse();

    let f = File::open(&args.initial_layout)?;
    let layout = serde_json::from_reader(f)?;

    let corpus = read_corpus(&args.corpus)?;

    let (l, _score) = run_trials(&args, &corpus, &layout);
    let f = File::create(args.output)?;
    serde_json::to_writer_pretty(f, &l)?;

    Ok(())
}

fn run_trials(args: &Args, corpus: &[Vec<u8>], layout: &Layout) -> (Layout, f64) {
    let start = Instant::now();

    let results: Vec<_> = if args.quiet {
        iter::repeatn(layout, args.trials as usize)
            .map(|l| optimise(args.iterations, l.clone(), corpus, |_i| {}))
            .collect()
    } else {
        let multiprog = MultiProgress::new();
        multiprog.set_move_cursor(true);

        let bars: Vec<_> =
            std::iter::repeat_with(|| multiprog.add(ProgressBar::new(args.iterations.into())))
                .take(args.trials as usize)
                .collect();

        for bar in bars.iter() {
            bar.set_position(0);
        }

        rayon::join(
            || multiprog.join_and_clear().unwrap(),
            || {
                iter::repeatn(layout, args.trials as usize)
                    .zip(bars)
                    .map(|(l, bar)| {
                        let res = optimise(args.iterations, l.clone(), corpus, |i| {
                            bar.set_position(i.into())
                        });
                        bar.finish();
                        res
                    })
                    .collect()
            },
        )
        .1
    };

    // let mut bar = ProgressBar::new(n.into());
    // let results = vec![optimise(n, layout.clone(), corpus, |i| {
    //     bar.set_position(i.into())
    // })];
    // bar.finish();

    // let results = vec![optimise(n, layout.clone(), corpus, |_i| {})];

    let mean = results.iter().map(|(_, i)| i).sum::<f64>() / args.trials as f64;
    let var = results.iter().map(|(_, i)| (i - mean).powi(2)).sum::<f64>() / args.trials as f64;
    let std_dev = var.sqrt();

    let mad = results
        .iter()
        .flat_map(|(a, _)| results.iter().map(|(b, _)| a.hamming_dist(b) as u32))
        .sum::<u32>() as f64
        / (args.trials * args.trials) as f64;

    let best = results
        .into_iter()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap();

    if !args.quiet {
        eprintln!(
            "layout MD = {:6.2}, mean = {:6.3}, std dev = {:5.3}, best = {:6.3} in {}",
            mad,
            mean,
            std_dev,
            best.1,
            HumanDuration(start.elapsed()),
        );
    }

    best
}
