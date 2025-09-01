use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracing::render;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let profile = &String::from("release");
    let scene = &String::from("triangle");
    c.bench_function("release triangle", |b| {
        b.iter(|| render(black_box(profile), black_box(scene)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
