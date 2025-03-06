use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
pub struct AcceptInviteRequest {
    pub password: MaxStr<72>,
}
