# util.normalize_str

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\util.rs` (lines 111-121)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn normalize_str(s: &str) -> Cow<'_, str> {
    let Some(percent_decoded) = percent_decode_str(s).decode_utf8().ok() else {
        return s.into();
    };

    if cfg!(windows) {
        percent_decoded.replace('\\', "/").into()
    } else {
        percent_decoded
    }
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) fn normalize_str(s: &str) -> Cow<'_, str> {
    let Some(percent_decoded) = percent_decode_str(s).decode_utf8().ok() else {
        return s.into();
    };

    if cfg!(windows) {
        percent_decoded.replace('\\', "/").into()
    } else {
        percent_decoded
    }
}
```

## Git History

- Commits: 11
- Authors: 4
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
