use galvyn::core::stuff::schema::SchemaDateTime;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::http::extractors::session_account::schema::Permissions;

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
}

#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
pub struct AdminCreateInviteRequest {
    pub username: String,
    pub display_name: String,
    pub permissions: Permissions,
    pub valid_days: u8,
}

#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug, Default)]
pub struct AdminCreateInviteError {
    pub username_already_occupied: bool,
    pub valid_days_too_small: bool,
    pub invalid_clubs: Vec<Uuid>,
}
