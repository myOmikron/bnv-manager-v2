use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

#[galvyn::get("/openapi.json")]
pub async fn openapi() -> Response {
    Json(galvyn::openapi::get_openapi()).into_response()
}
