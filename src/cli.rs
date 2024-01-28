//! The CLI is defined in this module

use clap::{Parser, Subcommand};

/// CLI interface of bnv-manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The path to the configuration file of the bnv-manager
    #[arg(long, default_value_t = String::from("/etc/bnv-manager/config.toml"))]
    pub config_path: String,

    /// The available commands of bnv-manager
    #[command(subcommand)]
    pub command: Command,
}

/// All available commands of bnv-manager
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the webserver
    Start,
    /// Apply migrations to the database
    Migrate {
        /// The path to the migration directory
        migration_dir: String,
    },
}
