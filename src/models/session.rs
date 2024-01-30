//! Everything regarding sessions are defined in this module

use std::collections::HashMap;

use rorm::fields::types::Json;
use rorm::Model;
use time::OffsetDateTime;

/// A webserver session
#[derive(Model)]
pub struct Session {
    /// The ID of a session
    #[rorm(max_length = 255, primary_key)]
    pub session_id: String,

    /// The expiration time of the session
    pub expiration_time: OffsetDateTime,

    /// The data of the session
    pub data: Json<HashMap<String, serde_json::Value>>,
}
