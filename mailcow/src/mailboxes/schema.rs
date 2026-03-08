//! Schema for interacting with mailboxes

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

/// Create a new app password
pub struct CreateAppPasswordRequest {
    /// Username of the mailbox
    pub username: String,
    /// Name of the app
    pub app_name: String,
    /// Password of the app
    pub app_passwd: String,
    /// Confirmation of the password
    pub app_passwd2: String,
}

/// Options for get app password responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetAppPasswordsResponse {
    /// List of app passwords
    List(Vec<GetAppPasswordSingleResponse>),
    /// Empty response
    Empty(HashMap<String, String>),
}

/// Response for a single app password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppPasswordSingleResponse {
    /// ID of the app password
    pub id: u64,
    /// Name of the app password
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InnerCreateAppPasswordRequest {
    pub active: String,
    pub username: String,
    pub app_name: String,
    pub app_passwd: String,
    pub app_passwd2: String,
    pub protocols: Vec<String>,
}
