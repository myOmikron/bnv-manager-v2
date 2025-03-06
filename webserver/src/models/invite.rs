use rorm::Model;
use uuid::Uuid;

use crate::models::user::UserRole;

/// Representation for an invitation
#[derive(Model)]
pub struct Invite {
    /// Primary key of an invitation
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The role of the user
    pub role: UserRole,

    /// The username
    #[rorm(max_length = 255)]
    pub username: String,

    /// The display name for the user
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// The point in time the invitation expires
    pub expires_at: time::OffsetDateTime,
}
