# from_syntax.from_syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\from_syntax.rs` (lines 374-376)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from_syntax(syntax: SyntaxElement) -> Self {
        Keys::new(keys_from_syntax(&syntax).map(Into::into))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from_syntax(syntax: SyntaxElement) -> Self {
        Keys::new(keys_from_syntax(&syntax).map(Into::into))
    }
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
