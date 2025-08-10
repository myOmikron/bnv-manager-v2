//! # bnv-manager-v2

#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

use std::error::Error;

use clap::Parser;
use galvyn::Galvyn;
use galvyn::core::DatabaseSetup;
use galvyn::rorm::Database;
use rorm::DatabaseConfiguration;
use rorm::cli as rorm_cli;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::DB;
use crate::tracing::opentelemetry_layer;

mod cli;
pub mod config;
pub mod http;
pub mod models;
pub mod tracing;
pub mod utils;

async fn start() -> Result<(), Box<dyn Error>> {
    let router = Galvyn::new()
        .register_module::<Database>(DatabaseSetup::Custom(DatabaseConfiguration::new(
            DB.clone(),
        )))
        .init_modules()
        .await?;

    http::server::run(router).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(errors) = config::load_env() {
        for error in errors {
            eprintln!("{error}");
        }
        return Err("Failed to load configuration".into());
    }

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO")))
        .with(tracing_forest::ForestLayer::default())
        .with(opentelemetry_layer()?)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Start => start().await?,
        #[cfg(debug_assertions)]
        Command::MakeMigrations { migrations_dir } => {
            use std::io::Write;

            const MODELS: &str = "/tmp/.models.json";

            let mut file = std::fs::File::create(MODELS)?;
            rorm::write_models(&mut file)?;
            file.flush()?;

            rorm_cli::make_migrations::run_make_migrations(
                rorm_cli::make_migrations::MakeMigrationsOptions {
                    models_file: MODELS.to_string(),
                    migration_dir: migrations_dir,
                    name: None,
                    non_interactive: false,
                    warnings_disabled: false,
                },
            )?;

            std::fs::remove_file(MODELS)?;
        }
        Command::Migrate { migrations_dir } => {
            rorm::cli::migrate::run_migrate_custom(
                rorm::config::DatabaseConfig {
                    driver: DB.clone(),
                    last_migration_table_name: None,
                },
                migrations_dir,
                false,
                None,
            )
            .await?
        }
    }

    Ok(())
}
