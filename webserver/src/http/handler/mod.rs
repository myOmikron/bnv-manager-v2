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
            "/clubs",
            GalvynRouter::new()
                .handler(clubs::get_club__admin)
                .handler(clubs::get_clubs__admin)
                .handler(clubs::create_club__admin)
                .handler(clubs::delete_club__admin)
                .handler(clubs::get_club_admins__admin)
                .handler(clubs::get_club_members__admin),
        )
        .nest(
            "/accounts",
            GalvynRouter::new().handler(accounts::get_all_superadmins__admin),
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
            .handler(openapi::openapi_admin)
            .handler(openapi::openapi_club_admin)
            .handler(openapi::openapi_club_member)
            .handler(openapi::openapi_common);
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

/// Openapi page for the common API
pub struct CommonApi;

/// Common handler
pub fn router_common() -> GalvynRouter {
    GalvynRouter::with_openapi_page(CommonApi)
        .handler(me::get_me)
        .nest("/auth", GalvynRouter::new().handler(auth::sign_out))
}

/// Initialize the router
pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .merge(router_unauthenticated())
        .nest("/common", router_common())
        .nest("/admin", router_admin())
        .nest("/club-admin", router_club_admin())
}
