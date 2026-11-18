# native.atty_stderr

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 62-65)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn atty_stderr(&self) -> bool {
        use std::io::IsTerminal;
        std::io::stderr().is_terminal()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn atty_stderr(&self) -> bool {
        use std::io::IsTerminal;
        std::io::stderr().is_terminal()
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
