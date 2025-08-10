//! Roles are defined in this module

use crate::models::club::ClubUuid;

pub(in crate::models) mod db;

/// The available roles of the manager
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Role {
    /// The super administrator. Has rights to manager clubs.
    SuperAdmin,
    /// The admin of a club. Can manage users and settings of its club
    ClubAdmin(ClubUuid),
    /// A member of a club.
    ClubMember(ClubUuid),
}
