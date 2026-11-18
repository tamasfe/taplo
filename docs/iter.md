# Module: iter

**File:** `\crates\taplo\src\util\iter.rs`

## Overview

Module `iter` contains 6 analyzed entities.

```
pub(crate) struct ExactIter<I>
where
    I: Iterator,
{
    len: usize,
    iter: I,
}

impl<I> ExactIter<I>
where
    I: Iterator,
{
    pub(crate) fn new(len: usize, iter: I) -> Self {
        Self { len, iter }
    }
}

impl<I> Iterator for ExactIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len != 0 {
            let v = self.iter.next().expect("exact iterator ended too early");
            self.len -= 1;
            Some(v)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I> ExactSizeIterator for ExactIter<I> where I: Iterator {}

pub(crate) trait ExactIterExt: Iterator + Sized {
    fn exactly(self, count: usize) -> ExactIter<Self>;
}

impl<I: Iterator + Sized> ExactIterExt for I {
    fn exactly(self, count: usize) -> ExactIter<Self> {
        ExactIter::new(count, self)
    }
}
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `iter.new` | function | No description |
| `iter.exactly` | function | No description |
| `iter.size_hint` | function | No description |
| `iter.next` | function | No description |
| `iter.ExactIter` | struct | No description |
| `iter.ExactIterExt` | trait | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
