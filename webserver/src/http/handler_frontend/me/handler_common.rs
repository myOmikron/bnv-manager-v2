//! Common handler_frontend for the currently logged-in user

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use rorm::Database;
use tracing::instrument;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::me::Me;
use crate::http::handler_frontend::me::Roles;
use crate::models::account::Account;
use crate::models::role::Role;

#[get("/")]
#[instrument(name = "Api::common::get_me")]
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
        roles: Roles {
            super_admin: roles.contains(&Role::SuperAdmin),
            member: roles
                .clone()
                .into_iter()
                .flat_map(|x| match x {
                    Role::ClubMember { .. } => Some(x),
                    _ => None,
                })
                .collect(),
            admins: roles
                .clone()
                .into_iter()
                .flat_map(|x| match x {
                    Role::ClubAdmin { .. } => Some(x),
                    _ => None,
                })
                .collect(),
        },
    }))
}
