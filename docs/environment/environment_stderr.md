# environment.stderr

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 281-283)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn stderr(&self) -> Self::Stderr {
        JsAsyncWrite::new(self.js_on_stderr.clone())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn stderr(&self) -> Self::Stderr {
        JsAsyncWrite::new(self.js_on_stderr.clone())
    }
```

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 3
- Features: 2

## Callers

None

## Callees

None
