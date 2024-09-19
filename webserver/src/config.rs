//! Definitions of the configuration file

use std::fs;
use std::io;
use std::net::IpAddr;
use std::path::Path;

use rorm::DatabaseDriver;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use url::Url;

use crate::utils::secure_string::SecureString;

/// Server related configuration.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HttpConfig {
    /// Address the API server should bind to
    pub listen_address: String,
    /// Port the API server should bind to
    pub listen_port: u16,
    /// The url of the webconf updater
    pub webconf_updater_url: Url,
    /// The token of the webconf updater
    pub webconf_updater_token: String,
}

/// Database related configuration.
///
/// As the only supported database is postgres, no driver configuration is needed
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DBConfig {
    /// The address of the database server
    pub host: String,
    /// The port of the database server
    pub port: u16,
    /// The database name
    pub name: String,
    /// The user to use for the database connection
    pub user: String,
    /// Password for the user
    pub password: SecureString,
}

impl From<DBConfig> for DatabaseDriver {
    fn from(value: DBConfig) -> Self {
        DatabaseDriver::Postgres {
            name: value.name,
            host: value.host,
            port: value.port,
            user: value.user,
            password: value.password.into_inner(),
        }
    }
}

/// LDAP server configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LdapConfig {
    /// The address the ldap server should listen on
    pub listen_address: IpAddr,
    /// The port the ldap server should listen on
    pub listen_port: u16,
}

/// Definition of the main configuration.
///
/// This model can be parsed from the config.toml
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    /// HTTP Server configuration
    pub http: HttpConfig,
    /// Database configuration
    pub database: DBConfig,
    /// The config for ldap
    pub ldap: LdapConfig,
}

/// All errors that can occur when parsing a configuration file
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ConfigError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("The config file is missing")]
    MissingFile,
    #[error("Parsing of config file failed: {0}")]
    ParsingFailed(#[from] toml::de::Error),
}

impl Config {
    /// Read and parse a [Config] from a path
    pub fn try_from_path(path: &str) -> Result<Self, ConfigError> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(ConfigError::MissingFile);
        }

        let c_str = fs::read_to_string(p)?;
        let config = toml::from_str(&c_str)?;

        Ok(config)
    }
}
