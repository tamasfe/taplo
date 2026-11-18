# Module: parse

**File:** `\crates\taplo\examples\parse.rs`

## Overview

Module `parse` contains 1 analyzed entities.

```
fn main() {
    const SOURCE: &str = "value = 1
value = 2

[table]
string = 'some string'";

    let parse_result = taplo::parser::parse(SOURCE);

    // Check for syntax errors.
    // These are not carried over to DOM errors.
    assert!(parse_result.errors.is_empty());

    // let root_node = parse_result.into_dom();

    // Check for semantic errors.
    // In this example "value" is a duplicate key.
    // assert_eq!(root_node.errors().len(), 1);
}
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `parse.main` | function | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
