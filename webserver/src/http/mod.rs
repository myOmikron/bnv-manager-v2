//! HTTP related code

pub mod extractors;
pub mod handler;
pub mod middlewares;
pub mod server;

/// Key for accessing a logged-in account
pub const SESSION_ACCOUNT: &str = "account";
