//! Common schemas in the API

use schemars::JsonSchema;
use schemars::JsonSchema_repr;
use serde::Deserialize;
use serde::Serialize;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use uuid::Uuid;

/// a uuid
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct UuidSchema {
    /// The uuid
    pub uuid: Uuid,
}

/// The Status code that are returned throughout the API
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, JsonSchema_repr)]
#[repr(u16)]
#[allow(missing_docs)]
pub enum ApiStatusCode {
    Unauthenticated = 1000,
    BadRequest = 1001,
    MissingPrivileges = 1002,
    ResourceNotFound = 1003,

    InternalServerError = 2000,
}

/// The response that is sent in a case of an error
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[allow(missing_docs)]
pub struct ApiErrorResponse {
    /// The Status code for the error.
    ///
    /// Important: Does not match http status codes
    pub status_code: ApiStatusCode,
    /// A human-readable error message.
    ///
    /// May be used for displaying purposes
    pub message: String,
}

/// An error in a form field
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FormFieldError<T> {
    /// The corresponding field
    pub field: T,
}

/// The response that should be used for inform the user about errors in the form
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FormError<T> {
    /// The errors that occurred
    pub errors: Vec<FormFieldError<T>>,
}

impl<T> FormError<T> {
    /// A single form error
    pub fn single(field: T) -> Self {
        Self {
            errors: vec![FormFieldError { field }],
        }
    }
}
