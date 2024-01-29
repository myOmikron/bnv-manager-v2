//! User related models

use rorm::Model;
use uuid::Uuid;

/// The representation for the user
#[derive(Model)]
pub struct User {
    /// Primary key of the user
    #[rorm(primary_key)]
    pub uuid: Uuid,
}
