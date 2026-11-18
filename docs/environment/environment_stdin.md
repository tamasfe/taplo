# environment.stdin

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 273-275)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn stdin(&self) -> Self::Stdin {
        JsAsyncRead::new(self.js_on_stdin.clone())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn stdin(&self) -> Self::Stdin {
        JsAsyncRead::new(self.js_on_stdin.clone())
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
