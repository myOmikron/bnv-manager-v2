//! The handler the ldap authentication

use axum::Json;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::post;
use time::OffsetDateTime;
use tower_sessions::Session;
use tracing::error;
use tracing::instrument;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::handler_frontend::ldap::schema::LdapLoginRequest;
use crate::models;
use crate::models::LdapUser;
use crate::models::User;

/// Authenticate using ldap login
#[post("/login-ldap")]
#[instrument(skip(password), ret, err)]
pub async fn login_ldap(
    session: Session,
    Json(LdapLoginRequest { username, password }): Json<LdapLoginRequest>,
) -> ApiResult<()> {
    let data = GLOBAL
        .ldap
        .get_user(username)
        .await?
        .ok_or(ApiError::Unauthenticated)?;

    if !GLOBAL.ldap.auth_dn(&data.dn, &password).await? {
        return Err(ApiError::Unauthenticated);
    }

    let mut tx = GLOBAL.db.start_transaction().await?;

    let user = query!(&mut tx, LdapUser)
        .condition(LdapUser::F.ldap_dn.equals(&data.dn))
        .optional()
        .await?;

    let user_uuid = match user {
        None => User::create_ldap(data.dn, data.display_name, &mut tx)
            .await
            .map_err(|_| ApiError::InternalServerError)?,
        Some(user) => *user.user.key(),
    };

    session.insert("user", user_uuid).await?;
    // We have to call save manually as the id is only populated after creating the session
    session.save().await?;

    let Some(id) = session.id() else {
        error!("No ID in session");
        return Err(ApiError::Unauthenticated);
    };
    update!(&mut tx, models::Session)
        .condition(models::Session::F.id.equals(id.to_string()))
        .set(
            models::Session::F.user,
            Some(ForeignModelByField::Key(user_uuid)),
        )
        .exec()
        .await?;

    update!(&mut tx, User)
        .condition(User::F.uuid.equals(user_uuid))
        .set(User::F.last_login, Some(OffsetDateTime::now_utc()))
        .await?;

    tx.commit().await?;

    Ok(())
}
