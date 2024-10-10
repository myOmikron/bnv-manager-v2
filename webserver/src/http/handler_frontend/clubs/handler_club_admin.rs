//! Handler of clubs for club admins

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
use crate::http::handler_frontend::clubs::schema::FullClub;
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
use crate::http::handler_frontend::users::schema::SimpleUser;
use crate::models::Club;
use crate::models::ClubAdmin;
use crate::models::ClubUser;
use crate::models::User;
use crate::models::UserRole;

/// Get the club of the current logged-in clubadmin
#[get("/clubs/my")]
pub async fn get_club_club_admin(
    SessionUser { role, .. }: SessionUser,
) -> ApiResult<ApiJson<FullClub>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let UserRoleWithClub::ClubAdmin { club } = role else {
        return Err(ApiError::new_internal_server_error("invalid role"));
    };

    let club = query!(&mut tx, Club)
        .condition(Club::F.uuid.equals(club))
        .optional()
        .await?
        .ok_or(ApiError::new_internal_server_error("Club not found"))?;

    let admins = query!(&mut tx, (ClubAdmin::F.user as User))
        .condition(ClubAdmin::F.club.equals(club.uuid))
        .stream()
        .map_ok(|(user,)| SimpleUser {
            uuid: user.uuid,
            username: user.username,
            display_name: user.display_name,
            role: UserRole::ClubAdmin,
            website_count: user.website_count as u64,
        })
        .try_collect()
        .await?;

    let user_count = query!(&mut tx, (ClubUser::F.uuid.count(),))
        .condition(ClubUser::F.club.equals(club.uuid))
        .one()
        .await?
        .0 as u64;

    tx.commit().await?;

    Ok(ApiJson(FullClub {
        uuid: club.uuid,
        name: club.name,
        domain: club.domain,
        user_count,
        admins,
        website_count: club.website_count as u64,
    }))
}
