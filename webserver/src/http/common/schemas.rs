//! Common schemas in the API

use schemars::JsonSchema;
use schemars::JsonSchema_repr;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use uuid::Uuid;

use crate::utils::checked_string::CheckedString;

/// A type without any runtime value
#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub enum Never {}

/// A single uuid wrapped in a struct
#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub struct SingleUuid {
    #[allow(missing_docs)]
    pub uuid: Uuid,
}

/// A single string representing a link wrapped in a struct
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SingleLink {
    #[allow(missing_docs)]
    pub link: String,
}

/// # Optional
/// A single field which might be `null`.
///
/// ## Rust Usage
///
/// If you want to return an `ApiJson<Option<T>>` from your handler,
/// please use `ApiJson<Optional<T>>` instead.
///
/// It simply wraps the option into a struct with a single field
/// to ensure the json returned from a handler is always an object.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub struct Optional<T> {
    #[allow(missing_docs)]
    pub optional: Option<T>,
}
impl<T> Optional<T> {
    /// Shorthand for `Optional { optional: Some(value) }`
    pub fn some(value: T) -> Self {
        Self {
            optional: Some(value),
        }
    }

    /// Shorthand for `Optional { optional: None }`
    pub fn none() -> Self {
        Self { optional: None }
    }
}
/// # List
/// A single field which is an array.
///
/// ## Rust Usage
///
/// If you want to return an `ApiJson<Vec<T>>` from your handler,
/// please use `ApiJson<List<T>>` instead.
///
/// It simply wraps the vector into a struct with a single field
/// to ensure the json returned from a handler is always an object.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct List<T> {
    #[allow(missing_docs)]
    pub list: Vec<T>,
}

/// The Status code that are returned throughout the API
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, JsonSchema_repr)]
#[repr(u16)]
#[allow(missing_docs)]
pub enum ApiStatusCode {
    Unauthenticated = 1000,
    BadRequest = 1001,
    InvalidJson = 1002,
    MissingPrivileges = 1003,

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

/// A `Result` with a custom serialization
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "result")]
#[allow(missing_docs)]
pub enum FormResult<T, E> {
    Ok { value: T },
    Err { error: E },
}
impl<T, E> FormResult<T, E> {
    /// Convenience function to construct a `FormResult::Ok`
    pub fn ok(value: T) -> Self {
        Self::Ok { value }
    }

    /// Convenience function to construct a `FormResult::Err`
    pub fn err(error: E) -> Self {
        Self::Err { error }
    }
}

/// A filename as query parameter.
///
/// This is most likly used for upload endpoints:
///     Since the entire request's body is used for the file's content,
///     the name has to be passed as query parameter.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FileNameQuery {
    /// The file's name
    pub file_name: CheckedString<1, 255>,
}

/// Custom deserializer to enable the distinction of missing keys vs null values in JSON requests
///
/// Annotate your field with: `
/// #[serde(default, deserialize_with = "deserialize_double_option", skip_if = "Option::is_none")]
/// `
///
/// # Example
/// ```rust
/// use webserver::http::common::schemas::deserialize_optional;
///
/// #[derive(serde::Serialize, serde::Deserialize)]
///  pub(crate) struct UpdateRequest {
///     name: Option<String>,
///
///     #[serde(default, deserialize_with = "deserialize_optional")] // Required for Deserialize
///     #[serde(skip_serializing_if = "Option::is_none")] // Required for Serialize
///     description: Option<Option<String>>,
/// }
/// ```
pub fn deserialize_optional<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(T::deserialize(d)?))
}
