//! The schema for ldap authentication

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// The request to login via LDAP
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct LdapLoginRequest {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}
