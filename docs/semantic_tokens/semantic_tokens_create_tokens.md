# semantic_tokens.create_tokens

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\semantic_tokens.rs` (lines 76-112)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn create_tokens(syntax: &SyntaxNode, mapper: &Mapper) -> Vec<SemanticToken> {
    let mut builder = SemanticTokensBuilder::new(mapper);

    for element in syntax.descendants_with_tokens() {
        match element {
            SyntaxElement::Node(_node) => {}
            SyntaxElement::Token(token) => {
                if let IDENT = token.kind() {
                    // look for an inline table value
                    let is_table_key = token
                        .parent()
                        .and_then(|p| p.next_sibling())
                        .and_then(|t| t.first_child())
                        .is_some_and(|t| t.kind() == INLINE_TABLE);

                    if is_table_key {
                        builder.add_token(&token, TokenType::TomlTableKey, &[]);
                        continue;
                    }

                    // look for an array
                    let is_array_key = token
                        .parent()
                        .and_then(|p| p.next_sibling())
                        .and_then(|t| t.first_child())
                        .is_some_and(|t| t.kind() == ARRAY);

                    if is_array_key {
                        builder.add_token(&token, TokenType::TomlArrayKey, &[]);
                    }
                }
            }
        }
    }

    builder.build()
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn create_tokens(syntax: &SyntaxNode, mapper: &Mapper) -> Vec<SemanticToken> {
    let mut builder = SemanticTokensBuilder::new(mapper);

    for element in syntax.descendants_with_tokens() {
        match element {
            SyntaxElement::Node(_node) => {}
            SyntaxElement::Token(token) => {
                if let IDENT = token.kind() {
                    // look for an inline table value
                    let is_table_key = token
                        .parent()
                        .and_then(|p| p.next_sibling())
                        .and_then(|t| t.first_child())
                        .is_some_and(|t| t.kind() == INLINE_TABLE);

                    if is_table_key {
                        builder.add_token(&token, TokenType::TomlTableKey, &[]);
                        continue;
                    }

                    // look for an array
                    let is_array_key = token
                        .parent()
                        .and_then(|p| p.next_sibling())
                        .and_then(|t| t.first_child())
                        .is_some_and(|t| t.kind() == ARRAY);

                    if is_array_key {
                        builder.add_token(&token, TokenType::TomlArrayKey, &[]);
                    }
                }
            }
        }
    }

    builder.build()
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
