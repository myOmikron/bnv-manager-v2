//! Authentication required middleware

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::schema::ApiStatusCode;
use uuid::Uuid;

use crate::http::SESSION_USER;

/// Checks the session if the [SESSION_USER] is present which will be the indicator
/// if the user is logged-in
pub(crate) async fn auth_required(
    session: Session,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    session
        .get::<Uuid>(SESSION_USER)
        .await?
        .ok_or(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Missing account uuid in session",
        ))?;

    Ok(next.run(req).await)
}

/// Checks the session if the [SESSION_USER] is present which will be the indicator
/// if the user is logged-in
pub(crate) async fn club_admin_required(
    session: Session,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    session
        .get::<Uuid>(SESSION_USER)
        .await?
        .ok_or(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Missing account uuid in session",
        ))?;

    Ok(next.run(req).await)
}

/// Checks the session if the [SESSION_USER] is present which will be the indicator
/// if the user is logged-in
pub(crate) async fn admin_required(
    session: Session,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    session
        .get::<Uuid>(SESSION_USER)
        .await?
        .ok_or(ApiError::new(
            ApiStatusCode::Unauthenticated,
            "Missing account uuid in session",
        ))?;

    Ok(next.run(req).await)
}
