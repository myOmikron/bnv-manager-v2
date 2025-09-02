//! Middlewares of this server are defined in this module

pub use auth_club_admin::*;
pub use auth_superadmin::*;

mod auth_club_admin;
mod auth_superadmin;
