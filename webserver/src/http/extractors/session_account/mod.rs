//! An extractor to retrieve the currently logged-in account

use axum::extract::FromRequest;
use axum::extract::Request;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::Module;
use rorm::Database;
use tracing::warn;
use uuid::Uuid;

use crate::http::extractors::session_account::schema::Permissions;
use crate::http::SESSION_ACCOUNT;
use crate::models::account::Account;
use crate::models::account::AccountRole;
use crate::models::role::ROLE_ADMIN;
use crate::models::role::ROLE_CLUB_ADMIN;
use crate::models::role::ROLE_USER;

pub mod schema;

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

        tx.commit().await?;

        let roles: Vec<AccountRole> = account.roles.cached.unwrap();

        let mut admin = false;
        let mut club_admin = vec![];
        let mut club_user = vec![];

        for role in roles {
            if role.role.0 == ROLE_ADMIN {}

            match role.role.0.as_str() {
                ROLE_ADMIN => {
                    admin = true;
                }
                ROLE_CLUB_ADMIN => {
                    club_admin.push(
                        role.club
                            .ok_or(ApiError::server_error("Clubadmin without associated club"))?
                            .0,
                    );
                }
                ROLE_USER => club_user.push(
                    role.club
                        .ok_or(ApiError::server_error("Clubuser without associated club"))?
                        .0,
                ),
                _ => warn!("Encountered invalid account role: {}", role.role.0),
            }
        }

        Ok(SessionAccount {
            uuid: account.uuid,
            username: account.username,
            display_name: account.display_name,
            permissions: Permissions {
                admin,
                club_admin,
                club_user,
            },
        })
    }
}
