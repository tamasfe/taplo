# mod.cleaned_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 376-384)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn cleaned_key(&self) -> &Vec<String> {
        self.cleaned_key.get_or_init(|| {
            self.key
                .replace(['\'', '"'], "")
                .split('.')
                .map(ToOwned::to_owned)
                .collect()
        })
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn cleaned_key(&self) -> &Vec<String> {
        self.cleaned_key.get_or_init(|| {
            self.key
                .replace(['\'', '"'], "")
                .split('.')
                .map(ToOwned::to_owned)
                .collect()
        })
    }
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
