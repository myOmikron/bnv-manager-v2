//! Common handlers of the openapi

use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;

/// Generate the openapi definition
#[galvyn::get("/openapi.json")]
pub async fn openapi() -> Response {
    Json(galvyn::openapi::get_openapi()).into_response()
}
