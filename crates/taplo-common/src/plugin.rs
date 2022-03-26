use std::{borrow::Cow, sync::Arc};

use crate::{environment::Environment, schema::Schemas};
use async_trait::async_trait;
use serde_json::Value;
use taplo::dom::Keys;
use url::Url;

#[async_trait(?Send)]
pub trait Plugin<E: Environment>: 'static {
    fn name(&self) -> Cow<'static, str>;

    fn settings(&self, value: Value);

    async fn possible_schemas(
        &self,
        schemas: &Schemas<E>,
        root_schema_url: &Url,
        schema: &Value,
        root_path: &Keys,
        relative_path: &Keys,
        all_schemas: &mut Vec<(Keys, Keys, Arc<Value>)>,
    ) -> CollectSchemasAction;
}

pub enum CollectSchemasAction {
    /// Continue collecting schemas.
    Continue,
    /// Stop collection of additional schemas.
    Stop,
}
