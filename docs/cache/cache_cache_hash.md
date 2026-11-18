# cache.cache_hash

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 140-144)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn cache_hash(url: &Url) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_str().as_bytes());
    hex::encode(&hasher.finalize()[..])
}
```

## Docstring

_No docstring provided._

## Source

```
fn cache_hash(url: &Url) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_str().as_bytes());
    hex::encode(&hasher.finalize()[..])
}
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
