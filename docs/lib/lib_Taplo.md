# lib.Taplo

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-cli\src\lib.rs` (lines 17-23)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Taplo<E: Environment> {
    env: E,
    colors: bool,
    #[cfg(feature = "lint")]
    schemas: Schemas<E>,
    config: Option<Arc<Config>>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Taplo<E: Environment> {
    env: E,
    colors: bool,
    #[cfg(feature = "lint")]
    schemas: Schemas<E>,
    config: Option<Arc<Config>>,
}
```

## Git History

- Commits: 10
- Authors: 4
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
