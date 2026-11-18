# mod.Parser

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 50-75)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct Parser<'p> {
    skip_whitespace: bool,
    // Allow glob patterns as keys and using [] instead of dots.
    key_pattern_syntax: bool,
    current_token: Option<SyntaxKind>,

    // These tokens are not consumed on errors.
    //
    // The syntax error is still reported,
    // but the the surrounding context can still
    // be parsed.
    // FIXME(bit_flags):
    //      This is VERY wrong, as the members of the
    //      enums are not proper bit flags.
    //
    //      However this incorrect behavior marks fewer tokens
    //      as errors making the parser more fault-tolerant.
    //      Instead of fixing this it would probably be better to
    //      remove the ERROR token altogether, or reserving it for
    //      special cases.
    error_whitelist: u16,

    lexer: Lexer<'p, SyntaxKind>,
    builder: GreenNodeBuilder<'p>,
    errors: Vec<Error>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct Parser<'p> {
    skip_whitespace: bool,
    // Allow glob patterns as keys and using [] instead of dots.
    key_pattern_syntax: bool,
    current_token: Option<SyntaxKind>,

    // These tokens are not consumed on errors.
    //
    // The syntax error is still reported,
    // but the the surrounding context can still
    // be parsed.
    // FIXME(bit_flags):
    //      This is VERY wrong, as the members of the
    //      enums are not proper bit flags.
    //
    //      However this incorrect behavior marks fewer tokens
    //      as errors making the parser more fault-tolerant.
    //      Instead of fixing this it would probably be better to
    //      remove the ERROR token altogether, or reserving it for
    //      special cases.
    error_whitelist: u16,

    lexer: Lexer<'p, SyntaxKind>,
    builder: GreenNodeBuilder<'p>,
    errors: Vec<Error>,
}
```

## Git History

- Commits: 8
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
