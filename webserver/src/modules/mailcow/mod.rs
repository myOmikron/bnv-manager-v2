//! Mailcow API integration module for the galvyn framework
//!
//! This module provides a structured interface for interacting with the Mailcow email server API.
//! It implements the galvyn module trait system, enabling seamless integration into the framework's
//! initialization and dependency management pipeline.
//!
//! The module serves as the primary entry point for all Mailcow API operations within the application,
//! providing access to the underlying MailcowClient SDK through a standardized interface.

use galvyn::core::InitError;
use galvyn::core::Module;
use galvyn::core::PreInitError;
use mailcow::MailcowClient;
use tracing::info;
use tracing::instrument;

use crate::config::MAILCOW_API_KEY;
use crate::config::MAILCOW_BASE_URL;

/// galvyn module that serves as the main entry point for interacting with the Mailcow API.
pub struct Mailcow {
    /// SDK client
    pub sdk: MailcowClient,
}

impl Module for Mailcow {
    type Setup = ();
    type PreInit = ();

    async fn pre_init(_setup: Self::Setup) -> Result<Self::PreInit, PreInitError> {
        Ok(())
    }

    type Dependencies = ();

    #[instrument(name = "Mailcow::initialize", skip_all)]
    async fn init(
        _pre_init: Self::PreInit,
        _dependencies: &mut Self::Dependencies,
    ) -> Result<Self, InitError> {
        let sdk = MailcowClient::new(MAILCOW_BASE_URL.clone(), MAILCOW_API_KEY.clone())?;

        let version = sdk.get_version().await?;
        info!("Mailcow is running version: {}", version.version);

        Ok(Self { sdk })
    }
}
