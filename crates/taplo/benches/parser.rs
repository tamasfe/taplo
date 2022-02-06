use criterion::{black_box, criterion_group, criterion_main, Criterion};
use taplo::parser::parse;

pub fn criterion_benchmark(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    c.bench_function("parse taplo syntax", |b| {
        b.iter(|| parse(black_box(source)))
    });
    c.bench_function("parse taplo dom", |b| {
        b.iter(|| parse(black_box(source)).into_dom())
    });
    c.bench_function("parse taplo dom and validate", |b| {
        b.iter(|| parse(black_box(source)).into_dom().validate())
    });
    c.bench_function("parse toml", |b| {
        b.iter(|| toml::from_str::<toml::Value>(black_box(source)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
