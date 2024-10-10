//! Parts of the http api for the frontend
//!
//! This included the router as well as the handlers and schemas

use axum::Router;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPageBuilder;
use tower::ServiceBuilder;

use crate::http::middlewares::auth_required::auth_required;
use crate::http::middlewares::role_required::RoleRequiredLayer;
use crate::models::UserRole;

pub mod auth;
pub mod clubs;
pub mod user_invites;
pub mod users;
pub mod websites;
pub mod ws;

/// The swagger page for the frontend
pub static FRONTEND_API_V1: SwaggapiPageBuilder = SwaggapiPageBuilder::new()
    .title("Frontend")
    .filename("frontend.json");

/// Admin routes
pub fn admin() -> ApiContext<Router> {
    ApiContext::new()
        .nest(
            "/clubs",
            ApiContext::new()
                .tag("clubs")
                .handler(clubs::handler_admin::get_all_clubs)
                .handler(clubs::handler_admin::create_club)
                .handler(clubs::handler_admin::get_club)
                .handler(clubs::handler_admin::delete_club)
                .handler(clubs::handler_admin::update_club),
        )
        .nest(
            "/user-invites",
            ApiContext::new()
                .tag("user-invites")
                .handler(user_invites::handler_admin::create_invite_admin),
        )
}

/// Club admin routes
pub fn club_admin() -> ApiContext<Router> {
    ApiContext::new()
        .nest(
            "/clubs",
            ApiContext::new()
                .tag("clubs")
                .handler(clubs::handler_club_admin::get_club_club_admin),
        )
        .nest(
            "/user-invites",
            ApiContext::new()
                .tag("user-invites")
                .handler(user_invites::handler_club_admin::create_invite_club_admin),
        )
        .nest(
            "/users",
            ApiContext::new()
                .tag("users")
                .handler(users::handler_club_admin::get_club_users_club_admin)
                .handler(users::handler_club_admin::delete_club_user),
        )
}

/// Normal user routes
pub fn user() -> ApiContext<Router> {
    ApiContext::new().nest(
        "/websites",
        ApiContext::new()
            .tag("websites")
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
}

/// Common routes
pub fn common() -> ApiContext<Router> {
    ApiContext::new()
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
                        .handler(users::handler_common::get_me)
                        .handler(users::handler_common::update_me)
                        .handler(users::handler_common::change_password),
                )
                .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_required))),
        )
        .nest(
            "/invites",
            ApiContext::new()
                .tag("user-invites")
                .handler(user_invites::handler_common::get_user_invite)
                .handler(user_invites::handler_common::accept_invite_pw),
        )
}

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
            .nest("/common", common())
            .nest(
                "/admin",
                admin().layer(
                    ServiceBuilder::new().layer(RoleRequiredLayer::new(UserRole::Administrator)),
                ),
            )
            .nest(
                "/club-admin",
                club_admin().layer(
                    ServiceBuilder::new().layer(RoleRequiredLayer::new(UserRole::ClubAdmin)),
                ),
            )
            .nest(
                "/user",
                user().layer(ServiceBuilder::new().layer(RoleRequiredLayer::new(UserRole::User))),
            ),
    )
}
