use rorm::prelude::ForeignModel;
use rorm::Model;
use uuid::Uuid;

use crate::models::club::Club;
use crate::models::role::Role;

pub mod impls;

/// Representation for an invitation
#[derive(Model)]
pub struct Invite {
    /// Primary key of an invitation
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The username
    #[rorm(max_length = 255)]
    pub username: String,

    /// The display name for the user
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// The point in time the invitation expires
    pub expires_at: time::OffsetDateTime,
}

/// Role of the invite
#[derive(Model)]
pub struct InviteRole {
    /// The primary key of the invite role
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The associated invite
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub invite: ForeignModel<Invite>,

    /// The associated role
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub role: ForeignModel<Role>,

    /// The associated club in case of club users or club admins
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: Option<ForeignModel<Club>>,
}
