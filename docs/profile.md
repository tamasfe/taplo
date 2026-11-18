# Module: profile

**File:** `\crates\taplo\benches\profile.rs`

## Overview

Module `profile` contains 2 analyzed entities.

```
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use taplo::parser::parse;

#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};

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

#[cfg(unix)]
criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = syntax, dom
);

#[cfg(not(unix))]
criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = syntax, dom
);

criterion_main!(benches);
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `profile.syntax` | function | No description |
| `profile.dom` | function | No description |

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
