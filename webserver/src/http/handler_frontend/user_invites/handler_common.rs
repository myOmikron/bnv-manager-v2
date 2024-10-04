//! The handler for retrieving

use axum::extract::Path;
use rorm::and;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::get;
use swaggapi::post;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::FormResult;
use crate::http::common::schemas::SingleUuid;
use crate::http::extractors::api_json::ApiJson;
use crate::http::handler_frontend::user_invites::schema::AcceptInvitePwRequest;
use crate::http::handler_frontend::user_invites::schema::FullUserInvite;
use crate::http::handler_frontend::user_invites::schema::GetUserInviteErrors;
use crate::models::User;
use crate::models::UserInvite;
use crate::utils::schemars::SchemaDateTime;

/// Retrieve a single user invite
#[get("/:uuid")]
pub async fn get_user_invite(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<FormResult<FullUserInvite, GetUserInviteErrors>>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let user_invite = query!(&mut tx, UserInvite)
        .condition(UserInvite::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    tx.commit().await?;

    if user_invite.accepted {
        return Ok(ApiJson(FormResult::err(GetUserInviteErrors {
            invite_used: true,
        })));
    }

    Ok(ApiJson(FormResult::ok(FullUserInvite {
        uuid: user_invite.uuid,
        username: user_invite.username,
        display_name: user_invite.display_name,
        preferred_lang: user_invite.preferred_lang,
        created_at: SchemaDateTime(user_invite.created_at),
        accepted: user_invite.accepted,
    })))
}

/// Accept the invite with a password
#[post("/:uuid/accept-with-pw")]
pub async fn accept_invite_pw(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    ApiJson(AcceptInvitePwRequest { password }): ApiJson<AcceptInvitePwRequest>,
) -> ApiResult<()> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let user_invite = query!(&mut tx, UserInvite)
        .condition(and!(
            UserInvite::F.uuid.equals(uuid),
            UserInvite::F.accepted.equals(false)
        ))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    User::create_user(
        user_invite.username,
        password.into_inner().into_inner(),
        user_invite.display_name,
        user_invite.role,
        user_invite.club.map(|x| *x.key()),
        user_invite.preferred_lang,
        &mut tx,
    )
    .await
    .map_err(ApiError::new_internal_server_error)?;

    update!(&mut tx, UserInvite)
        .condition(UserInvite::F.uuid.equals(uuid))
        .set(UserInvite::F.accepted, true)
        .await?;

    tx.commit().await?;

    Ok(())
}
