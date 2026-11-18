# taplo.conversion

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\benches\taplo.rs` (lines 36-47)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
