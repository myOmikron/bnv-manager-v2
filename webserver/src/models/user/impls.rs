use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHasher;
use rand::thread_rng;
use rorm::db::Executor;
use rorm::insert;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use rorm::Patch;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::models::LocalUser;
use crate::models::User;

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

impl User {
    /// Create an internal user
    #[instrument(skip(password, executor), ret, err)]
    pub async fn create_internal(
        username: String,
        password: String,
        display_name: String,
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

        let existing = query!(exe.get_transaction(), LocalUser)
            .condition(LocalUser::F.username.equals(&username))
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
            })
            .await?;

        insert!(exe.get_transaction(), LocalUser)
            .single(&LocalUser {
                uuid: Uuid::new_v4(),
                user: ForeignModelByField::Key(user),
                username,
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
    /// The display name of the user
    pub display_name: String,
}
