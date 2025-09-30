//! Schema for domain-admin-related endpoints

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CreateDomainAdminRequest {
    pub active: u8,
    pub domains: Vec<String>,
    pub password: String,
    pub password2: String,
    pub username: String,
}
