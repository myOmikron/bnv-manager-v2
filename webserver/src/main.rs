//! # bnv-manager-v2

#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

use std::error::Error;
use std::io;
use std::io::Write;

use clap::Parser;
use galvyn_core::re_exports::rorm::Database;
use galvyn_core::re_exports::rorm::DatabaseConfiguration;
use rorm::cli as rorm_cli;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::DB;
use crate::tracing::init_tracing_panic_hook;
use crate::tracing::opentelemetry_layer;

mod cli;
pub mod config;
pub mod http;
pub mod tracing;
pub mod utils;

async fn start() -> Result<(), Box<dyn Error>> {
    galvyn_core::module::registry::Registry::builder()
        .register_module::<Database>()
        .init()
        .await?;

    http::server::run().await?;

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

    let registry = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO")))
        .with(tracing_subscriber::fmt::layer());

    let registry = registry.with(opentelemetry_layer()?);

    registry.init();
    init_tracing_panic_hook();

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
        Command::CreateAdmin => {
            // Connect to the database
            let db = Database::connect(DatabaseConfiguration {
                ..DatabaseConfiguration::new(DB.clone())
            })
            .await?;

            create_user(db).await?;
        }
    }

    Ok(())
}

/// Creates a new user
///
/// **Parameter**:
/// - `db`: [Database]
// Unwrap is okay, as no handling of errors is possible if we can't communicate with stdin / stdout
async fn create_user(db: Database) -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut username = String::new();
    let mut display_name = String::new();

    print!("Enter a username: ");
    #[allow(clippy::unwrap_used)]
    stdout.flush().unwrap();
    #[allow(clippy::unwrap_used)]
    stdin.read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter a display name: ");
    #[allow(clippy::unwrap_used)]
    stdout.flush().unwrap();
    #[allow(clippy::unwrap_used)]
    stdin.read_line(&mut display_name).unwrap();
    let display_name = display_name.trim().to_string();

    println!("Created user {username}");

    db.close().await;

    Ok(())
}
