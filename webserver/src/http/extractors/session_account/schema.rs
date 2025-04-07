//! Schema of the extractor

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// Permissions of a session
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Permissions {
    /// User is admin
    pub admin: bool,
    /// The clubs an account is admin in
    pub club_admin: Vec<Uuid>,
    /// The clubs an account is user in
    pub club_user: Vec<Uuid>,
}
