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
    pub roles: Vec<Role>,
}
