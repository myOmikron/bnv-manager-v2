//! Setup and start of the HTTP server

use std::io;
use std::net::AddrParseError;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;

use axum::{middleware, Router};
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::post;
use rorm::{Database, DatabaseConfiguration};
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing::info;
use tracing::instrument;

use conf_updater_common::ApiFailure;

use crate::config::Config;
use crate::handler;

/// Application state used across all web request handlers
#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) config: Config,
    pub(crate) db: Database,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.config.fmt(f)
    }
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
        .route("/refresh", post(handler::refresh))
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state.clone(), enforce_auth))
        .layer(TraceLayer::new_for_http());
    let socket_addr = SocketAddr::new(
        IpAddr::from_str(&config.server.listen_address)?,
        config.server.listen_port,
    );

    info!("Start to listen on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn enforce_auth(
    headers: HeaderMap,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, ApiFailure> {
    let Some(auth) = headers.get("Authorization") else {
        return Err(ApiFailure::MissingAuthorization);
    };
    // The authorization value must be a valid string
    let Ok(v) = auth.to_str() else {
        return Err(ApiFailure::InvalidAuthorization);
    };
    // The authorization value must be type 'bearer' followed by exactly one space and the token
    let mut words = v.split(" ");
    if let Some(token_type) = words.next() {
        if token_type.to_ascii_lowercase() != "bearer" {
            return Err(ApiFailure::InvalidAuthorization);
        }
    } else {
        return Err(ApiFailure::InvalidAuthorization);
    }
    // Everything after the token is ignored; therefore the token must not contain spaces
    if let Some(token) = words.next() {
        if token != state.config.server.api_token {
            return Err(ApiFailure::WrongAuthorization);
        }
    } else {
        return Err(ApiFailure::InvalidAuthorization);
    }

    let response = next.run(request).await;
    Ok(response)
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
