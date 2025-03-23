use galvyn::core::stuff::schema::SchemaDateTime;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models::user::UserRole;

#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
pub struct AcceptInviteRequest {
    pub password: MaxStr<72>,
}

#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
pub struct FullInvite {
    pub uuid: Uuid,
    pub username: String,
    pub display_name: String,
    pub expires_at: SchemaDateTime,
    pub role: UserRole,
}
