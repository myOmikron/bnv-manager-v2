use std::ops::Add;
use std::panic::Location;
use std::process::exit;

use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::schema::ApiStatusCode;
use rorm::db::Executor;
use rorm::insert;
use rorm::query;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::account::Account;
use crate::models::invite::Invite;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CreateInviteParams {
    pub username: String,
    pub display_name: String,
    pub is_admin: bool,
    pub is_club_admin: bool,
    pub valid_days: u8,
}

impl Invite {
    pub async fn create(
        exe: impl Executor<'_>,
        CreateInviteParams {
            username,
            display_name,
            is_admin,
            is_club_admin,
            valid_days,
        }: CreateInviteParams,
    ) -> Result<Uuid, CreateInviteError> {
        let mut guard = exe.ensure_transaction().await?;

        let existing_account = query(guard.get_transaction(), Account)
            .condition(Account.username.equals(&username))
            .optional()
            .await?;

        if existing_account.is_some() {
            eprintln!("Already existing user with that username");
            exit(1);
        }

        let now = OffsetDateTime::now_utc();

        let uuid = insert(guard.get_transaction(), Invite)
            .return_primary_key()
            .single(&Invite {
                uuid: Uuid::new_v4(),
                admin: is_admin,
                club_admin: is_club_admin,
                club: None,
                username,
                display_name,
                expires_at: now.add(Duration::days(i64::from(valid_days))),
            })
            .await?;

        guard.commit().await?;

        Ok(uuid)
    }
}

/// Database errors
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum CreateInviteError {
    #[error("Database error: {0}")]
    Database(#[from] rorm::Error),
}

impl From<CreateInviteError> for ApiError {
    #[track_caller]
    fn from(value: CreateInviteError) -> Self {
        match value {
            CreateInviteError::Database(err) => ApiError {
                code: ApiStatusCode::InternalServerError,
                context: None,
                location: Location::caller(),
                source: Some(err.into()),
            },
        }
    }
}
