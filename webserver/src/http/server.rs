//! Server initialization

use std::net::SocketAddr;

use galvyn::core::GalvynRouter;
use galvyn::error::GalvynError;
use galvyn::RouterBuilder;

use crate::config::LISTEN_ADDRESS;
use crate::config::LISTEN_PORT;
use crate::http::handler::invites::handler::accept_invite;
use crate::http::handler::invites::handler::get_invite;
use crate::http::handler::openapi::handler::openapi;

/// Start the http server
pub async fn run(mut router: RouterBuilder) -> Result<(), GalvynError> {
    let addr = SocketAddr::new(*LISTEN_ADDRESS.get(), *LISTEN_PORT.get());

    router
        .add_routes(
            GalvynRouter::new().nest(
                "/api/v1",
                GalvynRouter::new().handler(openapi).nest(
                    "/invites",
                    GalvynRouter::new()
                        .handler(accept_invite)
                        .handler(get_invite),
                ),
            ),
        )
        .start(addr)
        .await
}
