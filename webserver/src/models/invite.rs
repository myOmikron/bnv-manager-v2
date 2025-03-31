use rorm::Model;
use uuid::Uuid;

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
