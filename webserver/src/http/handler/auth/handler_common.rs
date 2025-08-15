use galvyn::core::Module;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::post;
use rorm::Database;
use tracing::instrument;

use crate::http::extractors::session_user::SESSION_USER;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler::auth::SignInRequest;
use crate::models::account::Account;

#[post("/sign-in")]
#[instrument(name = "Api::sign_in", skip(password))]
pub async fn sign_in(
    session: Session,
    ApiJson(SignInRequest { username, password }): ApiJson<SignInRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let account = Account::find_by_username(&mut tx, &username)
        .await?
        .ok_or(ApiError::bad_request("Username not found"))?;

    tx.commit().await?;

    if !account.check_password(password)? {
        return Err(ApiError::bad_request("Invalid password"));
    }

    session
        .insert(SESSION_USER, SessionUser { uuid: account.uuid })
        .await?;

    Ok(())
}
