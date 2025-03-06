use axum::extract::Path;
use axum::Json;
use galvyn::core::Module;
use rorm::insert;
use rorm::query;
use rorm::Database;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::handler::invites::schema::AcceptInviteRequest;
use crate::models::invite::Invite;
use crate::models::user::User;
use crate::models::user::UserInsert;

#[galvyn::post("/{uuid}")]
pub async fn accept_invite(
    Path(uuid): Path<Uuid>,
    Json(AcceptInviteRequest { password }): Json<AcceptInviteRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let invite = query(&mut tx, Invite)
        .condition(Invite.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::bad_request("Invite not found"))?;

    let now = OffsetDateTime::now_utc();

    if invite.expires_at < now {
        return Err(ApiError::bad_request("Invite expired"));
    }

    let hashed = bcrypt::hash(password.into_inner(), 12)?;

    insert(&mut tx, User)
        .return_nothing()
        .single(&UserInsert {
            uuid,
            role: invite.role,
            username: invite.username.clone(),
            display_name: invite.display_name,
            password: hashed,
        })
        .await?;

    rorm::delete(&mut tx, Invite)
        .condition(Invite.username.equals(&invite.username))
        .await?;

    tx.commit().await?;

    Ok(())
}
