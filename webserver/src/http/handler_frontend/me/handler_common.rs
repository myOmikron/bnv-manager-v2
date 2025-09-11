//! Common handler_frontend for the currently logged-in user

use std::collections::HashMap;

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::get;
use galvyn::post;
use galvyn::put;
use rorm::Database;
use tracing::instrument;
use zxcvbn::Score;
use zxcvbn::zxcvbn;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::me::Me;
use crate::http::handler_frontend::me::Roles;
use crate::http::handler_frontend::me::SetPasswordErrors;
use crate::http::handler_frontend::me::SetPasswordRequest;
use crate::http::handler_frontend::me::UpdateMeRequest;
use crate::http::handler_frontend::me::schema;
use crate::models::account::Account;
use crate::models::club::Club;
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

    let clubs: HashMap<_, _> = Club::find_all(&mut tx)
        .await?
        .into_iter()
        .map(|x| (x.uuid, x))
        .collect();

    tx.commit().await?;

    Ok(ApiJson(Me {
        uuid: account.uuid(),
        username: account.username.to_string(),
        display_name: account.display_name.to_string(),
        roles: Roles {
            super_admin: roles.contains(&Role::SuperAdmin),
            member: roles
                .clone()
                .into_iter()
                .flat_map(|x| match x {
                    Role::ClubMember { club_uuid } => Some(schema::ClubMemberRole {
                        club_uuid,
                        club_name: clubs
                            .get(&club_uuid)
                            .map(|x| x.name.clone())
                            .unwrap_or_default(),
                    }),
                    _ => None,
                })
                .collect(),
            admins: roles
                .clone()
                .into_iter()
                .flat_map(|x| match x {
                    Role::ClubAdmin { club_uuid } => Some(schema::ClubAdminRole {
                        club_uuid,
                        club_name: clubs
                            .get(&club_uuid)
                            .map(|x| x.name.clone())
                            .unwrap_or_default(),
                    }),
                    _ => None,
                })
                .collect(),
        },
    }))
}

#[put("/")]
#[instrument(name = "Api::common::update_me")]
pub async fn update_me(
    SessionUser { uuid }: SessionUser,
    ApiJson(UpdateMeRequest { display_name }): ApiJson<UpdateMeRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let mut account = Account::find_by_uuid(&mut tx, uuid)
        .await?
        .ok_or(ApiError::server_error("Account from session not found"))?;

    account.display_name = display_name;

    account.update(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}

#[post("/set-password")]
#[instrument(name = "Api::common::set_password", skip(password))]
pub async fn set_password(
    SessionUser { uuid }: SessionUser,
    ApiJson(SetPasswordRequest { password }): ApiJson<SetPasswordRequest>,
) -> ApiResult<ApiJson<FormResult<(), SetPasswordErrors>>> {
    let mut tx = Database::global().start_transaction().await?;

    if password.is_empty() {
        return Err(ApiError::bad_request("Empty password"));
    }

    let mut account = Account::find_by_uuid(&mut tx, uuid)
        .await?
        .ok_or(ApiError::server_error("Account from session not found"))?;

    let entropy = zxcvbn(&password, &[&account.display_name, &account.username]);
    if entropy.score() < Score::Four {
        return Ok(ApiJson(FormResult::err(SetPasswordErrors {
            low_entropy: true,
        })));
    }
    account.set_password(&mut tx, password).await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(())))
}
