use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use taplo::parser::parse;

pub fn syntax(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    c.bench_function("parse-toml", |b| b.iter(|| parse(black_box(source))));
}

pub fn dom(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    c.bench_function("toml-dom", |b| {
        b.iter(|| parse(black_box(source)).into_dom())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = syntax, dom
);
criterion_main!(benches);
