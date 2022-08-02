use std::fs::File;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use ferropt::{
    evolve::{keys, lookahead, oneshot, read_named_corpus, AnnotatedLayout},
    layout::Layout,
};

fn keys_bench(c: &mut Criterion) {
    let f = File::open("initial_rnum.json").unwrap();
    let layout: Layout = serde_json::from_reader(f).unwrap();
    let ann_layout: AnnotatedLayout = layout.into();

    let mut group = c.benchmark_group("keys");

    for (path, string) in read_named_corpus().unwrap().into_iter().step_by(25) {
        group.throughput(Throughput::Bytes(string.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(path.display()),
            &string,
            |b, s| b.iter(|| keys(&ann_layout, s.iter().copied()).for_each(|_| {})),
        );
    }
    group.finish();
}

fn oneshot_bench(c: &mut Criterion) {
    let f = File::open("initial_rnum.json").unwrap();
    let layout: Layout = serde_json::from_reader(f).unwrap();
    let ann_layout: AnnotatedLayout = layout.into();

    let mut group = c.benchmark_group("oneshot");
    for (path, string) in read_named_corpus().unwrap().into_iter().step_by(25) {
        let events: Vec<_> = keys(&ann_layout, string.iter().copied()).collect();
        group.throughput(Throughput::Elements(events.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("oneshot", path.display()),
            &events,
            |b, e| b.iter(|| oneshot(lookahead(e.iter().copied())).for_each(|_| {})),
        );
    }
    group.finish();
}

fn combined_bench(c: &mut Criterion) {
    let f = File::open("initial_rnum.json").unwrap();
    let layout: Layout = serde_json::from_reader(f).unwrap();
    let ann_layout: AnnotatedLayout = layout.into();

    let mut group = c.benchmark_group("combined");
    for (path, string) in read_named_corpus().unwrap().into_iter().step_by(25) {
        group.throughput(Throughput::Bytes(string.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("combined", path.display()),
            &string,
            |b, s| b.iter(|| oneshot(keys(&ann_layout, s.iter().copied())).for_each(|_| {})),
        );
    }
    group.finish();
}

criterion_group!(benches, keys_bench, oneshot_bench, combined_bench);
criterion_main!(benches);
