use std::ops::Add;
use std::panic::Location;
use std::process::exit;

use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::schema::ApiStatusCode;
use rorm::db::Executor;
use rorm::prelude::ForeignModelByField;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::account::Account;
use crate::models::invite::Invite;
use crate::models::invite::InviteRole;
use crate::models::role::ROLE_ADMIN;
use crate::models::role::ROLE_CLUB_ADMIN;
use crate::models::role::ROLE_USER;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CreateInviteParams {
    pub username: String,
    pub display_name: String,
    pub is_admin: bool,
    pub club_admin: Vec<Uuid>,
    pub club_user: Vec<Uuid>,
    pub valid_days: u8,
}

impl Invite {
    pub async fn create(
        exe: impl Executor<'_>,
        CreateInviteParams {
            username,
            display_name,
            is_admin,
            valid_days,
            club_user,
            club_admin,
        }: CreateInviteParams,
    ) -> Result<Uuid, CreateInviteError> {
        let mut guard = exe.ensure_transaction().await?;

        let existing_account = rorm::query(guard.get_transaction(), Account)
            .condition(Account.username.equals(&username))
            .optional()
            .await?;

        if existing_account.is_some() {
            eprintln!("Already existing user with that username");
            exit(1);
        }

        let now = OffsetDateTime::now_utc();

        let invite_uuid = rorm::insert(guard.get_transaction(), Invite)
            .return_primary_key()
            .single(&Invite {
                uuid: Uuid::new_v4(),
                username,
                display_name,
                expires_at: now.add(Duration::days(i64::from(valid_days))),
            })
            .await?;

        if is_admin {
            rorm::insert(guard.get_transaction(), InviteRole)
                .return_nothing()
                .single(&InviteRole {
                    uuid: Uuid::new_v4(),
                    invite: ForeignModelByField(invite_uuid),
                    role: ForeignModelByField(ROLE_ADMIN.to_string()),
                    club: None,
                })
                .await?;
        }

        rorm::insert(guard.get_transaction(), InviteRole)
            .return_nothing()
            .bulk(club_user.into_iter().map(|uuid| InviteRole {
                uuid: Uuid::new_v4(),
                invite: ForeignModelByField(invite_uuid),
                role: ForeignModelByField(ROLE_USER.to_string()),
                club: Some(ForeignModelByField(uuid)),
            }))
            .await?;

        rorm::insert(guard.get_transaction(), InviteRole)
            .return_nothing()
            .bulk(club_admin.into_iter().map(|uuid| InviteRole {
                uuid: Uuid::new_v4(),
                invite: ForeignModelByField(invite_uuid),
                role: ForeignModelByField(ROLE_CLUB_ADMIN.to_string()),
                club: Some(ForeignModelByField(uuid)),
            }))
            .await?;

        guard.commit().await?;

        Ok(invite_uuid)
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
