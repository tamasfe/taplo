# native.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 16-20)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new() -> Self {
        Self {
            handle: tokio::runtime::Handle::current(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new() -> Self {
        Self {
            handle: tokio::runtime::Handle::current(),
        }
    }
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
