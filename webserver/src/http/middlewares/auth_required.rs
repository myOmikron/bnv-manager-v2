//! Authentication required middleware

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::schema::ApiStatusCode;
use uuid::Uuid;

use crate::http::handler::users::schema::Permissions;
use crate::http::SESSION_ACCOUNT;
use crate::http::SESSION_PERMISSIONS;

/// Checks the session if the [SESSION_ACCOUNT] is present which will be the indicator
/// if the user is logged-in
pub(crate) async fn auth_required(
    session: Session,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    session
        .get::<Uuid>(SESSION_ACCOUNT)
        .await?
        .ok_or(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Missing account uuid in session",
        ))?;

    Ok(next.run(req).await)
}

/// Checks the session if the [SESSION_ACCOUNT] is present which will be the indicator
/// if the user is logged-in
pub(crate) async fn club_admin_required(
    session: Session,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    session
        .get::<Uuid>(SESSION_ACCOUNT)
        .await?
        .ok_or(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Missing account uuid in session",
        ))?;
    let permissions = session
        .get::<Permissions>(SESSION_PERMISSIONS)
        .await?
        .ok_or(ApiError::server_error("Missing permissions"))?;

    if permissions.club_admin.is_empty() {
        return Err(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Invalid permissions",
        ));
    }

    Ok(next.run(req).await)
}

/// Checks the session if the [SESSION_ACCOUNT] is present which will be the indicator
/// if the user is logged-in
pub(crate) async fn admin_required(
    session: Session,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    session
        .get::<Uuid>(SESSION_ACCOUNT)
        .await?
        .ok_or(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Missing account uuid in session",
        ))?;
    let permissions = session
        .get::<Permissions>(SESSION_PERMISSIONS)
        .await?
        .ok_or(ApiError::server_error("Missing permissions"))?;

    if !permissions.admin {
        return Err(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Invalid permissions",
        ));
    }

    Ok(next.run(req).await)
}
