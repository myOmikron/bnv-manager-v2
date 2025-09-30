//! Schema for token requests

use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Request for a token
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TokenRequest {
    /// Type of the grant
    pub grant_type: String,
    /// Code to exchange for a token
    pub code: MaxStr<64>,
    /// Redirect url of the initial request
    pub redirect_uri: String,
}

/// Token response
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TokenResponse {
    /// Access token
    pub access_token: String,
    /// Access token
    pub id_token: String,
    /// Type of the token
    pub token_type: String,
    /// Expires in
    pub expires_in: usize,
}

/// Data for all claims
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Claims {
    /// Issuer
    pub iss: String,
    /// Identifier for the End-User
    pub sub: String,
    /// Audience
    pub aud: String,
    /// Expiry time
    pub exp: usize,
    /// Time at which the JWT was issued
    pub iat: usize,
    /// Optional nonce
    pub nonce: Option<String>,
    /// Optional email claims
    #[serde(flatten)]
    pub email_claim: Option<EmailClaim>,
    /// Optional profile claims
    #[serde(flatten)]
    pub profile_claim: Option<ProfileClaim>,
}

/// Data for the email scope
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmailClaim {
    /// The email
    pub email: String,
    /// Whether the mail is verified
    pub email_verified: bool,
}

/// Data for the profile scope
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProfileClaim {
    /// Preferred username
    pub preferred_username: String,
    /// Name of the user
    pub name: String,
}
