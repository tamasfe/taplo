use taplo::{dom::Node, parser::parse};

pub fn json_to_toml(json: &str, inline: bool) -> Result<String, anyhow::Error> {
    let root: Node = serde_json::from_str(json)?;
    Ok(root.to_toml(inline, false))
}

pub fn toml_to_json(toml: &str) -> Result<String, anyhow::Error> {
    let root = parse(toml).into_dom();
    Ok(serde_json::to_string_pretty(&root)?)
}
