# util.get_certs

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\util.rs` (lines 157-163)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn get_certs(
        builder: reqwest::ClientBuilder,
        path: std::ffi::OsString,
    ) -> reqwest::ClientBuilder {
        tracing::error!(?path, "Could not load certs, taplo was built without TLS");
        builder
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn get_certs(
        builder: reqwest::ClientBuilder,
        path: std::ffi::OsString,
    ) -> reqwest::ClientBuilder {
        tracing::error!(?path, "Could not load certs, taplo was built without TLS");
        builder
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
