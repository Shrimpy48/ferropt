use std::fs::File;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use ferropt::cost::*;
use ferropt::layout::{read_named_corpus, AnnotatedLayout, Layout};

fn heuristic(c: &mut Criterion) {
    let model = heuristic::Model::default();
    let mut group = c.benchmark_group("heuristic");

    for layout_path in ["qwerty.json", "initial.json", "optimised_noshifted.json"] {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let ann_layout: AnnotatedLayout = layout.into();

        for (path, string) in read_named_corpus("corpus").unwrap().into_iter().step_by(25) {
            group.throughput(Throughput::Bytes(string.len() as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{}", layout_path, path.display())),
                &[string],
                |b, c| b.iter(|| model.cost(c, &ann_layout)),
            );
        }
    }
    group.finish();
}

fn simple(c: &mut Criterion) {
    let model = simple::Model::default();
    let mut group = c.benchmark_group("simple");

    for layout_path in ["qwerty.json", "initial.json", "optimised_noshifted.json"] {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let ann_layout: AnnotatedLayout = layout.into();

        for (path, string) in read_named_corpus("corpus").unwrap().into_iter().step_by(25) {
            group.throughput(Throughput::Bytes(string.len() as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{}", layout_path, path.display())),
                &[string],
                |b, c| b.iter(|| model.cost(c, &ann_layout)),
            );
        }
    }
    group.finish();
}

fn measured(c: &mut Criterion) {
    let model = measured::Model::default();
    let mut group = c.benchmark_group("measured");

    for layout_path in ["qwerty.json", "initial.json", "optimised_noshifted.json"] {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let ann_layout: AnnotatedLayout = layout.into();

        for (path, string) in read_named_corpus("corpus").unwrap().into_iter().step_by(25) {
            group.throughput(Throughput::Bytes(string.len() as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{}", layout_path, path.display())),
                &[string],
                |b, c| b.iter(|| model.cost(c, &ann_layout)),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, heuristic, simple, measured);
criterion_main!(benches);
