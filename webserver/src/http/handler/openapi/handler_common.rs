//! Common handlers of the openapi

use galvyn::core::re_exports::axum::Json;
use galvyn::core::re_exports::axum::response::IntoResponse;
use galvyn::core::re_exports::axum::response::Response;
use galvyn::get;
use tracing::instrument;

/// Generate the openapi definition
#[get("/openapi.json")]
#[instrument(name = "Api::openapi")]
pub async fn openapi() -> Response {
    Json(galvyn::openapi::get_openapi()).into_response()
}
