# mod.format_with_scopes

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 275-300)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn format_with_scopes(
    dom: Node,
    options: Options,
    errors: &[TextRange],
    scopes: ScopedOptions,
) -> String {
    let c = Context {
        scopes: Rc::new(scopes),
        errors: errors.into(),
        ..Context::default()
    };

    let mut s = format_impl(
        dom.syntax().unwrap().clone().into_node().unwrap(),
        options.clone(),
        c,
    );

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    s
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn format_with_scopes(
    dom: Node,
    options: Options,
    errors: &[TextRange],
    scopes: ScopedOptions,
) -> String {
    let c = Context {
        scopes: Rc::new(scopes),
        errors: errors.into(),
        ..Context::default()
    };

    let mut s = format_impl(
        dom.syntax().unwrap().clone().into_node().unwrap(),
        options.clone(),
        c,
    );

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    s
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
