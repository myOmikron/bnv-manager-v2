//! The handler for local authentication

use axum::Json;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::post;
use tower_sessions::Session;
use tracing::instrument;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::handler_frontend::auth::schema::LoginRequest;
use crate::models;
use crate::models::LocalUser;
use crate::utils::hashing;
use crate::utils::hashing::VerifyPwError;

/// Use the local authentication for logging in
#[post("/login")]
#[instrument(skip(session))]
pub async fn login(
    session: Session,
    Json(LoginRequest { username, password }): Json<LoginRequest>,
) -> ApiResult<()> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let user = query!(&mut tx, LocalUser)
        .condition(LocalUser::F.username.equals(username))
        .optional()
        .await?
        .ok_or(ApiError::Unauthenticated)?;

    hashing::verify_pw(&password, &user.password).map_err(|x| match x {
        VerifyPwError::Hash(_) => ApiError::InternalServerError,
        VerifyPwError::Mismatch => ApiError::Unauthenticated,
    })?;

    session.insert("user", *user.user.key()).await?;
    // We have to call save manually as the id is only populated after creating the session
    session.save().await?;

    let Some(id) = session.id() else {
        return Err(ApiError::new_internal_server_error("No ID in session"));
    };
    update!(&mut tx, models::Session)
        .condition(models::Session::F.id.equals(id.to_string()))
        .set(
            models::Session::F.user,
            Some(ForeignModelByField::Key(*user.user.key())),
        )
        .exec()
        .await?;

    tx.commit().await?;

    Ok(())
}

/// Drop the current session and logg-out
#[post("/logout")]
#[instrument(skip_all)]
pub async fn logout(session: Session) -> ApiResult<()> {
    session.flush().await?;
    Ok(())
}
