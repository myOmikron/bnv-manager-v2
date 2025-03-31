use axum::extract::Path;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::core::Module;
use rorm::prelude::ForeignModelByField;
use rorm::Database;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::http::handler::invites::schema::AcceptInviteRequest;
use crate::http::handler::invites::schema::FullInvite;
use crate::models::invite::Invite;
use crate::models::user::User;
use crate::models::user::UserInsert;
use crate::models::user::UserPassword;

#[galvyn::get("/{uuid}")]
pub async fn get_invite(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<FullInvite>> {
    let mut tx = Database::global().start_transaction().await?;

    let invite = rorm::query(&mut tx, Invite)
        .condition(Invite.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::bad_request("Invite not found."))?;

    tx.commit().await?;

    Ok(ApiJson(FullInvite {
        uuid: invite.uuid,
        username: invite.username,
        display_name: invite.display_name,
        expires_at: SchemaDateTime(invite.expires_at),
    }))
}

#[galvyn::post("/{uuid}")]
pub async fn accept_invite(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    ApiJson(AcceptInviteRequest { password }): ApiJson<AcceptInviteRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let invite = rorm::query(&mut tx, Invite)
        .condition(Invite.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::bad_request("Invite not found"))?;

    let now = OffsetDateTime::now_utc();

    if invite.expires_at < now {
        return Err(ApiError::bad_request("Invite expired"));
    }

    let hashed = bcrypt::hash(password.into_inner(), 12)
        .map_err(ApiError::map_server_error("hashing failed"))?;

    let password = rorm::insert(&mut tx, UserPassword)
        .return_primary_key()
        .single(&UserPassword {
            uuid: Uuid::new_v4(),
            password: hashed,
        })
        .await?;

    rorm::insert(&mut tx, User)
        .return_nothing()
        .single(&UserInsert {
            uuid,
            admin: invite.admin,
            username: invite.username.clone(),
            display_name: invite.display_name,
            password: Some(ForeignModelByField(password)),
        })
        .await?;

    rorm::delete(&mut tx, Invite)
        .condition(Invite.username.equals(&invite.username))
        .await?;

    tx.commit().await?;

    Ok(())
}
