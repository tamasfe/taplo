use std::borrow::Cow;

use crate::{
    args::{GetCommand, OutputFormat},
    Taplo,
};
use anyhow::anyhow;
use codespan_reporting::files::SimpleFile;
use taplo::{
    dom::{Keys, Node},
    parser,
};
use taplo_common::environment::Environment;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl<E: Environment> Taplo<E> {
    pub async fn execute_get(&self, cmd: GetCommand) -> Result<(), anyhow::Error> {
        let mut stdout = self.env.stdout();

        // `--separator` should only be handled for a text output format
        if cmd.separator.is_some() && !matches!(cmd.output_format, OutputFormat::Value) {
            return Err(anyhow!(
                "`--separator` is only valid for `--output-format value`"
            ));
        }

        let source = match &cmd.file_path {
            Some(p) => String::from_utf8(self.env.read_file(p).await?)?,
            None => {
                let mut stdin = self.env.stdin();
                let mut s = String::new();
                stdin.read_to_string(&mut s).await?;
                s
            }
        };

        let parse = parser::parse(&source);

        let file_path = cmd
            .file_path
            .as_ref()
            .map(|p| p.to_string_lossy())
            .unwrap_or(Cow::Borrowed("-"));

        self.print_parse_errors(&SimpleFile::new(&file_path, &source), &parse.errors)
            .await?;

        if !parse.errors.is_empty() {
            return Err(anyhow!("syntax errors found"));
        }

        let node = parse.into_dom();

        if let Err(errors) = node.validate() {
            self.print_semantic_errors(&SimpleFile::new(&file_path, &source), errors)
                .await?;

            return Err(anyhow!("semantic errors found"));
        }

        match cmd.output_format {
            crate::args::OutputFormat::Json => {
                if let Some(p) = cmd.pattern {
                    let p = p.trim_start_matches('.');

                    let keys = p
                        .parse::<Keys>()
                        .map_err(|err| anyhow!("invalid pattern: {err}"))?;

                    let mut nodes = node
                        .find_all_matches(keys, false)
                        .map_err(|err| anyhow!("invalid pattern: {err}"))?;

                    if nodes.len() == 0 {
                        return Err(anyhow!("no values matched the pattern"));
                    }

                    if nodes.len() == 1 {
                        stdout
                            .write_all(&serde_json::to_vec_pretty(&nodes.next().unwrap().1)?)
                            .await?;
                        if !cmd.strip_newline {
                            stdout.write_all(b"\n").await?;
                        }
                        stdout.flush().await?;
                    } else {
                        stdout
                            .write_all(&serde_json::to_vec_pretty(
                                &nodes.map(|n| n.1).collect::<Vec<_>>(),
                            )?)
                            .await?;
                        if !cmd.strip_newline {
                            stdout.write_all(b"\n").await?;
                        }
                        stdout.flush().await?;
                    }
                } else {
                    stdout.write_all(&serde_json::to_vec_pretty(&node)?).await?;
                    if !cmd.strip_newline {
                        stdout.write_all(b"\n").await?;
                    }
                    stdout.flush().await?;
                }
            }
            crate::args::OutputFormat::Value => {
                let separator = cmd.separator.as_deref().unwrap_or("\n");
                let mut buf = if let Some(p) = cmd.pattern {
                    let p = p.trim_start_matches('.');

                    let nodes = p
                        .parse::<Keys>()
                        .and_then(|keys| node.find_all_matches(keys, false))
                        .map_err(|err| anyhow!("invalid pattern: {err}"))?;

                    if nodes.len() == 0 {
                        return Err(anyhow!("no values matched the pattern"));
                    }

                    let values = nodes
                        .map(|(_, node)| extract_value(&node, separator))
                        .collect::<Result<Vec<String>, _>>()?;

                    values.join(separator)
                } else {
                    extract_value(&node, separator)?
                };

                if !cmd.strip_newline {
                    buf += "\n";
                }

                stdout.write_all(buf.as_bytes()).await?;
                stdout.flush().await?;
            }
            crate::args::OutputFormat::Toml => {
                if let Some(p) = cmd.pattern {
                    let p = p.trim_start_matches('.');

                    let keys = p
                        .parse::<Keys>()
                        .map_err(|err| anyhow!("invalid pattern: {err}"))?;

                    let mut nodes = node
                        .find_all_matches(keys, false)
                        .map_err(|err| anyhow!("invalid pattern: {err}"))?;

                    if nodes.len() == 0 {
                        return Err(anyhow!("no values matched the pattern"));
                    }

                    if nodes.len() == 1 {
                        let mut buf = nodes.next().unwrap().1.to_toml(false, false);

                        if cmd.strip_newline {
                            if buf.ends_with('\n') {
                                let new_len = buf.trim_end().len();
                                buf.truncate(new_len);
                            }
                        } else if !buf.ends_with('\n') {
                            buf += "\n";
                        }

                        stdout.write_all(buf.as_bytes()).await?;
                        stdout.flush().await?;
                    } else {
                        let mut buf = String::from("[\n");

                        for (_, node) in nodes {
                            buf += "  ";
                            buf += &node.to_toml(true, false);
                            buf += ",\n";
                        }

                        buf += "]\n";

                        if cmd.strip_newline {
                            if buf.ends_with('\n') {
                                let new_len = buf.trim_end().len();
                                buf.truncate(new_len);
                            }
                        } else if !buf.ends_with('\n') {
                            buf += "\n";
                        }

                        stdout.write_all(buf.as_bytes()).await?;
                        stdout.flush().await?;
                    }
                    stdout.flush().await?;
                } else {
                    let mut buf = node.to_toml(false, false);

                    if cmd.strip_newline {
                        if buf.ends_with('\n') {
                            let new_len = buf.trim_end().len();
                            buf.truncate(new_len);
                        }
                    } else if !buf.ends_with('\n') {
                        buf += "\n";
                    }

                    stdout.write_all(buf.as_bytes()).await?;
                    stdout.flush().await?;
                }
            }
        }

        Ok(())
    }
}

fn extract_value(node: &Node, separator: &str) -> Result<String, anyhow::Error> {
    Ok(match node {
        Node::Table(_) => {
            return Err(anyhow!(
                r#"cannot print tables with the given output format, specify a different output format (e.g. with `-o json`) "#
            ))
        }
        Node::Array(arr) => {
            let mut values = Vec::new();

            for node in arr.items().read().iter() {
                values.push(extract_value(node, separator)?);
            }

            values.join(separator)
        }
        Node::Bool(b) => b.value().to_string(),
        Node::Str(s) => s.value().to_string(),
        Node::Integer(i) => i.value().to_string(),
        Node::Float(f) => f.value().to_string(),
        Node::Date(d) => d.value().to_string(),
        Node::Invalid(_) => "".into(),
    })
}
