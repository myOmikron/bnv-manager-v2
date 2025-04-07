use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::core::Module;
use rorm::Database;

use crate::http::handler::invites::schema::AdminCreateInviteError;
use crate::http::handler::invites::schema::AdminCreateInviteRequest;
use crate::models::account::Account;
use crate::models::club::Club;
use crate::models::invite::impls::CreateInviteParams;
use crate::models::invite::Invite;

#[galvyn::post("/invites")]
pub async fn admin_create_invite(
    ApiJson(AdminCreateInviteRequest {
        username,
        display_name,
        permissions,
        valid_days,
    }): ApiJson<AdminCreateInviteRequest>,
) -> ApiResult<ApiJson<FormResult<SingleUuid, AdminCreateInviteError>>> {
    let mut error = None;

    if valid_days == 0 {
        error = Some(AdminCreateInviteError {
            valid_days_too_small: true,
            ..Default::default()
        });
    }

    let mut tx = Database::global().start_transaction().await?;

    let existing = rorm::query(&mut tx, Account)
        .condition(Account.username.equals(&username))
        .optional()
        .await?;

    if existing.is_some() {
        let mut tmp = error.unwrap_or_default();
        tmp.username_already_occupied = true;
        error = Some(tmp);
    }

    let mut invalid_clubs = vec![];

    for club in permissions.club_admin.iter().chain(&permissions.club_user) {
        let c = rorm::query(&mut tx, Club)
            .condition(Club.uuid.equals(*club))
            .optional()
            .await?;

        if c.is_none() {
            invalid_clubs.push(*club);
        }
    }
    if !invalid_clubs.is_empty() {
        let mut tmp = error.unwrap_or_default();
        tmp.invalid_clubs = invalid_clubs;
        error = Some(tmp);
    }

    if let Some(err) = error {
        return Ok(ApiJson(FormResult::err(err)));
    }

    let uuid = Invite::create(
        &mut tx,
        CreateInviteParams {
            username,
            display_name,
            is_admin: permissions.admin,
            club_admin: permissions.club_admin,
            club_user: permissions.club_user,
            valid_days,
        },
    )
    .await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(SingleUuid { uuid })))
}
