//! Server initialization

use std::io;
use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::config::LISTEN_ADDRESS;
use crate::config::LISTEN_PORT;

/// Start the http server
pub async fn run() -> Result<(), io::Error> {
    let router = Router::new();

    let addr = SocketAddr::new(*LISTEN_ADDRESS.get(), *LISTEN_PORT.get());
    info!("Listen on http://{addr}");
    axum::serve(TcpListener::bind(addr).await?, router).await
}
