# semantic_tokens.add_token

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\semantic_tokens.rs` (lines 129-155)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn add_token(
        &mut self,
        token: &SyntaxToken,
        ty: TokenType,
        modifiers: &[SemanticTokenModifier],
    ) {
        let range = self.mapper.range(token.text_range()).unwrap();

        let relative = relative_range(
            range,
            lsp_async_stub::util::Range::from_lsp(self.last_range.unwrap_or_default()),
        );

        #[allow(clippy::cast_possible_truncation)]
        self.tokens.push(SemanticToken {
            delta_line: relative.start.line as u32,
            delta_start: relative.start.character as u32,
            length: (relative.end.character - relative.start.character) as u32,
            token_type: ty as u32,
            token_modifiers_bitset: modifiers.iter().enumerate().fold(0, |mut total, (i, _)| {
                total += 1 << i;
                total
            }),
        });

        self.last_range = Some(range.into_lsp());
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn add_token(
        &mut self,
        token: &SyntaxToken,
        ty: TokenType,
        modifiers: &[SemanticTokenModifier],
    ) {
        let range = self.mapper.range(token.text_range()).unwrap();

        let relative = relative_range(
            range,
            lsp_async_stub::util::Range::from_lsp(self.last_range.unwrap_or_default()),
        );

        #[allow(clippy::cast_possible_truncation)]
        self.tokens.push(SemanticToken {
            delta_line: relative.start.line as u32,
            delta_start: relative.start.character as u32,
            length: (relative.end.character - relative.start.character) as u32,
            token_type: ty as u32,
            token_modifiers_bitset: modifiers.iter().enumerate().fold(0, |mut total, (i, _)| {
                total += 1 << i;
                total
            }),
        });

        self.last_range = Some(range.into_lsp());
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
