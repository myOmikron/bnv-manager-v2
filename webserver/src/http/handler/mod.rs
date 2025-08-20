//! Handler for the webserver

use galvyn::core::GalvynRouter;
use galvyn::openapi::OpenapiRouterExt;

pub mod accounts;
pub mod auth;
pub mod clubs;
pub mod invites;
pub mod me;
pub mod openapi;

/// Openapi page for the admin API
pub struct AdminAPI;

/// Handler for the admin
pub fn router_admin() -> GalvynRouter {
    GalvynRouter::with_openapi_page(AdminAPI)
        .nest(
            "/accounts",
            GalvynRouter::new().handler(accounts::handler_admin::get_all_superadmins),
        )
        .nest(
            "/clubs",
            GalvynRouter::new()
                .handler(clubs::handler_admin::get_club)
                .handler(clubs::handler_admin::get_clubs)
                .handler(clubs::handler_admin::create_club)
                .handler(clubs::handler_admin::delete_club)
                .handler(clubs::handler_admin::get_club_admins)
                .handler(clubs::handler_admin::get_club_members),
        )
        .nest(
            "/invites",
            GalvynRouter::new().handler(invites::handler_admin::create_invite),
        )
}

/// Openapi page for the club admin API
pub struct ClubAdminApi;

/// Handler for the club admin
pub fn router_club_admin() -> GalvynRouter {
    GalvynRouter::with_openapi_page(ClubAdminApi)
}

/// Openapi page for the club member API
pub struct ClubMemberApi;

/// Handler for the club members
pub fn router_club_member() -> GalvynRouter {
    GalvynRouter::with_openapi_page(ClubMemberApi)
}

/// Unauthenticated handler
pub fn router_unauthenticated() -> GalvynRouter {
    let mut router = GalvynRouter::with_openapi_page(CommonApi);

    #[cfg(debug_assertions)]
    {
        router = router
            .handler(openapi::handler_common::openapi_admin)
            .handler(openapi::handler_common::openapi_club_admin)
            .handler(openapi::handler_common::openapi_club_member)
            .handler(openapi::handler_common::openapi_common);
    }

    router
        .nest(
            "/invite",
            GalvynRouter::new()
                .handler(invites::handler_common::get_invite_common)
                .handler(invites::handler_common::accept_invite),
        )
        .nest(
            "/auth",
            GalvynRouter::new().handler(auth::handler_common::sign_in),
        )
}

/// Openapi page for the common API
pub struct CommonApi;

/// Common handler
pub fn router_common() -> GalvynRouter {
    GalvynRouter::with_openapi_page(CommonApi)
        .nest(
            "/me",
            GalvynRouter::new().handler(me::handler_common::get_me),
        )
        .nest(
            "/auth",
            GalvynRouter::new().handler(auth::handler_common::sign_out),
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
