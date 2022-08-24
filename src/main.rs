use ferropt::anneal::*;
use ferropt::cost::measured::Model;
use ferropt::layout::*;

use clap::Parser;
use indicatif::ProgressStyle;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use rayon::iter;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::num::NonZeroU16;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    quiet: bool,
    #[clap(short = 'n', long, value_parser, default_value_t = 10_000)]
    iterations: u32,
    #[clap(short = 'k', long, value_parser, default_value_t = 10.)]
    cooling_rate: f64,
    #[clap(short = 's', long, value_parser, default_value_t = 1.5)]
    temp_scale: f64,
    #[clap(short, long, value_parser, default_value = "5")]
    trials: NonZeroU16,
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

fn run_trials(args: &Args, corpus: &[Vec<Win1252Char>], layout: &Layout) -> (Layout, f64) {
    let start = Instant::now();

    let cost_model = Model::default();

    let trials = args.trials.get() as usize;

    let results: Vec<_> = if args.quiet {
        iter::repeatn(layout, trials)
            .map(|l| {
                optimise(
                    &cost_model,
                    args.iterations,
                    args.cooling_rate,
                    args.temp_scale,
                    l.clone(),
                    corpus,
                    |_i| {},
                )
            })
            .collect()
    } else {
        // let multiprog = MultiProgress::new();
        // multiprog.set_move_cursor(true);

        // let bars: Vec<_> =
        //     std::iter::repeat_with(|| multiprog.add(ProgressBar::new(args.iterations.into())))
        //         .take(trials)
        //         .collect();

        // for bar in bars.iter() {
        //     bar.set_style(ProgressStyle::with_template("{wide_bar} {percent:>3}%").unwrap());
        //     bar.set_position(0);
        // }

        // rayon::join(
        //     || multiprog.clear().unwrap(),
        //     || {
        //         iter::repeatn(layout, trials)
        //             .zip(bars)
        //             .map(|(l, bar)| {
        //                 let res = optimise(
        //                     &cost_model,
        //                     args.iterations,
        //                     args.cooling_rate,
        //                     args.temp_scale,
        //                     l.clone(),
        //                     corpus,
        //                     |i| bar.set_position(i.into()),
        //                 );
        //                 bar.finish();
        //                 res
        //             })
        //             .collect()
        //     },
        // )
        // .1

        let bar = ProgressBar::new(args.iterations as u64 * trials as u64)
            .with_style(ProgressStyle::with_template("{percent:>3}% {wide_bar} {eta:>3}").unwrap());

        let res = iter::repeatn(layout, trials)
            .map(|l| {
                optimise(
                    &cost_model,
                    args.iterations,
                    args.cooling_rate,
                    args.temp_scale,
                    l.clone(),
                    corpus,
                    |_i| bar.inc(1),
                )
            })
            .collect();

        bar.finish_and_clear();

        res
    };

    // let mut bar = ProgressBar::new(n.into());
    // let results = vec![optimise(n, layout.clone(), corpus, |i| {
    //     bar.set_position(i.into())
    // })];
    // bar.finish();

    // let results = vec![optimise(n, layout.clone(), corpus, |_i| {})];

    let mean = results.iter().map(|(_, i)| i).sum::<f64>() / trials as f64;
    let var = results.iter().map(|(_, i)| (i - mean).powi(2)).sum::<f64>() / trials as f64;
    let std_dev = var.sqrt();

    let mad = results
        .iter()
        .flat_map(|(a, _)| results.iter().map(|(b, _)| a.hamming_dist(b) as u32))
        .sum::<u32>() as f64
        / (trials * trials) as f64;

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
