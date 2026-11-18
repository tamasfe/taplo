# nodes.text_ranges

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 226-237)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn text_ranges(&self) -> impl ExactSizeIterator<Item = TextRange> {
        let additional_syntaxes = self.inner.additional_syntaxes.read();

        let mut ranges = Vec::with_capacity(1 + additional_syntaxes.len());
        if let Some(s) = self.syntax() {
            ranges.push(s.text_range());
        }

        ranges.extend(additional_syntaxes.iter().map(|s| s.text_range()));

        ranges.into_iter()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn text_ranges(&self) -> impl ExactSizeIterator<Item = TextRange> {
        let additional_syntaxes = self.inner.additional_syntaxes.read();

        let mut ranges = Vec::with_capacity(1 + additional_syntaxes.len());
        if let Some(s) = self.syntax() {
            ranges.push(s.text_range());
        }

        ranges.extend(additional_syntaxes.iter().map(|s| s.text_range()));

        ranges.into_iter()
    }
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
