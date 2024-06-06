//! Definitions of the configuration file

use std::fs;
use std::io;
use std::path::Path;

use rorm::DatabaseDriver;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

/// Server related configuration.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ServerConfig {
    /// Address the API server should bind to
    pub listen_address: String,
    /// Port the API server should bind to
    pub listen_port: u16,
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
    pub password: String,
}

impl From<DBConfig> for DatabaseDriver {
    fn from(value: DBConfig) -> Self {
        DatabaseDriver::Postgres {
            name: value.name,
            host: value.host,
            port: value.port,
            user: value.user,
            password: value.password,
        }
    }
}

/// LDAP related configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LdapConfig {
    /// Connection URI
    pub uri: String,
    /// The DN to use for binding
    pub admin_bind_dn: String,
    /// The Password for the DN
    pub admin_bind_pw: String,
    /// User search base
    pub user_search_base: String,
    /// User search filter
    pub user_search_filter: String,
    /// Use start tls
    pub start_tls: bool,
    /// Do not verify TLS certificates
    pub no_tls_verify: bool,
}

/// Definition of the main configuration.
///
/// This model can be parsed from the config.toml
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    /// Server configuration
    pub server: ServerConfig,
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
