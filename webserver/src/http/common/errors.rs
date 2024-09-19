//! This module holds the errors and the error conversion for handlers
//! that are returned from handlers

use std::fmt;
use std::panic::Location;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use schemars::JsonSchema;
use serde::Serialize;
use swaggapi::as_responses::simple_responses;
use swaggapi::as_responses::AsResponses;
use swaggapi::as_responses::SimpleResponse;
use swaggapi::internals::SchemaGenerator;
use swaggapi::re_exports::mime;
use swaggapi::re_exports::openapiv3;
use swaggapi::re_exports::openapiv3::MediaType;
use swaggapi::re_exports::openapiv3::Responses;
use thiserror::Error;
use tracing::error;

use crate::http::common::schemas::ApiErrorResponse;
use crate::http::common::schemas::ApiStatusCode;
use crate::http::common::schemas::FormError;

/// A type alias that includes the ApiError
pub type ApiResult<T> = Result<T, ApiError>;

/// The type alias for providing more information about invalid form states
///
/// This should be nested in [ApiResult]
pub type FormResult<Ok, Err> = Result<Ok, FormError<Err>>;

/// The common error that is returned from the handlers
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ApiError {
    #[error("Unauthenticated")]
    Unauthenticated,
    #[error("Bad request")]
    BadRequest,
    #[error("Missing privileges")]
    MissingPrivileges,
    #[error("Resource not found")]
    ResourceNotFound,

    #[error("An internal server error occurred")]
    InternalServerError,
}

impl ApiError {
    /// Ad-hoc constructor for an internal server error
    /// where the causing `error` doesn't have a proper `From` impl.
    // This function takes `impl Debug + Display` instead of `impl Error` to also support strings
    #[track_caller]
    pub fn new_internal_server_error(error: impl fmt::Debug + fmt::Display) -> Self {
        log_internal_server_error(error, Location::caller());
        Self::InternalServerError
    }
}

impl<T> IntoResponse for FormError<T>
where
    T: JsonSchema + Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ApiError::Unauthenticated => ApiStatusCode::Unauthenticated,
            ApiError::BadRequest => ApiStatusCode::BadRequest,
            ApiError::MissingPrivileges => ApiStatusCode::MissingPrivileges,
            ApiError::InternalServerError => ApiStatusCode::InternalServerError,
            ApiError::ResourceNotFound => ApiStatusCode::ResourceNotFound,
        };

        let res = (
            if (status_code as u16) < 2000 {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            Json(ApiErrorResponse {
                status_code,
                message: self.to_string(),
            }),
        );

        res.into_response()
    }
}

impl AsResponses for ApiError {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        let media_type = Some(MediaType {
            schema: Some(gen.generate::<ApiErrorResponse>()),
            ..Default::default()
        });

        simple_responses([
            SimpleResponse {
                status_code: openapiv3::StatusCode::Code(400),
                mime_type: mime::APPLICATION_JSON,
                description: "Client side error".to_string(),
                media_type: media_type.clone(),
            },
            SimpleResponse {
                status_code: openapiv3::StatusCode::Code(500),
                mime_type: mime::APPLICATION_JSON,
                description: "Server side error".to_string(),
                media_type,
            },
        ])
    }
}

impl<T> AsResponses for FormError<T>
where
    T: JsonSchema,
{
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        let media_type = Some(MediaType {
            schema: Some(gen.generate::<FormError<T>>()),
            ..Default::default()
        });

        simple_responses([SimpleResponse {
            status_code: openapiv3::StatusCode::Code(200),
            mime_type: mime::APPLICATION_JSON,
            description: "Form field error".to_string(),
            media_type,
        }])
    }
}

/// Simple macro to reduce the noise of several identical `From` implementations
///
/// It takes a list of error types
/// which are supposed to be convertable into an [`ApiError::InternalServerError`] simplicity.
macro_rules! impl_into_internal_server_error {
    ($($error:ty,)*) => {$(
        impl From<$error> for ApiError {
            #[track_caller]
            fn from(value: $error) -> Self {
                log_internal_server_error(value, Location::caller());
                Self::InternalServerError
            }
        }
    )+};
}
impl_into_internal_server_error!(
    rorm::Error,
    argon2::password_hash::Error,
    tower_sessions_rorm_store::tower_sessions::session::Error,
);
/// Used by [`impl_into_internal_server_error`]'s `From` impls and [`ApiError::new_internal_server_error`]
/// to log the errors converted into an [`ApiError::InternalServerError`].
///
/// This function serves as an easy to find and understand place for tweaking the log's format.
///
/// It takes `impl Debug + Display` instead of `impl Error` to also support strings.
fn log_internal_server_error(error: impl fmt::Debug + fmt::Display, location: &Location) {
    error!(
        error.display = %error,
        error.debug = ?error,
        error.file = location.file(),
        error.line = location.line(),
        error.column = location.column(),
        "Internal server error",
    );
}
