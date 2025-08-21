use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub active: String,
    pub domain_name: String,
}
