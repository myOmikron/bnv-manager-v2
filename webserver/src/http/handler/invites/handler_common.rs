use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::get;
use galvyn::post;
use rorm::Database;
use tracing::instrument;

use crate::http::handler::invites::AcceptInvite;
use crate::http::handler::invites::AcceptInviteError;
use crate::http::handler::invites::GetInvite;
use crate::models::invite::AcceptInviteParams;
use crate::models::invite::Invite;
use crate::models::invite::InviteUuid;

#[get("/{uuid}")]
#[instrument(name = "Api::get_invite")]
pub async fn get_invite_common(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<GetInvite>> {
    let mut tx = Database::global().start_transaction().await?;

    let invite = Invite::find_by_uuid(&mut tx, InviteUuid(uuid))
        .await?
        .ok_or(ApiError::bad_request("Invite not found"))?;

    tx.commit().await?;

    Ok(ApiJson(GetInvite {
        uuid,
        username: invite.username,
        display_name: invite.display_name,
        expires_at: SchemaDateTime(invite.expires_at),
        created_at: SchemaDateTime(invite.created_at),
    }))
}

#[post("/{uuid}/accept")]
#[instrument(name = "Api::accept_invite", skip(password))]
pub async fn accept_invite(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    ApiJson(AcceptInvite { password }): ApiJson<AcceptInvite>,
) -> ApiResult<ApiJson<FormResult<(), AcceptInviteError>>> {
    let mut tx = Database::global().start_transaction().await?;

    let invite = Invite::find_by_uuid(&mut tx, InviteUuid(uuid))
        .await?
        .ok_or(ApiError::bad_request("Invite not found"))?;

    invite
        .accept_invite(&mut tx, AcceptInviteParams { password })
        .await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(())))
}
