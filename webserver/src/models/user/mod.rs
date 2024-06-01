//! All user related models are defined here

use rorm::prelude::ForeignModel;
use rorm::Model;
use time::OffsetDateTime;
use uuid::Uuid;

pub use crate::models::user::impls::UserInsert;

mod impls;

/// The representation of a user
#[derive(Model)]
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
}

/// A user that is identified though an IDM server
#[derive(Model)]
pub struct OidcUser {
    /// Primary key of an oidc user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The reference to the user model
    #[rorm(on_delete = "Cascade", on_update = "Cascade")]
    pub user: ForeignModel<User>,

    /// The ID provided by the openid server
    #[rorm(max_length = 255)]
    pub oidc_id: String,
}

/// A locally authenticated user
#[derive(Model)]
pub struct LocalUser {
    /// Primary key of an oidc user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The reference to the user model
    #[rorm(on_delete = "Cascade", on_update = "Cascade")]
    pub user: ForeignModel<User>,

    /// The username
    #[rorm(max_length = 255)]
    pub username: String,

    /// The hashed password
    #[rorm(max_length = 1024)]
    pub password: String,
}
