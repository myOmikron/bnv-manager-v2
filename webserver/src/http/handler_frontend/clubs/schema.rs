use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::http::handler_frontend::users::schema::SimpleUser;

/// A full representation of a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct FullClub {
    /// Primary key
    pub uuid: Uuid,
    /// Name of the club
    pub name: String,
    /// User count associated with the club
    pub user_count: u64,
    /// The users that are admins of the club
    pub admins: Vec<SimpleUser>,
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
    pub name: String,
}

/// Errors that may occur during creation of a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct CreateClubErrors {
    /// Name is already in use
    pub name_in_use: bool,
}
