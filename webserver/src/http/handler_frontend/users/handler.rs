//! The handler for the users

use axum::Json;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::get;
use swaggapi::post;
use tracing::instrument;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::errors::FormResult;
use crate::http::common::schemas::FormError;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::users::schema::ChangePwFormFields;
use crate::http::handler_frontend::users::schema::ChangePwRequest;
use crate::http::handler_frontend::users::schema::FullUser;
use crate::http::handler_frontend::users::schema::PwError;
use crate::models::User;
use crate::utils::hashing;
use crate::utils::hashing::hash_pw;
use crate::utils::hashing::VerifyPwError;
use crate::utils::schemars::SchemaDateTime;

/// Retrieve the currently logged-in user
#[get("/me")]
#[instrument(skip_all, ret, err)]
pub async fn get_me(SessionUser(user): SessionUser) -> ApiResult<Json<FullUser>> {
    Ok(Json(FullUser {
        uuid: user.uuid,
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
    SessionUser(user): SessionUser,
    Json(ChangePwRequest { current_pw, new_pw }): Json<ChangePwRequest>,
) -> ApiResult<FormResult<(), ChangePwFormFields>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let password = query!(&mut tx, (User::F.password,))
        .condition(User::F.uuid.equals(user.uuid))
        .optional()
        .await?
        .ok_or(ApiError::InternalServerError)?
        .0;

    if let Err(err) = hashing::verify_pw(&current_pw, &password) {
        return match err {
            VerifyPwError::Hash(_) => Err(ApiError::InternalServerError),
            VerifyPwError::Mismatch => Ok(Err(FormError::single(ChangePwFormFields::CurrentPw(
                PwError::Incorrect,
            )))),
        };
    }

    let hashed = hash_pw(&new_pw)?;

    update!(&mut tx, User)
        .condition(User::F.uuid.equals(user.uuid))
        .set(User::F.password, hashed)
        .exec()
        .await?;

    tx.commit().await?;

    Ok(Ok(()))
}
