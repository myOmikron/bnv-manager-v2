//! Schema for mailcow domain endpoints

use serde::Deserialize;
use serde::Serialize;

/// A domain in mailcow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailcowDomain {
    /// Integer to show if the domain is enabled
    pub active_int: u8,
    /// Domain
    pub domain_name: String,
    /// Number of mailboxes left to create on the domain
    pub mboxes_left: u64,
    /// Maximal quota for a mailbox
    pub max_quota_for_mbox: u64,
}
