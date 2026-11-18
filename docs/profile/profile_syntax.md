# profile.syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\benches\profile.rs` (lines 7-10)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn syntax(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    c.bench_function("parse-toml", |b| b.iter(|| parse(black_box(source))));
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn syntax(c: &mut Criterion) {
    let source = include_str!("../../../test-data/example.toml");
    c.bench_function("parse-toml", |b| b.iter(|| parse(black_box(source))));
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
