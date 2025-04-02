//! Handler for the webserver

use axum::middleware;
use galvyn::core::GalvynRouter;
use tower::ServiceBuilder;

use crate::http::middlewares::auth_required::admin_required;
use crate::http::middlewares::auth_required::auth_required;
use crate::http::middlewares::auth_required::club_admin_required;

pub(crate) mod auth;
pub(crate) mod clubs;
pub(crate) mod invites;
pub(crate) mod me;
pub(crate) mod openapi;

/// Handler for the admin
pub fn router_admin() -> GalvynRouter {
    GalvynRouter::new()
        .handler(clubs::handler::admin_get_clubs)
        .handler(clubs::handler::admin_create_club)
        .handler(clubs::handler::admin_delete_club)
        .layer(middleware::from_fn(admin_required))
}

/// Handler for the club admin
pub fn router_club_admin() -> GalvynRouter {
    GalvynRouter::new().layer(middleware::from_fn(club_admin_required))
}

/// Common handler
pub fn router_common() -> GalvynRouter {
    GalvynRouter::new()
        .nest("/users", GalvynRouter::new().handler(me::handler::get_me))
        .layer(middleware::from_fn(auth_required))
}

/// Unauthenticated handler
pub fn router_unauthenticated() -> GalvynRouter {
    let mut router = GalvynRouter::new();

    #[cfg(debug_assertions)]
    {
        router = router.handler(openapi::handler::openapi);
    }

    router
        .nest(
            "/invites",
            GalvynRouter::new()
                .handler(invites::handler::accept_invite)
                .handler(invites::handler::get_invite),
        )
        .nest(
            "/auth",
            GalvynRouter::new()
                .handler(auth::handler::login)
                .route_layer(ServiceBuilder::new().concurrency_limit(10))
                .handler(auth::handler::logout),
        )
}

/// Initialize the router
pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .merge(router_unauthenticated())
        .nest("/common", router_common())
        .nest("/admin", router_admin())
        .nest("/club-admin", router_club_admin())
}
