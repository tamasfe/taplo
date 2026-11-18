# mod.Context

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 199-204)
- **Role:** internal
- **Stability:** unknown

## Signature

```
struct Context {
    indent_level: usize,
    force_multiline: bool,
    errors: Rc<[TextRange]>,
    scopes: Rc<ScopedOptions>,
}
```

## Docstring

_No docstring provided._

## Source

```
struct Context {
    indent_level: usize,
    force_multiline: bool,
    errors: Rc<[TextRange]>,
    scopes: Rc<ScopedOptions>,
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
