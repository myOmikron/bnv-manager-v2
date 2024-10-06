//! The handler for the users

use axum::Json;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::get;
use swaggapi::post;
use swaggapi::put;
use tracing::instrument;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::FormResult;
use crate::http::extractors::api_json::ApiJson;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::users::schema::ChangeMeRequest;
use crate::http::handler_frontend::users::schema::ChangePwErrors;
use crate::http::handler_frontend::users::schema::ChangePwRequest;
use crate::http::handler_frontend::users::schema::FullUser;
use crate::models::User;
use crate::utils::hashing;
use crate::utils::hashing::hash_pw;
use crate::utils::hashing::VerifyPwError;
use crate::utils::schemars::SchemaDateTime;

/// Retrieve the currently logged-in user
#[get("/me")]
#[instrument(skip_all, ret, err)]
pub async fn get_me(SessionUser { user }: SessionUser) -> ApiResult<Json<FullUser>> {
    let user = user.0;

    Ok(Json(FullUser {
        uuid: user.uuid,
        username: user.username,
        preferred_lang: user.preferred_lang,
        role: user.role,
        display_name: user.display_name,
        last_login: user.last_login.map(SchemaDateTime),
        created_at: SchemaDateTime(user.created_at),
    }))
}

/// Change the password of the currently logged-in user
///
/// This may only be called by local users
#[post("/me/change-pw")]
#[instrument(skip_all, ret, err)]
pub async fn change_password(
    SessionUser { user }: SessionUser,
    Json(ChangePwRequest { current_pw, new_pw }): Json<ChangePwRequest>,
) -> ApiResult<ApiJson<FormResult<(), ChangePwErrors>>> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    let password = query!(&mut tx, (User::F.password,))
        .condition(User::F.uuid.equals(user.uuid))
        .optional()
        .await?
        .ok_or(ApiError::new_internal_server_error("".to_string()))?
        .0;

    if let Err(err) = hashing::verify_pw(&current_pw, &password) {
        return match err {
            VerifyPwError::Hash(_) => Err(ApiError::new_internal_server_error(
                "hash error".to_string(),
            )),
            VerifyPwError::Mismatch => Ok(ApiJson(FormResult::err(ChangePwErrors {
                current_pw: true,
            }))),
        };
    }

    let hashed = hash_pw(&new_pw)?;

    update!(&mut tx, User)
        .condition(User::F.uuid.equals(user.uuid))
        .set(User::F.password, hashed)
        .exec()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::Ok { value: () }))
}

/// Updates the current user information
#[put("/me")]
pub async fn update_me(
    SessionUser { user }: SessionUser,
    Json(ChangeMeRequest {
        display_name,
        preferred_lang,
    }): Json<ChangeMeRequest>,
) -> ApiResult<()> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    update!(&mut tx, User)
        .condition(User::F.uuid.equals(user.uuid))
        .begin_dyn_set()
        .set_if(User::F.display_name, display_name.map(|x| x.into_inner()))
        .set_if(
            User::F.preferred_lang,
            preferred_lang.map(|x| x.into_inner()),
        )
        .finish_dyn_set()
        .map_err(|_| ApiError::BadRequest)?
        .await?;

    tx.commit().await?;

    Ok(())
}
