//! HTTP related code

pub mod handler;
pub mod middlewares;
pub mod server;

/// Key for accessing a logged-in user
pub const SESSION_USER: &str = "user";
