//! Handlers for user invites of administrative users

use rorm::insert;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::post;
use uuid::Uuid;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::FormResult;
use crate::http::extractors::api_json::ApiJson;
use crate::http::handler_frontend::user_invites::schema::CreateUserInviteErrors;
use crate::http::handler_frontend::user_invites::schema::CreateUserInviteRequest;
use crate::http::handler_frontend::user_invites::schema::CreateUserInviteResponse;
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
use crate::models::Club;
use crate::models::UserInvite;
use crate::models::UserRole;

/// Create a new invite for a user
#[post("/")]
pub async fn create_invite(
    ApiJson(CreateUserInviteRequest {
        username,
        display_name,
        preferred_lang,
        role,
    }): ApiJson<CreateUserInviteRequest>,
) -> ApiResult<ApiJson<FormResult<CreateUserInviteResponse, CreateUserInviteErrors>>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let existing = query!(&mut tx, UserInvite)
        .condition(UserInvite::F.username.equals(&username))
        .optional()
        .await?;

    if existing.is_some() {
        return Ok(ApiJson(FormResult::err(CreateUserInviteErrors {
            username_in_use: true,
        })));
    }

    let mut club_uuid = None;
    let role = match role {
        UserRoleWithClub::Administrator => UserRole::Administrator,
        UserRoleWithClub::ClubAdmin(club) => {
            club_uuid = Some(club);
            UserRole::ClubAdmin
        }
        UserRoleWithClub::User(club) => {
            club_uuid = Some(club);
            UserRole::User
        }
    };

    if let Some(club) = club_uuid {
        query!(&mut tx, (Club::F.uuid,))
            .condition(Club::F.uuid.equals(club))
            .optional()
            .await?
            .ok_or(ApiError::BadRequest)?;
    }

    let invite = insert!(&mut tx, UserInvite)
        .single(&UserInvite {
            uuid: Uuid::new_v4(),
            preferred_lang,
            role,
            club: club_uuid.map(ForeignModelByField::Key),
            display_name,
            username,
        })
        .await?;

    tx.commit().await?;

    let mut link = GLOBAL.conf.http.origin.clone();
    link.set_path(&format!("/invites/{}", invite.uuid));

    Ok(ApiJson(FormResult::ok(CreateUserInviteResponse {
        link: link.to_string(),
    })))
}
