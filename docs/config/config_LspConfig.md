# config.LspConfig

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\config.rs` (lines 35-42)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct LspConfig {
    pub taplo: TaploConfig,
    pub schema: SchemaConfig,
    pub completion: CompletionConfig,
    pub syntax: SyntaxConfig,
    pub formatter: taplo::formatter::OptionsIncompleteCamel,
    pub rules: Vec<Rule>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct LspConfig {
    pub taplo: TaploConfig,
    pub schema: SchemaConfig,
    pub completion: CompletionConfig,
    pub syntax: SyntaxConfig,
    pub formatter: taplo::formatter::OptionsIncompleteCamel,
    pub rules: Vec<Rule>,
}
```

## Git History

- Commits: 3
- Authors: 1
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
