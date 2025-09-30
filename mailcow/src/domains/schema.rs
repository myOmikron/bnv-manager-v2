use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub active_int: u8,
    pub domain_name: String,
    pub mboxes_left: u64,
    pub max_quota_for_mbox: u64,
}
