//! Common handlers for invites.

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::get;
use galvyn::post;
use rorm::Database;
use tracing::instrument;

use crate::http::handler_frontend::invites::AcceptInvite;
use crate::http::handler_frontend::invites::AcceptInviteError;
use crate::http::handler_frontend::invites::GetInvite;
use crate::models::invite::AcceptInviteParams;
use crate::models::invite::Invite;
use crate::models::invite::InviteUuid;

#[get("/{uuid}")]
#[instrument(name = "Api::common::get_invite")]
pub async fn get_invite_common(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<GetInvite>> {
    let mut tx = Database::global().start_transaction().await?;

    let invite = Invite::find_by_uuid(&mut tx, InviteUuid(uuid))
        .await?
        .ok_or(ApiError::bad_request("Invite not found"))?;

    tx.commit().await?;

    Ok(ApiJson(GetInvite::from(invite)))
}

#[post("/{uuid}/accept")]
#[instrument(name = "Api::common::accept_invite", skip(password))]
pub async fn accept_invite(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    ApiJson(AcceptInvite { password }): ApiJson<AcceptInvite>,
) -> ApiResult<ApiJson<FormResult<(), AcceptInviteError>>> {
    let mut tx = Database::global().start_transaction().await?;

    let invite = Invite::find_by_uuid(&mut tx, InviteUuid(uuid))
        .await?
        .ok_or(ApiError::bad_request("Invite not found"))?;

    let res = invite
        .accept_invite(&mut tx, AcceptInviteParams { password })
        .await?;

    if let Err(err) = res {
        return match err {
            crate::models::invite::AcceptInviteError::Expired => {
                Ok(ApiJson(FormResult::err(AcceptInviteError {
                    expired: true,
                    ..Default::default()
                })))
            }
        };
    }

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(())))
}
