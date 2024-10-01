//! All user related models are defined here

use rorm::prelude::ForeignModel;
use rorm::Model;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;
use strum::EnumString;
use strum::IntoStaticStr;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::custom_db_enum;
use crate::models::Club;

mod impls;

custom_db_enum!(UserRole, "role");

/// The role of a user
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)] // std
#[derive(JsonSchema, Serialize, Deserialize)] // Conversion from and to json for api
#[derive(EnumString, IntoStaticStr, EnumIter)] // Conversion from and to string for db
#[allow(missing_docs)]
pub enum UserRole {
    Administrator,
    ClubAdmin,
    User,
}

/// The representation of a user
#[derive(Model, Debug)]
pub struct User {
    /// Primary key of a user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The preferred language of the user
    #[rorm(max_length = 255)]
    pub preferred_lang: String,

    /// The role of a user
    pub role: UserRole,

    /// The associated club
    ///
    /// In case role is Administrator, this field is empty
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: Option<ForeignModel<Club>>,

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

/// The representation of an open invitation for a user account
#[derive(Debug, Model)]
pub struct UserInvite {
    /// Primary key of a user
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The preferred language of the user
    #[rorm(max_length = 255)]
    pub preferred_lang: String,

    /// The role of a user
    pub role: UserRole,

    /// The associated club
    ///
    /// In case role is Administrator, this field is empty
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: Option<ForeignModel<Club>>,

    /// The name that is used for displaying purposes
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// The username
    #[rorm(max_length = 255)]
    pub username: String,
}
