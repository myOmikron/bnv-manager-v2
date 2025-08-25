//! Schema for the currently logged-in user.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models::role::Role;

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
    pub member: Vec<Role>,
    /// The user's admin roles.
    pub admins: Vec<Role>,
}
