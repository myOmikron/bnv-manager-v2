use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::http::extractors::session_account::schema::Permissions;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Me {
    pub uuid: Uuid,
    pub username: String,
    pub display_name: String,
    pub permissions: Permissions,
}
