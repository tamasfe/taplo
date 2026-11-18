# native.now

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 35-37)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn now(&self) -> time::OffsetDateTime {
        OffsetDateTime::now_utc()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn now(&self) -> time::OffsetDateTime {
        OffsetDateTime::now_utc()
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
