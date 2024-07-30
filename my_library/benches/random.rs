use criterion::{criterion_group, criterion_main, Criterion};
use my_library::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    // My benchmarks go here
    c.bench_function("random", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            rng.range(1.0_f32..10_000_000_f32);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);