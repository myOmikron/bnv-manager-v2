use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::Module;
use galvyn::rorm::Database;

use crate::http::handler::auth::schema::LoginRequest;
use crate::http::handler::auth::schema::LoginResponse;
use crate::http::SESSION_ACCOUNT;
use crate::http::SESSION_PERMISSIONS;
use crate::models::account::Account;

#[galvyn::post("/login")]
pub async fn login(
    session: Session,
    ApiJson(LoginRequest { username, password }): ApiJson<LoginRequest>,
) -> ApiResult<ApiJson<FormResult<(), LoginResponse>>> {
    let mut tx = Database::global().start_transaction().await?;

    let data = rorm::query(&mut tx, (Account, Account.password.password))
        .condition(Account.username.equals(&username))
        .optional()
        .await?;

    let mut account = if let Some((account, pw_hash)) = data {
        let pw_correct = bcrypt::verify(password, &pw_hash)
            .map_err(ApiError::map_server_error("Hashing error"))?;

        if !pw_correct {
            return Ok(ApiJson(FormResult::err(LoginResponse {
                username_or_password: true,
            })));
        }
        account
    } else {
        // We provide a default for the pw_hash to run the hash function regardless whether the user
        // was found or not. The empty password will not match the given hash
        // This is done to not give away whether a user exists
        bcrypt::verify(
            "",
            "$2b$12$LsJoVd8kpYpFhdCqragDquZByeotIQSbRfLx.38NwQTkgW5nM74WC",
        )
        .map_err(ApiError::map_server_error("Hashing error"))?;
        return Ok(ApiJson(FormResult::err(LoginResponse {
            username_or_password: true,
        })));
    };

    let permissions = account.get_permissions(&mut tx).await?;
    tx.commit().await?;

    session.insert(SESSION_ACCOUNT, &account.uuid).await?;
    session.insert(SESSION_PERMISSIONS, permissions).await?;

    Ok(ApiJson(FormResult::ok(())))
}

#[galvyn::post("/logout")]
pub async fn logout(session: Session) -> ApiResult<()> {
    session.delete().await?;

    Ok(())
}
