//! Common handler_frontend for the currently logged-in user

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::get;
use galvyn::post;
use galvyn::put;
use galvyn::rorm::Database;
use tracing::instrument;
use zxcvbn::Score;
use zxcvbn::zxcvbn;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::me::MeSchema;
use crate::http::handler_frontend::me::RoleSchema;
use crate::http::handler_frontend::me::SetPasswordErrors;
use crate::http::handler_frontend::me::SetPasswordRequest;
use crate::http::handler_frontend::me::UpdateMeRequest;
use crate::models::account::Account;
use crate::models::club::Club;

#[get("/")]
#[instrument(name = "Api::common::get_me")]
pub async fn get_me(SessionUser { uuid }: SessionUser) -> ApiResult<ApiJson<MeSchema>> {
    let mut tx = Database::global().start_transaction().await?;

    let account = Account::get_by_uuid(&mut tx, uuid)
        .await?
        .ok_or(ApiError::server_error(
            "Account not found, while session user was found",
        ))?;

    if let Account::Superadmin(superadmin) = account {
        return Ok(ApiJson(MeSchema {
            uuid: superadmin.uuid(),
            username: superadmin.username,
            display_name: superadmin.display_name,
            role: RoleSchema::SuperAdmin,
        }));
    }

    let club_name = Club::find_by_uuid(
        &mut tx,
        match &account {
            Account::ClubMember(account) => account.club,
            Account::ClubAdmin(account) => account.club,
            _ => unreachable!(),
        },
    )
    .await?
    .ok_or(ApiError::server_error("Club should exist"))?
    .name;

    tx.commit().await?;

    Ok(ApiJson(match account {
        Account::ClubMember(club_member) => MeSchema {
            uuid: club_member.uuid(),
            username: club_member.username,
            display_name: club_member.display_name,
            role: RoleSchema::ClubMember {
                club: club_member.club,
                club_name: club_name.clone(),
                email: club_member.email,
            },
        },
        Account::ClubAdmin(club_admin) => MeSchema {
            uuid: club_admin.uuid(),
            username: club_admin.username,
            display_name: club_admin.display_name,
            role: RoleSchema::ClubAdmin {
                club: club_admin.club,
                club_name,
            },
        },
        Account::Superadmin(_) => unreachable!(),
    }))
}

#[put("/")]
#[instrument(name = "Api::common::update_me")]
pub async fn update_me(
    SessionUser { uuid }: SessionUser,
    ApiJson(UpdateMeRequest { display_name }): ApiJson<UpdateMeRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let mut account = Account::get_by_uuid(&mut tx, uuid)
        .await?
        .ok_or(ApiError::server_error("Account from session not found"))?;

    account.set_display_name(&mut tx, display_name).await?;

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

    let mut account = Account::get_by_uuid(&mut tx, uuid)
        .await?
        .ok_or(ApiError::server_error("Account from session not found"))?;
    let [display_name, username] = match &account {
        Account::ClubMember(account) => &[
            account.display_name.clone().into_inner(),
            account.username.clone().into_inner(),
        ],
        Account::ClubAdmin(account) => &[
            account.display_name.clone().into_inner(),
            account.username.clone().into_inner(),
        ],
        Account::Superadmin(account) => &[
            account.display_name.clone().into_inner(),
            account.username.clone().into_inner(),
        ],
    };

    let entropy = zxcvbn(&password, &[display_name, username]);
    if entropy.score() < Score::Four {
        return Ok(ApiJson(FormResult::err(SetPasswordErrors {
            low_entropy: true,
        })));
    }
    account.set_password(&mut tx, &password).await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(())))
}
