use criterion::{black_box, criterion_group, criterion_main, Criterion};
use taplo::{
    dom::Node,
    formatter::{format, format_syntax, Options},
    parser::parse,
};

pub fn parsing(c: &mut Criterion) {
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
    c.bench_function("parse toml-rs", |b| {
        b.iter(|| toml::from_str::<toml::Value>(black_box(source)))
    });
}

pub fn formatting(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");

    let syntax = parse(source).into_syntax();
    c.bench_function("format syntax", |b| {
        b.iter(|| format_syntax(black_box(syntax.clone()), Options::default()))
    });
    c.bench_function("parse and format", |b| {
        b.iter(|| format(black_box(source), Options::default()))
    });
}

pub fn conversion(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    let v: serde_json::Value = toml::from_str(source).unwrap();

    c.bench_function("convert from JSON", |b| {
        b.iter(|| {
            serde_json::from_value::<Node>(black_box(v.clone()))
                .unwrap()
                .to_toml(false, false)
        })
    });
}

criterion_group!(benches, parsing, formatting, conversion);
criterion_main!(benches);
