//! The schema for the users

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::schemars::SchemaDateTime;

/// The full representation for the user
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullUser {
    /// The identifier of the user
    pub uuid: Uuid,
    /// Used for displaying purposes
    pub display_name: String,
    /// The last point in time the user has signed in
    pub last_login: Option<SchemaDateTime>,
    /// The point in time the user was created
    pub created_at: SchemaDateTime,
}
