# associations.is_match

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 415-433)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_match(&self, url: &Url) -> bool {
        match self {
            // Glob associations typically come from config files
            // with a glob pattern that is an absolute file path
            // without a scheme.
            //
            // So in order to be a match, we need to
            // strip the scheme from the URL.
            AssociationRule::Glob(g) => g.is_match(&*normalize_str(
                url.as_str()
                    .strip_prefix(url.scheme())
                    .unwrap()
                    .strip_prefix("://")
                    .unwrap(),
            )),
            AssociationRule::Regex(r) => r.is_match(&normalize_str(url.as_str())),
            AssociationRule::Url(u) => u == url,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_match(&self, url: &Url) -> bool {
        match self {
            // Glob associations typically come from config files
            // with a glob pattern that is an absolute file path
            // without a scheme.
            //
            // So in order to be a match, we need to
            // strip the scheme from the URL.
            AssociationRule::Glob(g) => g.is_match(&*normalize_str(
                url.as_str()
                    .strip_prefix(url.scheme())
                    .unwrap()
                    .strip_prefix("://")
                    .unwrap(),
            )),
            AssociationRule::Regex(r) => r.is_match(&normalize_str(url.as_str())),
            AssociationRule::Url(u) => u == url,
        }
    }
```

## Git History

- Commits: 16
- Authors: 5
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
