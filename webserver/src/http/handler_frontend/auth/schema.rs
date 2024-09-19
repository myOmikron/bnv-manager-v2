//! The schema for local authentication

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::checked_string::CheckedString;
use crate::utils::secure_string::SecureString;

/// The request for local authentication
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoginRequest {
    /// The username that is used for logging in
    pub username: CheckedString<1>,
    /// The password for the user
    pub password: CheckedString<1, 255, SecureString>,
}
