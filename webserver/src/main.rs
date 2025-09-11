//! # bnv-manager-v2

#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

use std::error::Error;

use ::tracing::error;
use clap::Parser;
use galvyn::Galvyn;
use galvyn::GalvynSetup;
use galvyn::core::DatabaseSetup;
use galvyn::rorm::Database;
use rorm::DatabaseConfiguration;
use rorm::cli as rorm_cli;
use rorm::fields::types::MaxStr;
use time::Duration;
use time::OffsetDateTime;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::DB;
use crate::models::invite::CreateInviteParams;
use crate::models::invite::Invite;
use crate::models::role::Role;
use crate::modules::garbage_collector::GarbageCollector;
use crate::modules::mailcow::Mailcow;
use crate::modules::oidc::Oidc;
use crate::tracing::opentelemetry_layer;
use crate::utils::links::Link;

mod cli;
pub mod config;
pub mod http;
pub mod models;
pub mod modules;
pub mod tracing;
pub mod utils;

async fn start() -> Result<(), Box<dyn Error>> {
    let router = Galvyn::builder(GalvynSetup::default())
        .register_module::<Database>(DatabaseSetup::Custom(DatabaseConfiguration::new(
            DB.clone(),
        )))
        .register_module::<GarbageCollector>(())
        .register_module::<Mailcow>(())
        .register_module::<Oidc>(())
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
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("DEBUG")))
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
            .await?;
        }
        Command::CreateAdmin {
            username,
            display_name,
        } => {
            let username = MaxStr::new(username)?;
            let display_name = MaxStr::new(display_name)?;

            let db = Database::connect(DatabaseConfiguration::new(DB.clone())).await?;
            let res = Invite::create(
                &db,
                CreateInviteParams {
                    username,
                    display_name,
                    roles: vec![Role::SuperAdmin],
                    expires_at: OffsetDateTime::now_utc() + Duration::minutes(15),
                },
            )
            .await?;

            db.close().await;

            match res {
                Ok(invite) => {
                    let link = Link::invite(invite.uuid);
                    println!("Invite link: {}", link);
                }
                Err(err) => {
                    error!("{err}");
                    return Err(err.into());
                }
            }
        }
    }

    Ok(())
}
