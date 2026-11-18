# config.Rule

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 273-317)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Rule {
    /// The name of the rule.
    ///
    /// Used in `taplo::<name>` comments.
    pub name: Option<String>,

    /// Files this rule is valid for.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// Omitting this property includes all files, **however an empty array will include none**.
    pub include: Option<Vec<String>>,

    /// Files that are excluded from this rule.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// This has priority over `include`.
    pub exclude: Option<Vec<String>>,

    /// Keys the rule is valid for in a document.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) dotted key patterns.
    ///
    /// This allows enabling the rule for specific paths in the document.
    ///
    /// For example:
    ///
    /// - `package.metadata` will enable the rule for everything inside the `package.metadata` table, including itself.
    ///
    /// If omitted, the rule will always be valid for all keys.
    pub keys: Option<Vec<String>>,

    #[serde(flatten)]
    pub options: Options,

    #[serde(skip)]
    pub file_rule: Option<GlobRule>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Rule {
    /// The name of the rule.
    ///
    /// Used in `taplo::<name>` comments.
    pub name: Option<String>,

    /// Files this rule is valid for.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// Omitting this property includes all files, **however an empty array will include none**.
    pub include: Option<Vec<String>>,

    /// Files that are excluded from this rule.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// This has priority over `include`.
    pub exclude: Option<Vec<String>>,

    /// Keys the rule is valid for in a document.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) dotted key patterns.
    ///
    /// This allows enabling the rule for specific paths in the document.
    ///
    /// For example:
    ///
    /// - `package.metadata` will enable the rule for everything inside the `package.metadata` table, including itself.
    ///
    /// If omitted, the rule will always be valid for all keys.
    pub keys: Option<Vec<String>>,

    #[serde(flatten)]
    pub options: Options,

    #[serde(skip)]
    pub file_rule: Option<GlobRule>,
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
