//! Parts of the http api for the frontend
//!
//! This included the router as well as the handlers and schemas

use axum::Router;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPageBuilder;
use tower::ServiceBuilder;

use crate::http::middlewares::auth_required::auth_required;

pub mod auth;
pub mod users;
pub mod websites;
pub mod ws;

/// The swagger page for the frontend
pub static FRONTEND_API_V1: SwaggapiPageBuilder = SwaggapiPageBuilder::new()
    .title("Frontend")
    .filename("frontend.json");

/// Create the router for the Frontend API
pub fn initialize() -> ApiContext<Router> {
    ApiContext::new().nest(
        "/v1",
        ApiContext::new()
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
                            .handler(users::handler::get_me)
                            .handler(users::handler::update_me)
                            .handler(users::handler::change_password),
                    )
                    .nest(
                        "/websites",
                        ApiContext::new()
                            .tag("Websites")
                            .handler(websites::handler::create_website)
                            .handler(websites::handler::get_website)
                            .handler(websites::handler::get_all_websites)
                            .handler(websites::handler::update_website)
                            .handler(websites::handler::add_domain_to_website)
                            .handler(websites::handler::remove_domain_from_website)
                            .handler(websites::handler::delete_website)
                            .handler(websites::handler::deploy_website)
                            .handler(websites::handler::check_dns),
                    )
                    .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_required))),
            ),
    )
}
