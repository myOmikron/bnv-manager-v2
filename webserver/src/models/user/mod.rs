//! All user related models are defined here

use rorm::Model;
use time::OffsetDateTime;
use uuid::Uuid;

mod impls;

/// The representation of a user
#[derive(Model, Debug)]
pub struct User {
    /// Primary key of a user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The name that is used for displaying purposes
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// The point in time the user signed in the last time
    pub last_login: Option<OffsetDateTime>,

    /// The point in time the user was created
    #[rorm(auto_create_time)]
    pub created_at: OffsetDateTime,

    /// The username
    #[rorm(max_length = 255)]
    pub username: String,

    /// The hashed password
    #[rorm(max_length = 1024)]
    pub password: String,
}
