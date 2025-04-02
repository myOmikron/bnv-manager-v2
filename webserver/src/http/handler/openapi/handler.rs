use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;

#[galvyn::get("/openapi.json")]
pub async fn openapi() -> Response {
    Json(galvyn::openapi::get_openapi()).into_response()
}
