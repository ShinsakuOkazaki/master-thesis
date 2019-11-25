use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vector_alloc_test::vector_addition;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("intial_10_fanal_9", |b| b.iter(|| vector_addition(black_box(10), black_box(9))));
    c.bench_function("intial_100_fanal_9", |b| b.iter(|| vector_addition(black_box(100), black_box(9))));
    c.bench_function("intial_1000_fanal_9", |b| b.iter(|| vector_addition(black_box(1000), black_box(9))));
    c.bench_function("intial_10000_fanal_9", |b| b.iter(|| vector_addition(black_box(10000), black_box(9))));
    c.bench_function("intial_10_fanal_99", |b| b.iter(|| vector_addition(black_box(10), black_box(99))));
    c.bench_function("intial_100_fanal_99", |b| b.iter(|| vector_addition(black_box(100), black_box(99))));
    c.bench_function("intial_1000_fanal_99", |b| b.iter(|| vector_addition(black_box(1000), black_box(99))));
    c.bench_function("intial_10000_fanal_99", |b| b.iter(|| vector_addition(black_box(10000), black_box(99))));
    c.bench_function("intial_10_fanal_999", |b| b.iter(|| vector_addition(black_box(10), black_box(999))));
    c.bench_function("intial_100_fanal_999", |b| b.iter(|| vector_addition(black_box(100), black_box(999))));
    c.bench_function("intial_1000_fanal_999", |b| b.iter(|| vector_addition(black_box(1000), black_box(999))));
    c.bench_function("intial_10000_fanal_999", |b| b.iter(|| vector_addition(black_box(10000), black_box(999))));
    c.bench_function("intial_10_fanal_9999", |b| b.iter(|| vector_addition(black_box(10), black_box(9999))));
    c.bench_function("intial_100_fanal_9999", |b| b.iter(|| vector_addition(black_box(100), black_box(9999))));
    c.bench_function("intial_1000_fanal_9999", |b| b.iter(|| vector_addition(black_box(1000), black_box(9999))));
    c.bench_function("intial_10000_fanal_9999", |b| b.iter(|| vector_addition(black_box(10000), black_box(9999))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);