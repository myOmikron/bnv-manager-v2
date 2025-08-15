//! Common handler for the currently logged-in user

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use rorm::Database;
use tracing::instrument;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler::me::Me;
use crate::models::account::Account;

#[get("/me")]
#[instrument(name = "Api::get_me")]
pub async fn get_me(SessionUser { uuid }: SessionUser) -> ApiResult<ApiJson<Me>> {
    let mut tx = Database::global().start_transaction().await?;

    let account = Account::find_by_uuid(&mut tx, uuid)
        .await?
        .ok_or(ApiError::server_error(
            "Account not found, while session user was found",
        ))?;

    let roles = account.roles(&mut tx).await?;

    tx.commit().await?;

    Ok(ApiJson(Me {
        uuid: account.uuid.0,
        username: account.username.to_string(),
        display_name: account.display_name.to_string(),
        roles,
    }))
}
