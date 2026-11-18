# Module: toml_test

**File:** `\crates\taplo-cli\src\commands\toml_test.rs`

## Overview

Module `toml_test` contains 5 analyzed entities.

```
use crate::Taplo;
use anyhow::anyhow;
use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};
use taplo::dom::{
    node::{DateTimeValue, DomNode},
    Node,
};
use taplo_common::environment::Environment;
use tokio::io::AsyncReadExt;

impl<E: Environment> Taplo<E> {
    pub async fn execute_toml_test(&self) -> Result<(), anyhow::Error> {
        let mut buf = String::new();
        self.env.stdin().read_to_string(&mut buf).await?;

        let parse = taplo::parser::parse(&buf);

        if !parse.errors.is_empty() {
            for err in parse.errors {
                eprintln!("{err}");
            }

            return Err(anyhow!("invalid toml"));
        }
        let dom = parse.into_dom();

        if let Err(err) = dom.validate() {
            for err in err {
                eprintln!("{err}");
            }
            return Err(anyhow!("invalid toml"));
        }

        serde_json::to_writer(std::io::stdout(), &TomlTestValue::new(&dom))?;

        Ok(())
    }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TomlTestType {
    String,
    Integer,
    Float,
    Bool,
    DateTime,
    #[serde(rename = "datetime-local")]
    DateTimeLocal,
    #[serde(rename = "date-local")]
    DateLocal,
    #[serde(rename = "time-local")]
    TimeLocal,
}

impl TomlTestType {
    fn of(node: &Node) -> Option<Self> {
        match node {
            Node::Bool(_) => Some(TomlTestType::Bool),
            Node::Integer(_) => Some(TomlTestType::Integer),
            Node::Float(_) => Some(TomlTestType::Float),
            Node::Str(_) => Some(TomlTestType::String),
            Node::Date(d) => match d.value() {
                DateTimeValue::OffsetDateTime(_) => Some(TomlTestType::DateTime),
                DateTimeValue::LocalDateTime(_) => Some(TomlTestType::DateTimeLocal),
                DateTimeValue::Date(_) => Some(TomlTestType::DateLocal),
                DateTimeValue::Time(_) => Some(TomlTestType::TimeLocal),
    
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `toml_test.execute_toml_test` | function | No description |
| `toml_test.of` | function | No description |
| `toml_test.new` | function | No description |
| `toml_test.serialize` | function | No description |
| `toml_test.TomlTestValue` | struct | No description |

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
