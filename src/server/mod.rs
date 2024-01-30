use std::io;
use std::net::SocketAddr;

use axum::Router;
use futures::stream::StreamExt;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook_tokio::Signals;
use thiserror::Error;
use tokio::net::TcpListener;
use tower_sessions::cookie::time::Duration;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, SessionManagerLayer};
use tracing::{error, info, info_span, instrument, Instrument};

use crate::config::Config;
use crate::server::middleware::sessions::DBStore;

pub mod handler;
pub mod middleware;

/// Start the webserver
#[instrument(name = "server", level = "debug", skip_all)]
pub async fn start_server(config: &Config) -> Result<(), StartServerError> {
    let session_store = DBStore;
    session_store.run_deletion_task(Duration::minutes(5));

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_http_only(true)
        .with_same_site(SameSite::Strict)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let router = Router::new().layer(session_layer);

    let sock = SocketAddr::new(
        config.server.listen_address,
        config.server.listen_port.get(),
    );
    info!("Starting to listen on: http://{sock}");

    axum::serve(TcpListener::bind(sock).await?, router)
        .with_graceful_shutdown(handle_signals().instrument(info_span!("signals")))
        .await?;

    Ok(())
}

async fn handle_signals() {
    let Ok(mut signals) = Signals::new(TERM_SIGNALS) else {
        error!("Could not register signals");
        return;
    };
    let handle = signals.handle();

    if let Some(signal) = signals.next().await {
        info!("Received signal {signal}, exiting ..");
    }

    handle.close();
}

/// The errors that can occur while starting the webserver
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum StartServerError {
    #[error("{0}")]
    Io(#[from] io::Error),
}
