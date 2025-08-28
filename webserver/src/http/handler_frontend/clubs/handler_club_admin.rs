//! Club admin endpoints

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::get;
use rorm::Database;
use tracing::instrument;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::clubs::schema;
use crate::models::account::Account;
use crate::models::club::Club;
use crate::models::club::ClubUuid;

#[get("/{uuid}")]
#[instrument(name = "Api::club_admin::get_club")]
pub async fn get_club(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    SessionUser { uuid: account_uuid }: SessionUser,
) -> ApiResult<ApiJson<schema::Club>> {
    let mut tx = Database::global().start_transaction().await?;

    let account = Account::find_by_uuid(&mut tx, account_uuid)
        .await?
        .ok_or(ApiError::server_error("Account not found"))?;
    if !account
        .is_club_admin_for_club(&mut tx, ClubUuid(uuid))
        .await?
    {
        return Err(ApiError::bad_request(
            "Account is not a club admin of this club",
        ));
    }

    let club = Club::find_by_uuid(&mut tx, ClubUuid(uuid))
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    tx.commit().await?;

    Ok(ApiJson(schema::Club::from(club)))
}
