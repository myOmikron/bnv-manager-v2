use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: MaxStr<64>,
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub nonce: Option<String>,
    #[serde(flatten)]
    pub email_claim: Option<EmailClaim>,
    #[serde(flatten)]
    pub profile_claim: Option<ProfileClaim>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmailClaim {
    pub email: String,
    pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProfileClaim {
    pub preferred_username: String,
    pub name: String,
}
