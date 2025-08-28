//! Schema for the currently logged-in user.

use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models::club::ClubUuid;

/// Representation of the currently logged-in user.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Me {
    /// The user's UUID.
    pub uuid: Uuid,
    /// The user's username.
    pub username: String,
    /// The user's display name.
    pub display_name: String,
    /// The user's roles.
    pub roles: Roles,
}

/// The roles of a user.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Roles {
    /// Whether the user is a super admin.
    pub super_admin: bool,
    /// The user's membership roles
    pub member: Vec<ClubMemberRole>,
    /// The user's admin roles.
    pub admins: Vec<ClubAdminRole>,
}

/// A club membership role.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClubMemberRole {
    /// The club's UUID.
    pub club_uuid: ClubUuid,
    /// The club's name.
    pub club_name: MaxStr<255>,
}

/// A club membership role.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClubAdminRole {
    /// The club's UUID.
    pub club_uuid: ClubUuid,
    /// The club's name.
    pub club_name: MaxStr<255>,
}
