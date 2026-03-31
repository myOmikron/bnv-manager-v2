use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::rorm::fields::types::MaxStr;
use serde::Deserialize;
use serde::Serialize;

use crate::models::domain::DomainUuid;
use crate::models::mail_alias::MailAlias;
use crate::models::mail_alias::MailAliasStatus;
use crate::models::mail_alias::MailAliasUuid;

/// API representation of a mail alias
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MailAliasSchema {
    /// UUID of the alias
    pub uuid: MailAliasUuid,
    /// Local part of the alias (before the @)
    pub local_part: MaxStr<255>,
    /// Domain name
    pub domain: String,
    /// Full alias address
    pub full_address: String,
    /// Display name of the owning account
    pub account_display_name: String,
    /// Status of the alias
    pub status: MailAliasStatus,
    /// Point in time the alias was created
    pub created_at: SchemaDateTime,
}

/// Domain available for alias creation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DomainOptionSchema {
    /// UUID of the domain
    pub uuid: DomainUuid,
    /// Domain name
    pub domain: String,
}

/// Request to propose a new alias
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProposeAliasRequest {
    /// Local part of the alias (before the @)
    pub local_part: MaxStr<255>,
    /// UUID of the domain to use
    pub domain_uuid: DomainUuid,
}

/// Errors that can occur while proposing an alias
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProposeAliasError {
    /// The alias address is already taken
    pub alias_already_taken: bool,
    /// The chosen domain does not belong to the member's club
    pub domain_not_in_club: bool,
    /// The local part contains invalid characters
    pub invalid_local_part: bool,
}

impl MailAliasSchema {
    /// Create a schema from a model and domain/account display names
    pub fn new(alias: MailAlias, domain_name: String, account_display_name: String) -> Self {
        let full_address = format!("{}@{}", &*alias.local_part, &domain_name);
        Self {
            uuid: alias.uuid,
            local_part: alias.local_part,
            domain: domain_name,
            full_address,
            account_display_name,
            status: alias.status,
            created_at: SchemaDateTime(alias.created_at),
        }
    }
}
