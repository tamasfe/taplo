# environment.stdout

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 277-279)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn stdout(&self) -> Self::Stdout {
        JsAsyncWrite::new(self.js_on_stdout.clone())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn stdout(&self) -> Self::Stdout {
        JsAsyncWrite::new(self.js_on_stdout.clone())
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
