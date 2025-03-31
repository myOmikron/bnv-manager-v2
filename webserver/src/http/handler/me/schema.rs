use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Me {
    pub uuid: Uuid,
    pub username: String,
    pub display_name: String,
}
