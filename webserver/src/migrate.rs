//! Implementation of [`Command::Migrate`](crate::cli::Command)

use rorm::config::DatabaseConfig;
use rorm::Database;
use rorm::DatabaseConfiguration;
use rorm::DatabaseDriver;
use tracing::instrument;

use crate::config::DBConfig;
use crate::utils::custom_db_enum;

/// Implementation of [`Command::Migrate`](crate::cli::Command)
#[instrument]
pub async fn migrate(
    db_config: DBConfig,
    migrations_dir: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_driver = DatabaseDriver::from(db_config);

    // Run rorm-cli
    rorm::cli::migrate::run_migrate_custom(
        DatabaseConfig {
            driver: db_driver.clone(),
            last_migration_table_name: None,
        },
        migrations_dir,
        false,
        None,
    )
    .await?;

    let db = Database::connect(DatabaseConfiguration::new(db_driver)).await?;
    let mut tx = db.start_transaction().await?;

    // Pre-populate custom db enums
    for migrate_db in custom_db_enum::MIGRATE_DB {
        migrate_db(&mut tx).await?;
    }

    tx.commit().await?;
    db.close().await;

    Ok(())
}
