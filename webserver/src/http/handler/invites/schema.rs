use galvyn::core::stuff::schema::SchemaDateTime;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// API representation of an invitation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetInvite {
    /// Primary key of the invite
    pub uuid: Uuid,
    /// Reserved username
    pub username: MaxStr<255>,
    /// Display-name of the user
    pub display_name: MaxStr<255>,
    /// The point in time the invite expires
    pub expires_at: SchemaDateTime,
    /// The point in time the invite was created
    pub created_at: SchemaDateTime,
}

/// Accept an open invite
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AcceptInvite {
    /// The new password to set
    pub password: MaxStr<72>,
}

/// Errors that can occur while accepting an invitation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AcceptInviteError {
    /// Empty password was supplied
    pub empty_password: bool,
}
