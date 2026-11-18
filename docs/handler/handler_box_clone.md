# handler.box_clone

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\handler.rs` (lines 181-183)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn box_clone(&self) -> Box<dyn Handler<W>> {
        Box::new((*self).clone())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn box_clone(&self) -> Box<dyn Handler<W>> {
        Box::new((*self).clone())
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
