use clc_engine::Calculator;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_line 20", |b| {
        b.iter(|| Calculator::new().calculate_line(black_box("(10 + 20) * 30 / 5")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
