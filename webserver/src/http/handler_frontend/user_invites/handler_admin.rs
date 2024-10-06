//! Handlers for user invites of administrative users

use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::post;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::FormResult;
use crate::http::extractors::api_json::ApiJson;
use crate::http::handler_frontend::user_invites::schema::CreateUserInviteErrors;
use crate::http::handler_frontend::user_invites::schema::CreateUserInviteRequestAdmin;
use crate::http::handler_frontend::user_invites::schema::CreateUserInviteResponse;
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
use crate::models::User;
use crate::models::UserInvite;
use crate::models::UserRole;

/// Create a new invite for a user
#[post("/")]
pub async fn create_invite_admin(
    ApiJson(CreateUserInviteRequestAdmin {
        username,
        display_name,
        preferred_lang,
        role,
    }): ApiJson<CreateUserInviteRequestAdmin>,
) -> ApiResult<ApiJson<FormResult<CreateUserInviteResponse, CreateUserInviteErrors>>> {
    let username = username.into_inner();
    let display_name = display_name.into_inner();
    let preferred_lang = preferred_lang.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;

    let mut existing = query!(&mut tx, (UserInvite::F.uuid,))
        .condition(UserInvite::F.username.equals(&username))
        .optional()
        .await?
        .is_some();

    // If existing is not set yet, check the userbase for existing users
    if !existing {
        existing = query!(&mut tx, (User::F.uuid,))
            .condition(User::F.username.equals(&username))
            .optional()
            .await?
            .is_some();
    }

    if existing {
        return Ok(ApiJson(FormResult::err(CreateUserInviteErrors {
            username_in_use: true,
        })));
    }

    let mut club_uuid = None;
    let role = match role {
        UserRoleWithClub::Administrator => UserRole::Administrator,
        UserRoleWithClub::ClubAdmin { club } => {
            club_uuid = Some(club);
            UserRole::ClubAdmin
        }
        UserRoleWithClub::User { club } => {
            club_uuid = Some(club);
            UserRole::User
        }
    };

    let invite = UserInvite::create_invite(
        &mut tx,
        username,
        display_name,
        preferred_lang,
        role,
        club_uuid,
    )
    .await?;

    tx.commit().await?;

    let mut link = GLOBAL.conf.http.origin.clone();
    link.set_path(&format!("/invites/{}", invite.uuid));

    Ok(ApiJson(FormResult::ok(CreateUserInviteResponse {
        link: link.to_string(),
    })))
}
