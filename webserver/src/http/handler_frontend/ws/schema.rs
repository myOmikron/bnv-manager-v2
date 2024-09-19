//! The schema for the websocket connection

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::global::webconf_updater::WebconfUpdateResult;

/// The result of a resolver
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ResolveResult {
    /// Ipv4 address
    pub ipv4: Option<Ipv4Addr>,
    /// Ipv6 address
    pub ipv6: Option<Ipv6Addr>,
}

/// The result from a dns query
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct DnsQueryResult {
    /// The uuid of the domain
    pub uuid: Uuid,
    /// The query result
    pub result: ResolveResult,
}

/// Websocket messages that originate from the server
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum WsServerMsg {
    /// Internal use only.
    ///
    /// This variant is used to close the websocket connection
    #[serde(skip_serializing, skip_deserializing)]
    Close,

    /// Deployment state has updated
    DeployUpdate {
        /// The task uuid
        task: Uuid,
        /// The state of the update
        state: WebconfUpdateResult,
    },

    /// DNS query update
    DnsUpdate {
        /// The task uuid
        task: Uuid,
        /// The dns result
        result: DnsQueryResult,
    },

    /// DNS task finished
    DnsFinished {
        /// The task uuid
        task: Uuid,
    },
}

/// Websocket messages that originate from the client
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum WsClientMsg {}
