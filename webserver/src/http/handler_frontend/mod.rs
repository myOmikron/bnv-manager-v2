//! Parts of the http api for the frontend
//!
//! This included the router as well as the handlers and schemas

use axum::Extension;
use axum::Router;
use openidconnect::core::CoreClient;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPageBuilder;
use tower::ServiceBuilder;

use crate::http::middlewares::auth_required::auth_required;

pub mod auth;
pub mod oidc;
pub mod users;
pub mod ws;

/// The swagger page for the frontend
pub static FRONTEND_API_V1: SwaggapiPageBuilder = SwaggapiPageBuilder::new()
    .title("Frontend")
    .filename("frontend.json");

/// Create the router for the Frontend API
pub fn initialize(oidc_client: Option<CoreClient>) -> ApiContext<Router> {
    let mut oidc_context = ApiContext::new()
        .tag("oidc")
        .handler(oidc::handler::start_auth)
        .handler(oidc::handler::finish_auth);

    if let Some(oidc_client) = oidc_client {
        oidc_context =
            oidc_context.route_layer(ServiceBuilder::new().layer(Extension(oidc_client)));
    };

    ApiContext::new().nest(
        "/v1",
        ApiContext::new()
            .nest("/oidc", oidc_context)
            .nest(
                "/auth",
                ApiContext::new()
                    .tag("auth")
                    .handler(auth::handler::login)
                    .route_layer(ServiceBuilder::new().concurrency_limit(10))
                    .handler(auth::handler::logout),
            )
            .merge(
                ApiContext::new()
                    .merge(
                        ApiContext::new()
                            .tag("websocket")
                            .handler(ws::handler::websocket),
                    )
                    .nest(
                        "/users",
                        ApiContext::new()
                            .tag("users")
                            .handler(users::handler::get_me),
                    )
                    .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_required))),
            ),
    )
}
