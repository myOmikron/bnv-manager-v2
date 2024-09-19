//! Configuration and start of the webserver is defined in this module

use std::io;
use std::net::AddrParseError;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::Request;
use axum::Router;
use axum::ServiceExt;
use futures::StreamExt;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook_tokio::Signals;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPage;
use swaggapi::SwaggerUi;
use thiserror::Error;
use tokio::net::TcpListener;
use tower::Layer;
use tower::ServiceBuilder;
use tower_http::normalize_path::NormalizePath;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::trace::TraceLayer;
use tower_sessions_rorm_store::tower_sessions::cookie::SameSite;
use tower_sessions_rorm_store::tower_sessions::Expiry;
use tower_sessions_rorm_store::tower_sessions::SessionManagerLayer;
use tower_sessions_rorm_store::RormStore;
use tracing::error;
use tracing::info;
use tracing::info_span;
use tracing::instrument;
use tracing::Instrument;

use crate::config::Config;
use crate::global::GLOBAL;
use crate::http::handler_frontend;
use crate::http::handler_frontend::ws::schema::WsClientMsg;
use crate::http::handler_frontend::ws::schema::WsServerMsg;
use crate::http::handler_frontend::FRONTEND_API_V1;
use crate::models;

/// Start the http server
#[instrument(skip_all, ret)]
pub async fn run(config: Arc<Config>) -> Result<(), StartServerError> {
    // Register models that are not used in handlers
    (&FRONTEND_API_V1)
        .add_schema::<WsServerMsg>()
        .add_schema::<WsClientMsg>();

    let mut swaggui = SwaggerUi::without_everything().page("Frontend", &FRONTEND_API_V1);
    swaggui.path = "/docs";

    let router = Router::new()
        .merge(
            ApiContext::new()
                .page(&FRONTEND_API_V1)
                .nest("/api/frontend", handler_frontend::initialize()),
        )
        .merge(swaggui)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    SessionManagerLayer::new(RormStore::<models::Session>::new(GLOBAL.db.clone()))
                        .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)))
                        .with_same_site(SameSite::Lax),
                ),
        );

    let middleware = NormalizePathLayer::trim_trailing_slash();
    let app: NormalizePath<Router> = middleware.layer(router);

    let socket_addr = SocketAddr::new(
        IpAddr::from_str(&config.http.listen_address)?,
        config.http.listen_port,
    );

    info!("Start to listen on http://{socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
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

/// Errors that can occur while starting the server
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum StartServerError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] AddrParseError),
}
