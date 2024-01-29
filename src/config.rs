//! The configuration definition of bnv-manager is defined in this module

use std::net::IpAddr;
use std::num::NonZeroU16;
use std::path::Path;
use std::{fs, io};

use rorm::DatabaseDriver;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

/// The top-level config of the bnv-manager
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(missing_docs)]
pub struct Config {
    pub database: DatabaseConfig,
    pub tracing: TracingConfig,
    pub server: ServerConfig,
    pub ldap: LdapConfig,
}

/// The configuration for the webserver
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerConfig {
    /// The address to listen on
    pub listen_address: IpAddr,
    /// The port to listen on
    pub listen_port: NonZeroU16,
}

/// The configuration of the ldap connection
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LdapConfig {
    /// The ldap uri
    pub uri: Url,
    /// The distinguished name that should be used for binding
    pub bind_dn: String,
    /// The password that should be used for binding
    pub bind_password: String,
    /// Don't verify certificates
    pub do_not_verify_certs: Option<bool>,
}

/// The configuration for tracing capabilities
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TracingConfig {
    /// Endpoint for connection to the open telemetry receiver
    pub oltp_endpoint: Url,
}

/// Database related configuration
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    /// The name of the database to connect to
    pub name: String,
    /// The user to use for connection
    pub user: String,
    /// The password to use for connection
    pub password: String,
    /// Host to connect to
    pub host: String,
    /// Port to connect to
    pub port: NonZeroU16,
}

/// The errors that can occur while parsing a configuration
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum ConfigError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Config path not found")]
    ConfigNotFound,
    #[error("Error deserializing config: {0}")]
    Deserialize(#[from] toml::de::Error),
}

impl Config {
    /// Retrieve the configuration
    pub fn from_path(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
        let p = path.as_ref();

        if !p.exists() {
            return Err(ConfigError::ConfigNotFound);
        }

        let config_str = fs::read_to_string(p)?;
        let config = toml::from_str(&config_str)?;

        Ok(config)
    }
}

impl From<&DatabaseConfig> for DatabaseDriver {
    fn from(config: &DatabaseConfig) -> Self {
        Self::Postgres {
            name: config.name.clone(),
            host: config.host.clone(),
            port: config.port.get(),
            user: config.user.clone(),
            password: config.password.clone(),
        }
    }
}
