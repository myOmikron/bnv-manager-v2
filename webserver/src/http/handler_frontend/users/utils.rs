//! Utilities for working with [`users::schema`](super::schema)

use crate::http::common::errors::ApiResult;
use crate::http::handler_frontend::users::schema::FullUser;
use crate::models::User;
use crate::utils::schemars::SchemaDateTime;

/// Converts the populated `User` model into a `FullUser` schema.
///
/// Errors:
/// - if `user.role` is `UserRole::Internal` but `user.internal_groups` is not populated
/// - if `user.role` is `UserRole::Customer` but `user.customers` is not populated
#[track_caller]
pub fn new_full_user(user: User) -> ApiResult<FullUser> {
    Ok(FullUser {
        uuid: user.uuid,
        username: user.username,
        display_name: user.display_name,
        role: user.role,
        preferred_lang: user.preferred_lang,
        last_login: user.last_login.map(|x| SchemaDateTime(x)),
        created_at: SchemaDateTime(user.created_at),
    })
}
