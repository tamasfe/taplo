# config.Config

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 23-60)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Config {
    /// Files to include.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// Omitting this property includes all files, **however an empty array will include none**.
    pub include: Option<Vec<String>>,

    /// Files to exclude (ignore).
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// This has priority over `include`.
    pub exclude: Option<Vec<String>>,

    /// Rules are used to override configurations by path and keys.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<Rule>,

    #[serde(flatten)]
    pub global_options: Options,

    #[serde(skip)]
    pub file_rule: Option<GlobRule>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<HashMap<String, Plugin>>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Config {
    /// Files to include.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// Omitting this property includes all files, **however an empty array will include none**.
    pub include: Option<Vec<String>>,

    /// Files to exclude (ignore).
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// This has priority over `include`.
    pub exclude: Option<Vec<String>>,

    /// Rules are used to override configurations by path and keys.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<Rule>,

    #[serde(flatten)]
    pub global_options: Options,

    #[serde(skip)]
    pub file_rule: Option<GlobRule>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<HashMap<String, Plugin>>,
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
