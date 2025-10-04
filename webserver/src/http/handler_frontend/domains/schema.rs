use galvyn::rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::models::domain::DomainUuid;

/// The representation of a domain
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Domain {
    /// Internal identifier of the domain
    pub uuid: DomainUuid,
    /// The domain
    pub domain: MaxStr<255>,
}

impl From<crate::models::domain::Domain> for Domain {
    fn from(domain: crate::models::domain::Domain) -> Self {
        Self {
            uuid: domain.uuid,
            domain: domain.domain,
        }
    }
}
