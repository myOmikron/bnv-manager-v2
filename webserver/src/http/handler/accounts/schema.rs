use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::models::account::AccountUuid;

/// Simple representation of an account.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleAccount {
    /// The account's UUID.
    pub uuid: AccountUuid,
    /// The account's username.
    pub username: MaxStr<255>,
    /// The account's display name.
    pub display_name: MaxStr<255>,
}
