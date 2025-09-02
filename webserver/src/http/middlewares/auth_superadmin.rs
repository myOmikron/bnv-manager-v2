use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Request;
use galvyn::core::re_exports::axum::middleware::Next;
use galvyn::core::re_exports::axum::response::Response;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use rorm::Database;

use crate::http::extractors::session_user::SessionUser;
use crate::models::account::Account;

/// Middleware function to check for superadmins
pub async fn auth_superadmin(
    SessionUser { uuid: account_uuid }: SessionUser,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    let mut tx = Database::global().start_transaction().await?;

    let account = Account::find_by_uuid(&mut tx, account_uuid)
        .await?
        .ok_or(ApiError::server_error("Account not found"))?;
    if !account
        .roles(&mut tx)
        .await?
        .iter()
        .any(|role| role.is_superadmin())
    {
        return Err(ApiError::bad_request("Account is not a superadmin"));
    }

    tx.commit().await?;

    Ok(next.run(req).await)
}
