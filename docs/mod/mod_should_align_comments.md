# mod.should_align_comments

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 193-195)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn should_align_comments(&self, comment_count: usize) -> bool {
        (comment_count != 1 || self.align_single_comments) && self.align_comments
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn should_align_comments(&self, comment_count: usize) -> bool {
        (comment_count != 1 || self.align_single_comments) && self.align_comments
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
