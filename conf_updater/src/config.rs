//! Definitions of the configuration file

use std::fs;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;

use rorm::DatabaseDriver;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use url::Url;

/// Server related configuration
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ServerConfig {
    /// Address the API server should bind to
    pub listen_address: String,
    /// Port the API server should bind to
    pub listen_port: u16,
    /// Pre-shared secret that allows access to the server API
    pub(crate) api_token: String,
}

/// Miscellaneous configuration used for this server
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MiscConfig {
    /// List of valid global IPv4 addresses of this server
    pub(crate) global_ipv4: Vec<Ipv4Addr>,
    /// List of valid global IPv6 addresses of this server
    pub(crate) global_ipv6: Vec<Ipv6Addr>,
    /// Full HTTPS URL to push notifications to the manager API
    pub(crate) manager_push_url: Url,
    /// Nginx website configuration directory, defaults to `/etc/nginx/sites-enabled/`
    #[serde(default = "get_default_nginx_config_dir")]
    pub(crate) nginx_config_dir: String,
    /// Root directory for website files, defaults to `/var/www/html/`
    #[serde(default = "get_default_htdocs_root_dir")]
    pub(crate) htdocs_root_dir: String,
    /// UNIX group identification for the web server user, defaults to `www-data`
    #[serde(default = "get_default_nginx_group")]
    pub(crate) nginx_group: String,
}

fn get_default_nginx_config_dir() -> String {
    "/etc/nginx/sites-enabled/".to_string()
}

fn get_default_htdocs_root_dir() -> String {
    "/var/www/html/".to_string()
}

fn get_default_nginx_group() -> String {
    "www-data".to_string()
}

/// Certbot related configuration
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CertbotConfig {
    /// Acquire test certificates instead of real ones, defaults to `false` (= use real certs)
    #[serde(default = "get_default_test_certs")]
    pub(crate) test_certs: bool,
    /// Base directory for Let's Encrypt certificates, defaults to `/etc/letsencrypt/live/`
    #[serde(default = "get_default_lets_encrypt")]
    pub(crate) cert_dir: String,
}

fn get_default_test_certs() -> bool {
    false
}

fn get_default_lets_encrypt() -> String {
    "/etc/letsencrypt/live/".to_string()
}

/// Configuration about the hosting provider of this service
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HostingConfig {
    /// Name of the hosting provider
    pub name: String,
    /// Website of the hosting provider (should be a URL)
    pub website: String,
    /// Support e-mail address or help desk e-mail address
    pub help_address: String,
    /// URL of the location where users can log in to their webspace for content changes
    pub webspace_login: String,
}

/// Database configuration (only supports Postgres)
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
    /// Miscellaneous configuration
    pub misc: MiscConfig,
    /// Hosting provider configuration
    pub hosting: HostingConfig,
    /// Certbot configuration
    #[serde(default = "get_default_certbot_conf")]
    pub certbot: CertbotConfig,
}

fn get_default_certbot_conf() -> CertbotConfig {
    CertbotConfig {
        test_certs: get_default_test_certs(),
        cert_dir: get_default_lets_encrypt(),
    }
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
