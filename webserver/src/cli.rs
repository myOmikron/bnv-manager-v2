//! Definitions of the CLI arguments

use clap::Parser;
use clap::Subcommand;

/// The cli
#[derive(Parser)]
pub struct Cli {
    /// The available subcommands
    #[clap(subcommand)]
    pub command: Command,
}

/// All available commands
#[derive(Subcommand)]
pub enum Command {
    /// Start the server
    Start,
    /// Run the migrations on the database
    Migrate {
        /// The directory where the migration files are located in
        #[clap(default_value_t = String::from("/migrations"))]
        migrations_dir: String,
    },
    /// Create new migrations
    #[cfg(debug_assertions)]
    MakeMigrations {
        /// The directory where the migration files are located in
        #[clap(default_value_t = String::from("/migrations"))]
        migrations_dir: String,
    },
    /// Create a superadmin account
    CreateAdmin {
        /// The username that should be set for the new account
        username: String,
        /// The display name for the user
        display_name: String,
    },
}
