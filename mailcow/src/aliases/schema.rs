//! Schema for mailcow alias endpoints

use serde::Deserialize;
use serde::Serialize;

/// An alias in mailcow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailcowAlias {
    /// ID of the alias
    pub id: u64,
    /// The alias address
    pub address: String,
    /// The target address (goto)
    pub goto: String,
    /// Whether the alias is active
    pub active_int: u8,
}

/// Request to create a new alias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAliasRequest {
    /// The alias address (e.g. alias@domain.com)
    pub address: String,
    /// The target address (e.g. user@domain.com)
    pub goto: String,
    /// Whether the alias is active ("1" or "0")
    pub active: String,
}
