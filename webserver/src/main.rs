//! # bnv-manager-v2

#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

use std::env;
use std::io;
use std::io::Write;
use std::sync::Arc;

use clap::Parser;
use rorm::cli as rorm_cli;
use rorm::Database;
use rorm::DatabaseConfiguration;
use tracing::instrument;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::Config;
use crate::global::dns::GlobalDns;
use crate::global::webconf_updater::GlobalWebconfUpdater;
use crate::global::ws::GlobalWs;
use crate::global::GlobalEntities;
use crate::global::GLOBAL;
use crate::models::User;
use crate::models::UserRole;

mod cli;
pub mod config;
pub mod global;
pub mod http;
pub mod ldap;
mod migrate;
pub mod models;
pub mod utils;

#[instrument(skip_all)]
async fn start(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the database
    let mut conf = DatabaseConfiguration::new(config.database.clone().into());
    conf.disable_logging = Some(true);
    let db = Database::connect(conf).await?;

    let ws = GlobalWs::new();

    let dns = GlobalDns::new();

    let webconf_updater = GlobalWebconfUpdater::new(
        config.http.webconf_updater_url.clone(),
        config.http.webconf_updater_token.clone(),
    );

    // Initialize Globals
    GLOBAL.init(GlobalEntities {
        db,
        ws,
        dns,
        conf: config.clone(),
        webconf_updater,
    });

    let c = Arc::new(config.clone());

    // Start the ldap server
    ldap::server::start_server(c.clone()).await?;

    // Start the webserver
    http::server::run(c).await?;

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
            migrate::migrate(config.database.clone(), migrations_dir).await?
        }
        Command::CreateAdmin => {
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

    User::create_user(
        username.to_string(),
        password,
        display_name,
        UserRole::Administrator,
        "EN".to_string(),
        &db,
    )
    .await
    .map_err(|e| format!("Failed to create user: {e}"))?;

    println!("Created user {username}");

    db.close().await;

    Ok(())
}
