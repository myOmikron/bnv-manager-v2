//! Setup and start of the HTTP server

use std::io;
use std::net::AddrParseError;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;

use axum::Router;
use axum::routing::post;
use rorm::{Database, DatabaseConfiguration};
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::error;
use tracing::info;
use tracing::instrument;

use crate::config::Config;
use crate::handler;

/// Application state used across all web request handlers
#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) config: Config,
    pub(crate) db: Database,
}

/// Start the HTTP server
#[instrument(skip_all, ret)]
pub(crate) async fn start(config: Config) -> Result<(), StartServerError> {
    let mut conf = DatabaseConfiguration::new(config.database.clone().into());
    conf.disable_logging = Some(true);
    let db = Database::connect(conf).await?;

    let state = AppState {
        config: config.clone(),
        db,
    };
    let router = Router::new()
        .route("/setup", post(handler::setup))
        .route("/teardown", post(handler::teardown))
        .with_state(state);
    let socket_addr = SocketAddr::new(
        IpAddr::from_str(&config.server.listen_address)?,
        config.server.listen_port,
    );

    info!("Start to listen on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

/// Errors that can occur while starting the HTTP server
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub(crate) enum StartServerError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rorm::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] AddrParseError),
}
