//! Schema for the settings

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

/// Schema for the common available settings
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SettingsSchema {
    /// Mailcow URL
    pub mailcow_url: Url,
}
