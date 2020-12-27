use crate::{formatter, parser::parse, value::Value};
use schemars::schema::RootSchema;
use serde_crate::{Deserialize, Serialize};
use std::convert::TryFrom;
use verify::{
    serde::{KeySpans, Spanned},
    Verifier,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(
    source: &str,
    ignore_errors: bool,
    formatter_opts_json: Option<String>,
) -> Result<JsValue, JsValue> {
    let formatter_opts = match formatter_opts_json {
        Some(s) => {
            let incomplete: formatter::OptionsIncompleteCamel = match serde_json::from_str(&s) {
                Ok(c) => c,
                Err(err) => {
                    return Err(JsValue::from_str(&format!(
                        "invalid formatter configuration: {}",
                        err
                    )));
                }
            };

            let mut o = formatter::Options::default();
            o.update_camel(incomplete);
            o
        }
        None => formatter::Options::default(),
    };

    let p = parse(source);

    if !p.errors.is_empty() && !ignore_errors {
        return Err(JsValue::from_str(
            r#"the document has syntax errors, use the "ignoreErrors" option to ignore them (potentially destructive)"#,
        ));
    }

    Ok(JsValue::from_str(&formatter::format_green(
        p.green_node,
        formatter_opts,
    )))
}

#[derive(Default, Serialize, Deserialize)]
#[serde(crate = "serde_crate")]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(crate = "serde_crate")]
pub struct LintError {
    pub range: Option<Range>,
    pub error: String,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(crate = "serde_crate")]
pub struct LintResult {
    pub errors: Vec<LintError>,
}

#[wasm_bindgen]
pub fn lint(
    source: &str,
    schema_key_spans: bool,
    schema_json: Option<String>,
) -> Result<JsValue, JsValue> {
    let p = parse(source);

    if !p.errors.is_empty() {
        return Ok(JsValue::from_serde(&LintResult {
            errors: p
                .errors
                .into_iter()
                .map(|err| LintError {
                    range: Range {
                        start: err.range.start().into(),
                        end: err.range.end().into(),
                    }
                    .into(),
                    error: err.message,
                })
                .collect(),
        })
        .unwrap());
    }

    let dom = p.into_dom();

    if !dom.errors().is_empty() {
        return Ok(JsValue::from_serde(&LintResult {
            errors: dom
                .errors()
                .into_iter()
                .map(|err| LintError {
                    range: None,
                    error: err.to_string(),
                })
                .collect(),
        })
        .unwrap());
    }

    if let Some(schema_json) = schema_json {
        let schema: RootSchema = serde_json::from_str(&schema_json)
            .map_err(|err| JsValue::from_str(&format!("invalid schema: {}", err)))?;

        if schema_key_spans {
            if let Err(errors) = schema.verify_value(&Spanned::new(
                &Value::try_from(dom).unwrap(),
                KeySpans::default(),
            )) {
                return Ok(JsValue::from_serde(&LintResult {
                    errors: errors
                        .iter()
                        .map(|err| LintError {
                            range: None,
                            error: format!(
                                "{err}{span}",
                                err = &err,
                                span = match &err.span {
                                    Some(s) => format!(" ({})", s.dotted()),
                                    None => String::new(),
                                }
                            ),
                        })
                        .collect(),
                })
                .unwrap());
            }
        } else {
            if let Err(errors) = schema.verify_value(&dom) {
                return Ok(JsValue::from_serde(&LintResult {
                    errors: errors
                        .iter()
                        .map(|err| LintError {
                            range: err.span.map(|s| Range {
                                start: s.0.start().into(),
                                end: s.0.end().into(),
                            }),
                            error: err.value.to_string(),
                        })
                        .collect(),
                })
                .unwrap());
            }
        }
    }

    Ok(JsValue::from_serde(&LintResult::default()).unwrap())
}

#[wasm_bindgen]
pub fn from_json(json_source: &str) -> Result<JsValue, JsValue> {
    let v: serde_json::Value =
        serde_json::from_str(json_source).map_err(|err| JsValue::from_str(&format!("{}", err)))?;
    Ok(JsValue::from_str(
        &toml::to_string_pretty(&v).map_err(|err| JsValue::from_str(&format!("{}", err)))?,
    ))
}

#[wasm_bindgen]
pub fn to_json(toml_source: &str) -> Result<JsValue, JsValue> {
    let parse = parse(&toml_source);

    if !parse.errors.is_empty() {
        return Err(JsValue::from_str("invalid TOML"));
    }

    let dom = parse.into_dom();

    if !dom.errors().is_empty() {
        return Err(JsValue::from_str("invalid TOML"));
    }

    let val = Value::try_from(dom).unwrap();

    Ok(JsValue::from_str(
        &serde_json::to_string_pretty(&val)
            .map_err(|err| JsValue::from_str(&format!("{}", err)))?,
    ))
}
