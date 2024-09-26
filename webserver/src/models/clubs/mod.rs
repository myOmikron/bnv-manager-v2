//! Models related to a club

use rorm::prelude::ForeignModel;
use rorm::Model;
use uuid::Uuid;

use crate::models::User;

mod impls;

/// Representation of a club
#[derive(Debug, Model)]
pub struct Club {
    /// The primary key of a club
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Name of the club
    #[rorm(unique, max_length = 255)]
    pub name: String,
}

/// Representation of an admin of a club
#[derive(Debug, Model)]
pub struct ClubAdmin {
    /// The primary key of a club
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Reference to the club
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<Club>,

    /// Reference to the user
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub user: ForeignModel<User>,
}

/// Representation of a user of a club
#[derive(Debug, Model)]
pub struct ClubUser {
    /// Primary key of the club user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Reference to the club
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<Club>,

    /// Reference to the user
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub user: ForeignModel<User>,
}