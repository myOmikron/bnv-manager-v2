use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::core::Module;
use rorm::Database;

use crate::http::handler::invites::schema::AdminCreateInviteError;
use crate::http::handler::invites::schema::AdminCreateInviteRequest;
use crate::models::account::Account;
use crate::models::invite::impls::CreateInviteParams;
use crate::models::invite::Invite;

#[galvyn::post("/invites")]
pub async fn admin_create_invite(
    ApiJson(AdminCreateInviteRequest {
        username,
        display_name,
    }): ApiJson<AdminCreateInviteRequest>,
) -> ApiResult<ApiJson<FormResult<SingleUuid, AdminCreateInviteError>>> {
    let mut tx = Database::global().start_transaction().await?;

    let existing = rorm::query(&mut tx, Account)
        .condition(Account.username.equals(&username))
        .optional()
        .await?;

    if existing.is_some() {
        return Ok(ApiJson(FormResult::err(
            AdminCreateInviteError::UsernameAlreadyOccupied,
        )));
    }

    let uuid = Invite::create(
        &mut tx,
        CreateInviteParams {
            username,
            display_name,
            is_admin: true,
            is_club_admin: false,
            valid_days: 8,
        },
    )
    .await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(SingleUuid { uuid })))
}
