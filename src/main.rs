//! # bnv-manager-v2
//!
//! The second version of the bnv-manager

#![warn(missing_docs, clippy::expect_used, clippy::unwrap_used)]

use std::error::Error;

use clap::Parser;
use rorm::cli::migrate;
use rorm::config::DatabaseConfig;
use rorm::{Database, DatabaseConfiguration, DatabaseDriver};
use tracing::debug;

use crate::cli::{Cli, Command};
use crate::config::Config;
use crate::global::{Globals, GLOBAL};
use crate::modules::ldap::LdapConn;
use crate::server::start_server;

pub mod cli;
pub mod config;
pub mod global;
pub mod models;
pub mod modules;
mod server;
mod trace;

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config = Config::from_path(&cli.config_path)?;

    trace::init_trace_oltp(&config.tracing.oltp_endpoint)?;

    let res = startup(cli, config).await;

    opentelemetry::global::shutdown_tracer_provider();

    res
}

#[tracing::instrument(level = "debug", skip_all)]
async fn startup(cli: Cli, config: Config) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Command::Start => {
            debug!("Begin startup");

            let db = get_database(&config).await?;
            debug!("Connected to database");

            let ldap = LdapConn::new(
                &config.ldap.uri,
                config.ldap.bind_dn.clone(),
                config.ldap.bind_password.clone(),
                config.ldap.do_not_verify_certs.unwrap_or_default(),
            )
            .await?;

            GLOBAL.init(Globals { db, ldap });
            debug!("Initialized globals");

            start_server(&config).await?;
        }
        Command::Migrate { migration_dir } => run_migrate(migration_dir, &config).await?,
    }

    Ok(())
}

async fn get_database(config: &Config) -> Result<Database, rorm::Error> {
    Database::connect(DatabaseConfiguration {
        driver: DatabaseDriver::from(&config.database),
        min_connections: 2,
        max_connections: 20,
        disable_logging: Some(true),
        statement_log_level: None,
        slow_statement_log_level: None,
    })
    .await
}

async fn run_migrate(migration_dir: String, config: &Config) -> Result<(), Box<dyn Error>> {
    migrate::run_migrate_custom(
        DatabaseConfig {
            driver: DatabaseDriver::from(&config.database),
            last_migration_table_name: None,
        },
        migration_dir,
        false,
        None,
    )
    .await?;

    Ok(())
}
