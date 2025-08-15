//! Handler for the webserver

use galvyn::core::GalvynRouter;

pub mod auth;
pub mod invites;
pub mod me;
pub mod openapi;

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
    GalvynRouter::new().handler(me::get_me)
}

/// Unauthenticated handler
pub fn router_unauthenticated() -> GalvynRouter {
    let mut router = GalvynRouter::new();

    #[cfg(debug_assertions)]
    {
        router = router.handler(openapi::openapi);
    }

    router
        .nest(
            "/invite",
            GalvynRouter::new()
                .handler(invites::get_invite_common)
                .handler(invites::accept_invite),
        )
        .nest("/auth", GalvynRouter::new().handler(auth::sign_in))
}

/// Initialize the router
pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .merge(router_unauthenticated())
        .nest("/common", router_common())
        .nest("/admin", router_admin())
        .nest("/club-admin", router_club_admin())
}
