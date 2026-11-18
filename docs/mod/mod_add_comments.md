# mod.add_comments

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 447-464)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn add_comments(
        comments: &mut Vec<String>,
        formatted: &mut String,
        context: &Context,
        options: &Options,
    ) -> bool {
        let were_comments = !comments.is_empty();

        for (idx, comment) in comments.drain(0..).enumerate() {
            if idx != 0 {
                *formatted += options.newline();
            }
            formatted.extend(context.indent(options));
            *formatted += &comment;
        }

        were_comments
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn add_comments(
        comments: &mut Vec<String>,
        formatted: &mut String,
        context: &Context,
        options: &Options,
    ) -> bool {
        let were_comments = !comments.is_empty();

        for (idx, comment) in comments.drain(0..).enumerate() {
            if idx != 0 {
                *formatted += options.newline();
            }
            formatted.extend(context.indent(options));
            *formatted += &comment;
        }

        were_comments
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
