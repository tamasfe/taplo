# Module: convert

**File:** `\crates\taplo-common\src\convert.rs`

## Overview

Module `convert` contains 2 analyzed entities.

```
use taplo::{dom::Node, parser::parse};

pub fn json_to_toml(json: &str, inline: bool) -> Result<String, anyhow::Error> {
    let root: Node = serde_json::from_str(json)?;
    Ok(root.to_toml(inline, false))
}

pub fn toml_to_json(toml: &str) -> Result<String, anyhow::Error> {
    let root = parse(toml).into_dom();
    Ok(serde_json::to_string_pretty(&root)?)
}
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `convert.json_to_toml` | function | No description |
| `convert.toml_to_json` | function | No description |

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
