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
use crate::models::account::Account;
use crate::models::account::AccountInsert;
use crate::models::account::AccountPassword;
use crate::models::account::AccountRole;
use crate::models::invite::Invite;
use crate::models::invite::InviteRole;

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

    let invite_roles = rorm::query(&mut tx, InviteRole)
        .condition(InviteRole.invite.equals(uuid))
        .all()
        .await?;

    if invite_roles.is_empty() {
        return Err(ApiError::server_error("Invite without roles are invalid"));
    }

    let now = OffsetDateTime::now_utc();

    if invite.expires_at < now {
        return Err(ApiError::bad_request("Invite expired"));
    }

    let hashed = bcrypt::hash(password.into_inner(), 12)
        .map_err(ApiError::map_server_error("hashing failed"))?;

    let password = rorm::insert(&mut tx, AccountPassword)
        .return_primary_key()
        .single(&AccountPassword {
            uuid: Uuid::new_v4(),
            password: hashed,
        })
        .await?;

    let account_uuid = rorm::insert(&mut tx, Account)
        .return_primary_key()
        .single(&AccountInsert {
            uuid,
            username: invite.username.clone(),
            display_name: invite.display_name,
            password: Some(ForeignModelByField(password)),
        })
        .await?;

    rorm::insert(&mut tx, AccountRole)
        .bulk(invite_roles.into_iter().map(|role| AccountRole {
            uuid: Uuid::new_v4(),
            account: ForeignModelByField(account_uuid),
            role: role.role,
            club: role.club,
        }))
        .await?;

    rorm::delete(&mut tx, Invite)
        .condition(Invite.username.equals(&invite.username))
        .await?;

    tx.commit().await?;

    Ok(())
}
