//! An extractor to retrieve the currently logged-in account

use axum::extract::FromRequest;
use axum::extract::Request;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::Module;
use rorm::Database;
use uuid::Uuid;

use crate::http::handler::users::schema::Permissions;
use crate::http::SESSION_ACCOUNT;
use crate::models::account::Account;

/// The account of the currently logged-in user
pub struct SessionAccount {
    /// Uuid of the account
    pub uuid: Uuid,
    /// Username associated with the account
    pub username: String,
    /// Displayname of the associated account
    pub display_name: String,
    /// Permissions of the associated user
    pub permissions: Permissions,
}

impl<S> FromRequest<S> for SessionAccount
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request(req, state)
            .await
            .map_err(|_| ApiError::server_error("Missing session"))?;

        let user_uuid: Uuid = session
            .get(SESSION_ACCOUNT)
            .await?
            .ok_or(ApiError::bad_request("Invalid session user"))?;

        let mut tx = Database::global().start_transaction().await?;

        let mut account = rorm::query(&mut tx, Account)
            .condition(Account.uuid.equals(user_uuid))
            .optional()
            .await?
            .ok_or(ApiError::server_error("Invalid session state"))?;

        Account.roles.populate(&mut tx, &mut account).await?;

        let permissions = account.get_permissions(&mut tx).await?;

        tx.commit().await?;

        Ok(SessionAccount {
            uuid: account.uuid,
            username: account.username,
            display_name: account.display_name,
            permissions,
        })
    }
}
