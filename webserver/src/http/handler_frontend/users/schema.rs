//! The schema for the users

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models::UserRole;
use crate::utils::checked_string::CheckedString;
use crate::utils::schemars::SchemaDateTime;
use crate::utils::secure_string::SecureString;

/// The fields of the change password request
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "error")]
#[allow(missing_docs)]
pub enum PwError {
    Incorrect,
}

/// The fields of the change password request
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "field")]
#[allow(missing_docs)]
pub enum ChangePwFormFields {
    CurrentPw(PwError),
}

/// The request to change user information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChangeMeRequest {
    /// The new name
    pub display_name: Option<CheckedString<1>>,
    /// The preferred user language
    pub preferred_lang: Option<CheckedString<1>>,
}

/// The request to change the password
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChangePwRequest {
    /// The current password of the user
    pub current_pw: CheckedString<1, 255, SecureString>,
    /// The password that should be set
    pub new_pw: CheckedString<1, 255, SecureString>,
}

/// The full representation for the user
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullUser {
    /// The identifier of the user
    pub uuid: Uuid,
    /// Preferred language of the user
    pub preferred_lang: String,
    /// Role of the user
    pub role: UserRole,
    /// The username
    pub username: String,
    /// Used for displaying purposes
    pub display_name: String,
    /// The last time the user has logged in
    pub last_login: Option<SchemaDateTime>,
    /// The time the user was created
    pub created_at: SchemaDateTime,
}
