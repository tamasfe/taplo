# native.to_file_path

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 98-100)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn to_file_path(&self, url: &url::Url) -> Option<std::path::PathBuf> {
        url.to_file_path().ok()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn to_file_path(&self, url: &url::Url) -> Option<std::path::PathBuf> {
        url.to_file_path().ok()
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
