use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vector_alloc_test::vector_addition;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("intial: 10, fanal 9", |b| b.iter(|| vector_addition(black_box(10), black_box(9))));
    c.bench_function("intial: 100, fanal 9", |b| b.iter(|| vector_addition(black_box(100), black_box(9))));
    c.bench_function("intial: 1000, fanal 9", |b| b.iter(|| vector_addition(black_box(1000), black_box(9))));
    c.bench_function("intial: 10000, fanal 9", |b| b.iter(|| vector_addition(black_box(10000), black_box(9))));
    c.bench_function("intial: 10, fanal 99", |b| b.iter(|| vector_addition(black_box(10), black_box(99))));
    c.bench_function("intial: 100, fanal 99", |b| b.iter(|| vector_addition(black_box(100), black_box(99))));
    c.bench_function("intial: 1000, fanal 99", |b| b.iter(|| vector_addition(black_box(1000), black_box(99))));
    c.bench_function("intial: 10000, fanal 99", |b| b.iter(|| vector_addition(black_box(10000), black_box(99))));
    c.bench_function("intial: 10, fanal 999", |b| b.iter(|| vector_addition(black_box(10), black_box(999))));
    c.bench_function("intial: 100, fanal 999", |b| b.iter(|| vector_addition(black_box(100), black_box(999))));
    c.bench_function("intial: 1000, fanal 999", |b| b.iter(|| vector_addition(black_box(1000), black_box(999))));
    c.bench_function("intial: 10000, fanal 999", |b| b.iter(|| vector_addition(black_box(10000), black_box(999))));
    c.bench_function("intial: 10, fanal 9999", |b| b.iter(|| vector_addition(black_box(10), black_box(9999))));
    c.bench_function("intial: 100, fanal 9999", |b| b.iter(|| vector_addition(black_box(100), black_box(9999))));
    c.bench_function("intial: 1000, fanal 9999", |b| b.iter(|| vector_addition(black_box(1000), black_box(9999))));
    c.bench_function("intial: 10000, fanal 9999", |b| b.iter(|| vector_addition(black_box(10000), black_box(9999))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);