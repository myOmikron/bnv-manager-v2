use galvyn::core::stuff::schema::SchemaDateTime;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::models;
use crate::models::club::ClubUuid;

/// A single club
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Club {
    /// Primary key of a club
    pub uuid: ClubUuid,
    /// Name of the club
    pub name: MaxStr<255>,
    /// The last point in time the club was modified
    pub modified_at: SchemaDateTime,
    /// The point in time the club was created
    pub created_at: SchemaDateTime,
    /// The number of members in the club
    pub member_count: u64,
    /// The number of admins in the club
    pub admin_count: u64,
}

/// Request to create a club
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateClubRequest {
    /// Name of the club
    pub name: MaxStr<255>,
    /// Primary domain of the club
    pub primary_domain: MaxStr<255>,
}

/// Error when creating a club
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct CreateClubError {
    /// Whether the club name already exists
    pub name_already_exists: bool,
    /// Domain already exists
    pub domain_already_exists: bool,
}

/// Parameters for pagination
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PageParams {
    /// Offset for pagination
    pub offset: u64,
    /// Limit for pagination
    pub limit: u64,
    /// Search for usernames
    pub search: Option<MaxStr<255>>,
}

impl From<models::club::Club> for Club {
    fn from(value: models::club::Club) -> Self {
        Self {
            uuid: value.uuid,
            name: value.name,
            modified_at: SchemaDateTime(value.modified_at),
            created_at: SchemaDateTime(value.created_at),
            member_count: value.member_count,
            admin_count: value.admin_count,
        }
    }
}
