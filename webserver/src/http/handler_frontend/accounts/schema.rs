use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

use crate::models::account::AccountUuid;
use crate::models::account::AdministrativeAccount;
use crate::models::account::ClubAccount;
use crate::models::account::ClubAdminAccount;
use crate::models::credential_reset::CredentialReset;
use crate::models::credential_reset::CredentialResetUuid;

/// Simple representation of an account.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleAccountSchema {
    /// The account's UUID.
    pub uuid: AccountUuid,
    /// The account's username.
    pub username: MaxStr<255>,
    /// The account's display name.
    pub display_name: MaxStr<255>,
}

/// Simple representation of an account.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleMemberAccountSchema {
    /// The account's UUID.
    pub uuid: AccountUuid,
    /// The account's username.
    pub username: MaxStr<255>,
    /// The account's display name.
    pub display_name: MaxStr<255>,
    /// The account's email
    pub email: MaxStr<255>,
}

/// Instance of the credential reset
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CredentialResetSchema {
    /// Identifier
    pub uuid: CredentialResetUuid,
    /// Point in time the reset expires
    pub expires_at: SchemaDateTime,
    /// The link to give to the user
    pub link: Url,
}

impl From<CredentialReset> for CredentialResetSchema {
    fn from(value: CredentialReset) -> Self {
        Self {
            uuid: value.uuid,
            expires_at: SchemaDateTime(value.expires_at),
            link: value.link(),
        }
    }
}

impl From<AdministrativeAccount> for SimpleAccountSchema {
    fn from(value: AdministrativeAccount) -> Self {
        Self {
            uuid: value.uuid(),
            username: value.username,
            display_name: value.display_name,
        }
    }
}

impl From<ClubAdminAccount> for SimpleAccountSchema {
    fn from(value: ClubAdminAccount) -> Self {
        Self {
            uuid: value.uuid(),
            username: value.username,
            display_name: value.display_name,
        }
    }
}

impl From<ClubAccount> for SimpleMemberAccountSchema {
    fn from(value: ClubAccount) -> Self {
        Self {
            uuid: value.uuid(),
            username: value.username,
            display_name: value.display_name,
            email: value.email,
        }
    }
}
