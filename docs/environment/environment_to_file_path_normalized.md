# environment.to_file_path_normalized

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment.rs` (lines 75-77)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn to_file_path_normalized(&self, url: &Url) -> Option<PathBuf> {
        self.to_file_path(url).map(Normalize::normalize)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn to_file_path_normalized(&self, url: &Url) -> Option<PathBuf> {
        self.to_file_path(url).map(Normalize::normalize)
    }
```

## Git History

- Commits: 4
- Authors: 1
- Bug fixes: 1
- Features: 3

## Callers

None

## Callees

None
