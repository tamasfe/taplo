# mod.reference_url

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 627-634)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn reference_url(root_url: &Url, reference: &str) -> Option<Url> {
    if !reference.starts_with('#') {
        return Url::parse(reference).ok();
    }
    let mut url = root_url.clone();
    url.set_fragment(Some(reference.trim_start_matches("#/")));
    Some(url)
}
```

## Docstring

_No docstring provided._

## Source

```
fn reference_url(root_url: &Url, reference: &str) -> Option<Url> {
    if !reference.starts_with('#') {
        return Url::parse(reference).ok();
    }
    let mut url = root_url.clone();
    url.set_fragment(Some(reference.trim_start_matches("#/")));
    Some(url)
}
```

## Git History

- Commits: 13
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
