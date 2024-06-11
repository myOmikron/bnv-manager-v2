//! # Website Configuration Updater

use std::env;

use clap::Parser;
use rorm::config::DatabaseConfig;

use crate::cli::{Cli, Command};
use crate::config::Config;

mod cli;
mod config;
mod handler;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let config = Config::try_from_path(&cli.config_path)?;

    match cli.command {
        Command::Start => server::start(config).await?,

        #[cfg(debug_assertions)]
        Command::MakeMigrations { migrations_dir } => {
            use std::io::Write;

            const MODELS: &str = ".models.json";

            let mut file = std::fs::File::create(MODELS)?;
            rorm::write_models(&mut file)?;
            file.flush()?;

            rorm::cli::make_migrations::run_make_migrations(
                rorm::cli::make_migrations::MakeMigrationsOptions {
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
    }

    Ok(())
}
