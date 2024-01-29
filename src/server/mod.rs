use std::io;
use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;
use futures::stream::StreamExt;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook_tokio::Signals;
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::{error, info, info_span, instrument, Instrument};

use crate::config::Config;
use crate::server::handler::test;

pub mod handler;

/// Start the webserver
#[instrument(name = "server", level = "debug", skip_all)]
pub async fn start_server(config: &Config) -> Result<(), StartServerError> {
    let router = Router::new().route("/", get(test::test));

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
