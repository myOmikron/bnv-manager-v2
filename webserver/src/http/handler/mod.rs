//! Handler for the webserver

use galvyn::core::GalvynRouter;

pub(crate) mod openapi;

/// Handler for the admin
pub fn router_admin() -> GalvynRouter {
    GalvynRouter::new()
}

/// Handler for the club admin
pub fn router_club_admin() -> GalvynRouter {
    GalvynRouter::new()
}

/// Common handler
pub fn router_common() -> GalvynRouter {
    GalvynRouter::new()
}

/// Unauthenticated handler
pub fn router_unauthenticated() -> GalvynRouter {
    let mut router = GalvynRouter::new();

    #[cfg(debug_assertions)]
    {
        router = router.handler(openapi::handler_common::openapi);
    }

    router
}

/// Initialize the router
pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .merge(router_unauthenticated())
        .nest("/common", router_common())
        .nest("/admin", router_admin())
        .nest("/club-admin", router_club_admin())
}
