//! Common schemas and models used by the WebConf updater API

#[cfg(feature = "axum")]
use axum::http::StatusCode;
#[cfg(feature = "axum")]
use axum::Json;
#[cfg(feature = "axum")]
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

/// End user of the website configuration utility
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebsiteUser {
    /// Unique identifier of a website user
    pub id: Uuid,
    /// LDAP common name of a website user; must be unique across all users
    pub cn: String,
    /// LDAP distinguished name of a website user
    pub dn: String,
}

/// Request used to provision a single website in the web server configuration and with TLS certs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProvisioningRequest {
    pub website: Uuid,
    pub user: WebsiteUser,
    pub domains: Vec<String>,
    pub forwarded_domains: Vec<String>,
    pub test_certificate: Option<bool>,
}

/// Request to remove a website from the web server configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemovalRequest {
    pub website: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FailedDomain {
    pub domain: String,
    /// Human-readable error message, may not be suitable for end users
    pub message: String,
}

/// Potential errors as returned by the API endpoints of the web conf updater
#[derive(Error, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum ApiFailure {
    #[error("Missing authorization")]
    MissingAuthorization,
    #[error("Invalid authorization")]
    InvalidAuthorization,
    #[error("Wrong authorization")]
    WrongAuthorization,

    #[error("Invalid current nginx config: {0}")]
    InvalidCurrentNginxConfig(String),
    #[error("Invalid updated nginx config: {0}")]
    InvalidUpdatedNginxConfig(Uuid),
    #[error("Failed to reload web server: {0}")]
    FailedToReloadWebserver(String),

    /// Trying to resolve any of these domains did not yield the host's IP address
    #[error("Domain check failure")]
    DomainCheckFailure(Vec<FailedDomain>),
    /// Trying to GET any of these domains did not produce an HTTP result
    #[error("Webserver check failure")]
    WebserverCheckFailure(Vec<FailedDomain>),

    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServerError,
}

#[cfg(feature = "axum")]
impl IntoResponse for ApiFailure {
    fn into_response(self) -> Response {
        (
            match self {
                ApiFailure::MissingAuthorization => StatusCode::UNAUTHORIZED,
                ApiFailure::InvalidAuthorization => StatusCode::BAD_REQUEST,
                ApiFailure::WrongAuthorization => StatusCode::FORBIDDEN,
                ApiFailure::InvalidCurrentNginxConfig(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ApiFailure::InvalidUpdatedNginxConfig(_) => StatusCode::BAD_REQUEST,
                ApiFailure::FailedToReloadWebserver(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ApiFailure::DomainCheckFailure(_) => StatusCode::BAD_REQUEST,
                ApiFailure::WebserverCheckFailure(_) => StatusCode::BAD_REQUEST,
                ApiFailure::BadRequest(_) => StatusCode::BAD_REQUEST,
                ApiFailure::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Json(self),
        )
            .into_response()
    }
}

#[cfg(feature = "rorm")]
impl From<rorm::Error> for ApiFailure {
    fn from(err: rorm::Error) -> Self {
        tracing::event!(tracing::Level::ERROR, "{}", err);
        ApiFailure::InternalServerError
    }
}
