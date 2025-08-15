use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Sign in request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SignInRequest {
    /// Username
    pub username: MaxStr<255>,
    /// Password
    pub password: MaxStr<72>,
}
