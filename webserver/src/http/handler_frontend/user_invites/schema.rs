//! Schema of all user invite related handlers

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// The request when creating a user invite
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateUserInviteRequest {
    /// The username for the new user
    pub username: String,
    /// The display name of the new user
    pub display_name: String,
    /// Preferred language of the new user
    pub preferred_lang: String,
    /// The role of the new user
    pub role: UserRoleWithClub,
}

/// the user role with the corresponding club associated to it
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[allow(missing_docs)]
pub enum UserRoleWithClub {
    Administrator,
    ClubAdmin(Uuid),
    User(Uuid),
}

/// The response when creating a user invite
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateUserInviteResponse {
    /// The link of a user
    pub link: String,
}

/// The errors that can occur on creation of user invites
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateUserInviteErrors {
    /// The username is already in use
    pub username_in_use: bool,
}
