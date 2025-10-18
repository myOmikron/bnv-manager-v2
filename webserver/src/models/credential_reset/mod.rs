//! Models for credential resets

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::utils::links::Link;

pub(in crate::models) mod db;

/// Data model for handle credential resets
pub struct CredentialReset {
    /// UUID of the reset
    pub uuid: CredentialResetUuid,
    /// Point in time the reset expires
    pub expires_at: time::OffsetDateTime,
}

/// Wrapper for uuid
#[derive(Debug, Copy, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CredentialResetUuid(pub Uuid);

impl CredentialReset {
    /// Provide the constructed link the reset can be used at
    pub fn link(&self) -> Url {
        Link::reset_credentials(self.uuid)
    }
}
