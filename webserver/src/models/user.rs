use rorm::DbEnum;
use rorm::Model;
use rorm::Patch;
use uuid::Uuid;

/// Roles for a user
#[derive(DbEnum)]
pub enum UserRole {
    /// Admin user
    Admin,
    /// The admin of a club
    ClubAdmin,
    /// The user that's part of a club
    ClubUser,
}

/// User representation
#[derive(Model)]
pub struct User {
    /// Primary key of the user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Role of the user
    pub role: UserRole,

    /// The name of the user
    #[rorm(max_length = 255, unique)]
    pub username: String,

    /// The name of the user
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// User hash
    #[rorm(max_length = 1024)]
    pub password: String,

    /// Whether the user is disabled
    #[rorm(default = false)]
    pub disabled: bool,

    /// The point in time the user was created
    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
}

#[derive(Patch)]
#[rorm(model = "User")]
pub(crate) struct UserInsert {
    pub uuid: Uuid,
    pub role: UserRole,
    pub username: String,
    pub display_name: String,
    pub password: String,
}
