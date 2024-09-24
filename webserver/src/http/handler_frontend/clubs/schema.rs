use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// A full representation of a club
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct FullClub {
    /// Primary key
    pub uuid: Uuid,
    /// Name of the club
    pub name: String,
    /// User count associated with the club
    pub users: u64,
}

/// A list of clubs
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct ClubList {
    /// List of all clubs
    pub clubs: Vec<FullClub>,
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
