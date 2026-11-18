# mod.format_with_path_scopes

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 305-346)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn format_with_path_scopes<I, S>(
    dom: Node,
    options: Options,
    errors: &[TextRange],
    scopes: I,
) -> Result<String, dom::Error>
where
    I: IntoIterator<Item = (S, OptionsIncomplete)>,
    S: AsRef<str>,
{
    let mut c = Context {
        errors: errors.into(),
        ..Context::default()
    };

    let mut s = Vec::new();

    for (scope, opts) in scopes {
        let keys: Keys = scope.as_ref().parse()?;
        let matched = dom.find_all_matches(keys, false)?;

        for (_, node) in matched {
            s.extend(node.text_ranges(true).map(|r| (r, opts.clone())));
        }
    }

    c.scopes = Rc::new(ScopedOptions::from_iter(s));

    let mut s = format_impl(
        dom.syntax().unwrap().clone().into_node().unwrap(),
        options.clone(),
        c,
    );

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    Ok(s)
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn format_with_path_scopes<I, S>(
    dom: Node,
    options: Options,
    errors: &[TextRange],
    scopes: I,
) -> Result<String, dom::Error>
where
    I: IntoIterator<Item = (S, OptionsIncomplete)>,
    S: AsRef<str>,
{
    let mut c = Context {
        errors: errors.into(),
        ..Context::default()
    };

    let mut s = Vec::new();

    for (scope, opts) in scopes {
        let keys: Keys = scope.as_ref().parse()?;
        let matched = dom.find_all_matches(keys, false)?;

        for (_, node) in matched {
            s.extend(node.text_ranges(true).map(|r| (r, opts.clone())));
        }
    }

    c.scopes = Rc::new(ScopedOptions::from_iter(s));

    let mut s = format_impl(
        dom.syntax().unwrap().clone().into_node().unwrap(),
        options.clone(),
        c,
    );

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    Ok(s)
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
