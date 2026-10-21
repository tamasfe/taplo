#![cfg(all(feature = "schema", not(target_family = "wasm")))]

use serde_json::{json, Value};
use std::sync::Arc;
use taplo::dom::Keys;
use taplo_common::{environment::native::NativeEnvironment, schema::Schemas};
use url::Url;

fn completion_schema(schema: Value) -> Arc<Value> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let schemas = Schemas::new(NativeEnvironment::new(), reqwest::Client::new());
            let url = Url::parse("https://example.com/schema.json").unwrap();
            schemas.add_schema(&url, Arc::new(schema)).await;

            schemas
                .possible_schemas_from(&url, &Value::Null, &Keys::empty(), 2)
                .await
                .unwrap()
                .into_iter()
                .find(|(_, path, _)| path.to_string() == "mode")
                .expect("the composed property should produce a completion")
                .2
        })
}

#[test]
fn all_of_reference_preserves_outer_description() {
    let schema = completion_schema(json!({
        "type": "object",
        "definitions": {
            "Mode": {
                "type": "string",
                "enum": ["fast", "slow"],
                "description": "Shared mode description"
            }
        },
        "properties": {
            "mode": {
                "description": "Property-specific description",
                "allOf": [{ "$ref": "#/definitions/Mode" }]
            }
        }
    }));

    assert_eq!(schema["type"], "string");
    assert_eq!(schema["enum"], json!(["fast", "slow"]));
    assert_eq!(schema["description"], "Property-specific description");
    assert!(schema.get("allOf").is_none());
}

#[test]
fn all_of_merges_multiple_inline_schemas() {
    let schema = completion_schema(json!({
        "type": "object",
        "properties": {
            "mode": {
                "description": "Choose a mode",
                "allOf": [
                    { "type": "string", "enum": ["fast", "slow"] },
                    { "default": "fast" }
                ]
            }
        }
    }));

    assert_eq!(schema["type"], "string");
    assert_eq!(schema["enum"], json!(["fast", "slow"]));
    assert_eq!(schema["default"], "fast");
    assert_eq!(schema["description"], "Choose a mode");
    assert!(schema.get("allOf").is_none());
}

#[test]
fn all_of_handles_nested_reference_wrappers() {
    let schema = completion_schema(json!({
        "type": "object",
        "definitions": {
            "Mode": {
                "type": "string",
                "enum": ["fast", "slow"]
            },
            "WrappedMode": {
                "description": "Inner description",
                "allOf": [{ "$ref": "#/definitions/Mode" }]
            }
        },
        "properties": {
            "mode": {
                "description": "Outer description",
                "allOf": [{ "$ref": "#/definitions/WrappedMode" }]
            }
        }
    }));

    assert_eq!(schema["type"], "string");
    assert_eq!(schema["enum"], json!(["fast", "slow"]));
    assert_eq!(schema["description"], "Outer description");
    assert!(schema.get("allOf").is_none());
}
