# taplo.formatting

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\benches\taplo.rs` (lines 24-34)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
