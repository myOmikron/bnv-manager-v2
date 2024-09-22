use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHasher;
use rand::thread_rng;
use rorm::db::Executor;
use rorm::insert;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use rorm::Patch;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::models::User;
use crate::models::UserRole;

/// The error that might occur when creating an internal user
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum CreateInternalUserError {
    #[error("Database error: {0}")]
    Database(#[from] rorm::Error),
    #[error("Hashing error: {0}")]
    Hash(#[from] argon2::password_hash::Error),
    #[error("There's already a user with the chosen username")]
    UsernameOccupied,
    #[error("There were empty fields")]
    EmptyData,
}

/// The error that might occur when creating a ldap user
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum CreateLdapUserError {
    #[error("Database error: {0}")]
    Database(#[from] rorm::Error),
}

impl User {
    /// Create an internal user
    #[instrument(skip(password, executor), ret, err)]
    pub async fn create_user(
        username: String,
        password: String,
        display_name: String,
        role: UserRole,
        preferred_lang: String,
        executor: impl Executor<'_>,
    ) -> Result<Uuid, CreateInternalUserError> {
        if username.is_empty() || password.is_empty() || display_name.is_empty() {
            return Err(CreateInternalUserError::EmptyData);
        }

        let salt = SaltString::generate(&mut thread_rng());
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        let mut exe = executor.ensure_transaction().await?;

        let existing = query!(exe.get_transaction(), User)
            .condition(User::F.username.equals(&username))
            .optional()
            .await?
            .is_some();

        if existing {
            return Err(CreateInternalUserError::UsernameOccupied);
        }

        let user = insert!(exe.get_transaction(), User)
            .return_primary_key()
            .single(&UserInsert {
                uuid: Uuid::new_v4(),
                display_name,
                username,
                role,
                preferred_lang,
                password: password_hash,
            })
            .await?;

        exe.commit().await?;

        Ok(user)
    }
}

/// The patch to insert a user
#[derive(Debug, Patch)]
#[rorm(model = "User")]
pub struct UserInsert {
    /// The primary key of the user
    pub uuid: Uuid,
    /// The role of the user
    pub role: UserRole,
    /// The chosen language of the user
    pub preferred_lang: String,
    /// The display name of the user
    pub display_name: String,
    /// Username of the user
    pub username: String,
    /// Hashed password of the user
    pub password: String,
}
