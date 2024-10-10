//! The handler for accessing users for club admins

use axum::extract::Path;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use rorm::and;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::delete;
use swaggapi::get;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::Csv;
use crate::http::common::schemas::SingleUuid;
use crate::http::extractors::api_json::ApiJson;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
use crate::http::handler_frontend::users::schema::ExportUser;
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
                website_count: x.website_count as u64,
            }),
            Err(err) => Err(err),
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(users))
}

/// Retrieve the users of a club
#[delete("/:uuid")]
pub async fn delete_club_user(
    SessionUser { role, .. }: SessionUser,
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<()> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let club_uuid = if let UserRoleWithClub::ClubAdmin { club } = role {
        club
    } else {
        return Err(ApiError::new_internal_server_error("Received invalid role"));
    };

    let club_user = query!(&mut tx, (ClubUser::F.uuid,))
        .condition(and!(
            ClubUser::F.user.equals(uuid),
            ClubUser::F.club.equals(club_uuid)
        ))
        .optional()
        .await?;

    if club_user.is_some() {
        rorm::delete!(&mut tx, User)
            .condition(User::F.uuid.equals(uuid))
            .await?;
    }

    tx.commit().await?;

    Ok(())
}

/// Export all users of the club as json
#[get("/export/json")]
pub async fn export_json_ca(
    SessionUser { role, .. }: SessionUser,
) -> ApiResult<ApiJson<Vec<ExportUser>>> {
    let UserRoleWithClub::ClubAdmin { club } = role else {
        return Err(ApiError::new_internal_server_error("Received invalid role"));
    };

    let mut tx = GLOBAL.db.start_transaction().await?;

    let users = query!(&mut tx, (ClubUser::F.user as User,))
        .condition(ClubUser::F.club.equals(club))
        .stream()
        .map_ok(|(user,)| ExportUser {
            uuid: user.uuid,
            username: user.username,
            display_name: user.display_name,
            website_count: user.website_count as u64,
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(users))
}

/// Export all users of the club as csv
#[get("/export/csv")]
pub async fn export_csv_ca(SessionUser { role, .. }: SessionUser) -> ApiResult<Csv<Vec<u8>>> {
    let UserRoleWithClub::ClubAdmin { club } = role else {
        return Err(ApiError::new_internal_server_error("Received invalid role"));
    };

    let mut tx = GLOBAL.db.start_transaction().await?;

    let users: Vec<_> = query!(&mut tx, (ClubUser::F.user as User,))
        .condition(ClubUser::F.club.equals(club))
        .stream()
        .map_ok(|(user,)| ExportUser {
            uuid: user.uuid,
            username: user.username,
            display_name: user.display_name,
            website_count: user.website_count as u64,
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    let mut writer = csv::Writer::from_writer(vec![]);

    for user in users {
        writer.serialize(&user).map_err(ApiError::from)?;
    }

    let data = writer
        .into_inner()
        .map_err(|_| ApiError::new_internal_server_error("export error"))?;

    Ok(Csv(data))
}
