//! Schemas of all club handlers

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::http::handler_frontend::users::schema::SimpleUser;
use crate::utils::checked_string::CheckedString;

/// A full representation of a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct FullClub {
    /// Primary key
    pub uuid: Uuid,
    /// Name of the club
    pub name: String,
    /// Domain that is used in mailcow
    pub domain: String,
    /// User count associated with the club
    pub user_count: u64,
    /// The users that are admins of the club
    pub admins: Vec<SimpleUser>,
    /// The count of websites of all users
    pub website_count: u64,
}

/// A simple representation of a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct SimpleClub {
    /// Primary key
    pub uuid: Uuid,
    /// Name of the club
    pub name: String,
    /// User count associated with the club
    pub user_count: u64,
    /// The count of the websites of all users
    pub website_count: u64,
}

/// A list of clubs
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct ClubList {
    /// List of all clubs
    pub clubs: Vec<SimpleClub>,
}

/// The request to create a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct CreateClubRequest {
    /// The name of the club
    pub name: CheckedString<1, 255>,
    /// The domain associated with the club
    pub domain: CheckedString<1, 255>,
}

/// Errors that may occur during creation of a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct CreateClubErrors {
    /// Name is already in use
    pub name_in_use: bool,
}

/// The request to update a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct UpdateClubRequest {
    /// The name of the club
    pub name: Option<String>,
}

/// Errors that may occur in an update club request
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateClubErrors {
    /// The new name is already in use
    pub name_in_use: bool,
}
