use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rorm::query;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

use crate::global::GLOBAL;
use crate::models::user::User;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error")]
    Database(rorm::Error),
}

impl From<rorm::Error> for ApiError {
    fn from(value: rorm::Error) -> Self {
        error!("Database error: {value}");
        Self::Database(value)
    }
}

#[derive(Serialize, Debug)]
pub struct ApiErrorResponse {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let res = match self {
            ApiError::Database(_) => Json(ApiErrorResponse {
                message: self.to_string(),
            })
            .into_response()
            .into_body(),
        };

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(res)
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Q {
    pub q: String,
}

#[tracing::instrument(ret)]
pub async fn test(q: Query<Q>) -> Result<String, ApiError> {
    let db = &GLOBAL.db.clone();

    query!(db, User).all().await?;

    Ok("Hello, World!".to_string())
}
