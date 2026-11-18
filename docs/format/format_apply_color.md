# format.apply_color

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\format.rs` (lines 132-139)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn apply_color<'a>(
                s: &'a [&'a str],
                prefix: &'a str,
                color: Colour,
            ) -> impl IntoIterator<Item = String> + 'a {
                s.iter()
                    .map(move |&s| color.paint(prefix.to_owned() + s).to_string())
            }
```

## Docstring

_No docstring provided._

## Source

```
            fn apply_color<'a>(
                s: &'a [&'a str],
                prefix: &'a str,
                color: Colour,
            ) -> impl IntoIterator<Item = String> + 'a {
                s.iter()
                    .map(move |&s| color.paint(prefix.to_owned() + s).to_string())
            }
```

## Git History

- Commits: 19
- Authors: 6
- Bug fixes: 6
- Features: 2

## Callers

None

## Callees

None
