# mod.FormattedItem

- **Type:** trait
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 1215-1220)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
trait FormattedItem {
    fn syntax(&self) -> SyntaxElement;
    #[allow(clippy::ptr_arg)]
    fn write_to(&self, formatted: &mut String, options: &Options);
    fn trailing_comment(&self) -> Option<String>;
}
```

## Docstring

_No docstring provided._

## Source

```
trait FormattedItem {
    fn syntax(&self) -> SyntaxElement;
    #[allow(clippy::ptr_arg)]
    fn write_to(&self, formatted: &mut String, options: &Options);
    fn trailing_comment(&self) -> Option<String>;
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
