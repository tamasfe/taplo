# mod.semver

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 742-744)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(super) fn semver(value: &str) -> bool {
        semver::Version::parse(value).is_ok()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(super) fn semver(value: &str) -> bool {
        semver::Version::parse(value).is_ok()
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
