//! Setup and start of the HTTP server

use std::{fs, io};
use std::net::AddrParseError;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::path::Path;
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
use crate::utils::{certbot, nginx};

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
    check_env(&config)?;

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

/// Perform environment and configuration checks
fn check_env(config: &Config) -> Result<(), StartServerError> {
    if !certbot::check_available() {
        return Err(StartServerError::ProgramUnavailable("certbot".to_string()));
    }
    certbot::check_account().map_err(|err| StartServerError::ProgramUnavailable(err))?;
    if !nginx::check_available() {
        return Err(StartServerError::ProgramUnavailable("nginx".to_string()));
    }
    let htdocs = Path::new(&config.misc.htdocs_root_dir);
    if !htdocs.is_absolute() || !htdocs.exists() {
        return Err(StartServerError::ConfigError(
            "htdocs_root_dir must be an absolute, existing path".to_string(),
        ));
    }
    let nginx_conf = Path::new(&config.misc.nginx_config_dir);
    if !nginx_conf.is_absolute() || !nginx_conf.exists() {
        return Err(StartServerError::ConfigError(
            "nginx_config_dir must be an absolute, existing path".to_string(),
        ));
    }
    let certbot_dir = Path::new(&config.certbot.cert_dir);
    if !certbot_dir.is_absolute() || !certbot_dir.exists() {
        return Err(StartServerError::ConfigError(
            "cert_dir must be an absolute, existing path".to_string(),
        ));
    }
    for (dir, read_result) in [
        (htdocs, fs::read_dir(htdocs)),
        (nginx_conf, fs::read_dir(nginx_conf)),
        (certbot_dir, fs::read_dir(certbot_dir)),
    ] {
        if read_result.is_err() {
            return Err(StartServerError::ConfigError(format!(
                "could not read from {}",
                dir.display()
            )));
        }
    }
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
    #[error("Program unavailable or missing permissions: {0}")]
    ProgramUnavailable(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
