use galvyn::core::stuff::schema::SchemaDateTime;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AdminAccount {
    pub uuid: Uuid,
    pub username: String,
    pub display_name: String,
    pub disabled: bool,
    pub created_at: SchemaDateTime,
}
