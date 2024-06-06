//! # bnv-manager-v2

#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

use std::env;
use std::io;
use std::io::Write;

use clap::Parser;
use rorm::cli as rorm_cli;
use rorm::config::DatabaseConfig;
use rorm::Database;
use rorm::DatabaseConfiguration;
use tracing::instrument;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::Config;
use crate::global::ldap::GlobalLdap;
use crate::global::ws::GlobalWs;
use crate::global::GlobalEntities;
use crate::global::GLOBAL;
use crate::models::User;

mod cli;
pub mod config;
pub mod global;
pub mod http;
pub mod models;
pub mod utils;

#[instrument(skip_all)]
async fn start(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the database
    let mut conf = DatabaseConfiguration::new(config.database.clone().into());
    conf.disable_logging = Some(true);
    let db = Database::connect(conf).await?;

    let ws = GlobalWs::new();

    let ldap = GlobalLdap::new(config.ldap.clone()).await?;

    // Initialize Globals
    GLOBAL.init(GlobalEntities { db, ws, ldap });

    // Start the webserver
    http::server::run(config).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let config = Config::try_from_path(&cli.config_path)?;

    match cli.command {
        Command::Start => start(&config).await?,
        #[cfg(debug_assertions)]
        Command::MakeMigrations { migrations_dir } => {
            use std::io::Write;

            const MODELS: &str = ".models.json";

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
            rorm_cli::migrate::run_migrate_custom(
                DatabaseConfig {
                    driver: config.database.into(),
                    last_migration_table_name: None,
                },
                migrations_dir,
                false,
                None,
            )
            .await?
        }
        Command::CreateUser => {
            // Connect to the database
            let mut conf = DatabaseConfiguration::new(config.database.clone().into());
            conf.disable_logging = Some(true);
            let db = Database::connect(conf).await?;

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

    #[allow(clippy::unwrap_used)]
    let password = rpassword::prompt_password("Enter password: ").unwrap();

    User::create_internal(username.to_string(), password, display_name, &db)
        .await
        .map_err(|e| format!("Failed to create user: {e}"))?;

    println!("Created user {username}");

    db.close().await;

    Ok(())
}
