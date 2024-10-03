//! Schema of all user invite related handlers

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::checked_string::CheckedString;
use crate::utils::schemars::SchemaDateTime;
use crate::utils::secure_string::SecureString;

/// The request when creating a user invite
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateUserInviteRequest {
    /// The username for the new user
    pub username: CheckedString<1, 255>,
    /// The display name of the new user
    pub display_name: CheckedString<1, 255>,
    /// Preferred language of the new user
    pub preferred_lang: CheckedString<1, 255>,
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

/// A user invite
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct FullUserInvite {
    /// Primary key of a user invite
    pub uuid: Uuid,
    /// The username
    pub username: String,
    /// The name that is used for displaying purposes
    pub display_name: String,
    /// The preferred language of the user
    pub preferred_lang: String,
    /// The point in time the invite was created
    pub created_at: SchemaDateTime,
    /// Whether the invite was accepted
    pub accepted: bool,
}

/// The errors that may occur while retrieving an invitation
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct GetUserInviteErrors {
    /// The invite was already used
    pub invite_used: bool,
}

/// Accept the invite with a password
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AcceptInvitePwRequest {
    /// The password that should be set
    pub password: CheckedString<1, 255, SecureString>,
}
