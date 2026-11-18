# mod.dangling_newlines

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 1322-1334)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn dangling_newlines(t: SyntaxToken) -> Option<usize> {
    let newline_count = t.text().newline_count();

    if let Some(nt) = t.next_sibling_or_token() {
        if let Some(nnt) = nt.next_sibling_or_token() {
            if nt.kind() == WHITESPACE && nnt.kind() == NEWLINE {
                return Some(newline_count);
            }
        }
    }

    None
}
```

## Docstring

_No docstring provided._

## Source

```
fn dangling_newlines(t: SyntaxToken) -> Option<usize> {
    let newline_count = t.text().newline_count();

    if let Some(nt) = t.next_sibling_or_token() {
        if let Some(nnt) = nt.next_sibling_or_token() {
            if nt.kind() == WHITESPACE && nnt.kind() == NEWLINE {
                return Some(newline_count);
            }
        }
    }

    None
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
