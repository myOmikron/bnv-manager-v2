//! The handler for retrieving

use axum::extract::Path;
use rorm::query;
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
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
use crate::models::User;
use crate::models::UserInvite;
use crate::models::UserRole;
use crate::utils::schemars::SchemaDateTime;

/// Retrieve a single user invite
#[get("/:uuid")]
pub async fn get_user_invite(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<FormResult<FullUserInvite, GetUserInviteErrors>>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let Some(user_invite) = query!(&mut tx, UserInvite)
        .condition(UserInvite::F.uuid.equals(uuid))
        .optional()
        .await?
    else {
        return Ok(ApiJson(FormResult::err(GetUserInviteErrors {
            invite_invalid: true,
        })));
    };

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(FullUserInvite {
        uuid: user_invite.uuid,
        username: user_invite.username,
        display_name: user_invite.display_name,
        preferred_lang: user_invite.preferred_lang,
        created_at: SchemaDateTime(user_invite.created_at),
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
        .condition(UserInvite::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    User::create_user(
        user_invite.username,
        password.into_inner().into_inner(),
        user_invite.display_name,
        if let Some(club_uuid) = user_invite.club {
            match user_invite.role {
                UserRole::ClubAdmin => UserRoleWithClub::ClubAdmin {
                    club: *club_uuid.key(),
                },
                UserRole::User => UserRoleWithClub::User {
                    club: *club_uuid.key(),
                },
                _ => {
                    return Err(ApiError::new_internal_server_error(
                        "Invalid role and club combination",
                    ))
                }
            }
        } else {
            UserRoleWithClub::Administrator
        },
        user_invite.preferred_lang,
        &mut tx,
    )
    .await
    .map_err(ApiError::new_internal_server_error)?;

    rorm::delete!(&mut tx, UserInvite)
        .condition(UserInvite::F.uuid.equals(uuid))
        .await?;

    tx.commit().await?;

    Ok(())
}
