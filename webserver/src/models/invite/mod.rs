use rorm::prelude::ForeignModel;
use rorm::Model;
use uuid::Uuid;

use crate::models::club::Club;

pub mod impls;

/// Representation for an invitation
#[derive(Model)]
pub struct Invite {
    /// Primary key of an invitation
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Whether the user should receive administrative privileges
    pub admin: bool,

    /// Whether the user is a club admin for the specified club
    pub club_admin: bool,

    /// The club associated with this user
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: Option<ForeignModel<Club>>,

    /// The username
    #[rorm(max_length = 255)]
    pub username: String,

    /// The display name for the user
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// The point in time the invitation expires
    pub expires_at: time::OffsetDateTime,
}
