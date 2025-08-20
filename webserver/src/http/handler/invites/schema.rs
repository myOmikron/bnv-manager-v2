use std::num::NonZeroU8;

use galvyn::core::stuff::schema::SchemaDateTime;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::models::invite::InviteUuid;
use crate::models::role::Role;

/// API representation of an invitation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetInvite {
    /// Primary key of the invite
    pub uuid: InviteUuid,
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
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct AcceptInviteError {
    /// Empty password was supplied
    pub empty_password: bool,
    /// Invite has expired
    pub expired: bool,
}

/// Request to create an invitation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateInviteRequest {
    /// Reserved username
    pub username: MaxStr<255>,
    /// Display-name of the user
    pub display_name: MaxStr<255>,
    /// The point in time the invite expires
    pub valid_days: NonZeroU8,
    /// Roles to assign to the user
    pub roles: Vec<Role>,
}

/// Errors that can occur while creating an invitation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateInviteError {
    /// Username is already taken
    pub username_already_occupied: bool,
}
