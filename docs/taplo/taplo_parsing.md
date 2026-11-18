# taplo.parsing

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\benches\taplo.rs` (lines 8-22)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
