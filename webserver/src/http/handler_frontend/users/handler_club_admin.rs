//! The handler for accessing users for club admins

use futures_util::StreamExt;
use futures_util::TryStreamExt;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::get;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::extractors::api_json::ApiJson;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
use crate::http::handler_frontend::users::schema::SimpleUser;
use crate::models::ClubUser;
use crate::models::User;
use crate::models::UserRole;

/// Retrieve the users of a club
#[get("/")]
pub async fn get_club_users_club_admin(
    SessionUser { role, .. }: SessionUser,
) -> ApiResult<ApiJson<Vec<SimpleUser>>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let club_uuid = if let UserRoleWithClub::ClubAdmin { club } = role {
        club
    } else {
        return Err(ApiError::new_internal_server_error("Received invalid role"));
    };
    let users: Vec<SimpleUser> = query!(&mut tx, (ClubUser::F.user as User,))
        .condition(ClubUser::F.club.equals(club_uuid))
        .stream()
        .map(|x| match x {
            Ok((x,)) => Ok(SimpleUser {
                uuid: x.uuid,
                username: x.username,
                display_name: x.display_name,
                role: UserRole::User,
            }),
            Err(err) => Err(err),
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(users))
}
