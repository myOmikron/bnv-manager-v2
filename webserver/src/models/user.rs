use rorm::Model;
use rorm::Patch;
use rorm::prelude::ForeignModel;
use uuid::Uuid;

/// User representation
#[derive(Model, Clone)]
pub struct User {
    /// Primary key of the user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// If true, the user has admin privileges
    pub admin: bool,

    /// An optional linked password of the user
    #[rorm(on_update = "Cascade", on_delete = "SetNull")]
    pub password: Option<ForeignModel<UserPassword>>,

    /// The name of the user
    #[rorm(max_length = 255, unique)]
    pub username: String,

    /// The name of the user
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// Whether the user is disabled
    #[rorm(default = false)]
    pub disabled: bool,

    /// The point in time the user was created
    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
}

/// Representation of a user password
#[derive(Model)]
pub struct UserPassword {
    /// Primary key of a user password
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// User hash
    #[rorm(max_length = 1024)]
    pub password: String,
}

#[derive(Patch)]
#[rorm(model = "User")]
pub(crate) struct UserInsert {
    pub uuid: Uuid,
    pub admin: bool,
    pub username: String,
    pub display_name: String,
    pub password: Option<ForeignModel<UserPassword>>,
}
