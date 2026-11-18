# semantic_tokens.SemanticTokensBuilder

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\semantic_tokens.rs` (lines 114-118)
- **Role:** internal
- **Stability:** unknown

## Signature

```
struct SemanticTokensBuilder<'b> {
    tokens: Vec<SemanticToken>,
    mapper: &'b Mapper,
    last_range: Option<Range>,
}
```

## Docstring

_No docstring provided._

## Source

```
struct SemanticTokensBuilder<'b> {
    tokens: Vec<SemanticToken>,
    mapper: &'b Mapper,
    last_range: Option<Range>,
}
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
