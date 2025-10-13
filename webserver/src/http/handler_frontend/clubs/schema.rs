use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::models::club::Club;
use crate::models::club::ClubUuid;
use crate::models::domain::DomainUuid;

/// A single club
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClubSchema {
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
    /// Primary domain of the club
    pub primary_domain: MaxStr<255>,
}

/// Request to create a club
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateClubRequest {
    /// Name of the club
    pub name: MaxStr<255>,
    /// Primary domain of the club
    pub primary_domain: DomainUuid,
}

/// Error when creating a club
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct CreateClubError {
    /// Whether the club name already exists
    pub name_already_exists: bool,
    /// The domain is already associated with another club and can't be reused
    pub domain_already_associated: bool,
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

/// Request to associate a domain with a club
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AssociateDomainRequest {
    /// The domain to associate with the club
    pub domain: DomainUuid,
}

impl From<Club> for ClubSchema {
    fn from(value: Club) -> Self {
        Self {
            uuid: value.uuid,
            name: value.name,
            modified_at: SchemaDateTime(value.modified_at),
            created_at: SchemaDateTime(value.created_at),
            member_count: value.member_count,
            admin_count: value.admin_count,
            primary_domain: value.primary_domain,
        }
    }
}
