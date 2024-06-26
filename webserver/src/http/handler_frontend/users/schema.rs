//! The schema for the users

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::not_empty_string::NotEmptyString;
use crate::utils::schemars::SchemaDateTime;

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

/// The request to change the password
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChangePwRequest {
    /// The current password of the user
    pub current_pw: NotEmptyString,
    /// The password that should be set
    pub new_pw: NotEmptyString,
}

/// The full representation for the user
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullUser {
    /// The identifier of the user
    pub uuid: Uuid,
    /// Used for displaying purposes
    pub display_name: String,
    /// The last time the user has logged in
    pub last_login: Option<SchemaDateTime>,
    /// The time the user was created
    pub created_at: SchemaDateTime,
}
