use std::path::Path;

use schemars::schema::RootSchema;
use taplo::schema::SchemaIndex;
use crate::external::{mkdir, write_file};

pub async fn get_schema(
    path: &str,
    cache_path: Option<impl AsRef<Path>>,
) -> Result<(RootSchema, Option<anyhow::Error>), anyhow::Error> {
    crate::external::get_schema(path, cache_path).await
}

pub async fn cache_schema_index(
    index: &SchemaIndex,
    cache_path: impl AsRef<Path>,
) -> Result<(), anyhow::Error> {
    mkdir(cache_path.as_ref().to_str().unwrap())?;
    write_file(
        cache_path
            .as_ref()
            .join("schema_index")
            .with_extension("json")
            .to_str()
            .unwrap(),
        &serde_json::to_vec(&index).unwrap(),
    )
    .await
}
