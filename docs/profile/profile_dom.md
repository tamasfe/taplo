# profile.dom

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\benches\profile.rs` (lines 12-17)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn dom(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    c.bench_function("toml-dom", |b| {
        b.iter(|| parse(black_box(source)).into_dom())
    });
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn dom(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    c.bench_function("toml-dom", |b| {
        b.iter(|| parse(black_box(source)).into_dom())
    });
}
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
