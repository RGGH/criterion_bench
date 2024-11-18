use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion_bench::{euler1_par, euler1_series, euler1_simple};

fn criterion_benchmark(c: &mut Criterion) {
    let input = 1000000;
    c.bench_function("simple", |b| b.iter(|| euler1_simple(black_box(input))));
    c.bench_function("parallel", |b| b.iter(|| euler1_par(black_box(input))));
    c.bench_function("series", |b| b.iter(|| euler1_series(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
