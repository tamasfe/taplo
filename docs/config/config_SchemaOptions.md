# config.SchemaOptions

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 358-375)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct SchemaOptions {
    /// Whether the schema should be enabled or not.
    ///
    /// Defaults to true if omitted.
    pub enabled: Option<bool>,

    /// A local file path to the schema, overrides `url` if set.
    ///
    /// URLs are also accepted here, but it's not a guarantee and might
    /// change in newer releases.
    /// Please use the `url` field instead whenever possible.
    pub path: Option<String>,

    /// A full absolute URL to the schema.
    ///
    /// The url of the schema, supported schemes are `http`, `https`, `file` and `taplo`.
    pub url: Option<Url>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct SchemaOptions {
    /// Whether the schema should be enabled or not.
    ///
    /// Defaults to true if omitted.
    pub enabled: Option<bool>,

    /// A local file path to the schema, overrides `url` if set.
    ///
    /// URLs are also accepted here, but it's not a guarantee and might
    /// change in newer releases.
    /// Please use the `url` field instead whenever possible.
    pub path: Option<String>,

    /// A full absolute URL to the schema.
    ///
    /// The url of the schema, supported schemes are `http`, `https`, `file` and `taplo`.
    pub url: Option<Url>,
}
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 4
- Features: 1

## Callers

None

## Callees

None
