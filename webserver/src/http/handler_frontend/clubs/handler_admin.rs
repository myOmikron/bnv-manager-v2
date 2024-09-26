use axum::extract::Path;
use futures_util::TryStreamExt;
use rorm::insert;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::get;
use swaggapi::post;
use uuid::Uuid;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::FormResult;
use crate::http::common::schemas::SingleUuid;
use crate::http::extractors::api_json::ApiJson;
use crate::http::handler_frontend::clubs::schema::ClubList;
use crate::http::handler_frontend::clubs::schema::CreateClubErrors;
use crate::http::handler_frontend::clubs::schema::CreateClubRequest;
use crate::http::handler_frontend::clubs::schema::FullClub;
use crate::http::handler_frontend::clubs::schema::SimpleClub;
use crate::http::handler_frontend::users::schema::SimpleUser;
use crate::models::Club;
use crate::models::ClubUser;
use crate::models::User;

/// Get all clubs
#[get("/")]
pub async fn get_all_clubs() -> ApiResult<ApiJson<ClubList>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let c = query!(&mut tx, Club).all().await?;
    let mut clubs = vec![];

    for club in c {
        let club_users = query!(&mut tx, (ClubUser::F.uuid.count(),))
            .condition(ClubUser::F.club.equals(club.uuid))
            .one()
            .await?
            .0;

        clubs.push(SimpleClub {
            uuid: club.uuid,
            name: club.name,
            user_count: club_users as u64,
        })
    }

    tx.commit().await?;

    Ok(ApiJson(ClubList { clubs }))
}

/// Get a single club
#[get("/:uuid")]
pub async fn get_club(Path(SingleUuid { uuid }): Path<SingleUuid>) -> ApiResult<ApiJson<FullClub>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let club = query!(&mut tx, Club)
        .condition(Club::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    let club_users = query!(&mut tx, (ClubUser::F.uuid.count(),))
        .condition(ClubUser::F.club.equals(club.uuid))
        .one()
        .await?
        .0;

    let users = query!(&mut tx, (ClubUser::F.user as User,))
        .condition(ClubUser::F.club.equals(club.uuid))
        .stream()
        .map_ok(|x| {
            let x = x.0;
            SimpleUser {
                uuid,
                role: x.role,
                username: x.username,
                display_name: x.display_name,
            }
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(FullClub {
        uuid,
        name: club.name,
        user_count: club_users as u64,
        admins: users,
    }))
}

/// Create a new club
#[post("/create")]
pub async fn create_club(
    ApiJson(CreateClubRequest { name }): ApiJson<CreateClubRequest>,
) -> ApiResult<ApiJson<FormResult<SingleUuid, CreateClubErrors>>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let club = query!(&mut tx, Club)
        .condition(Club::F.name.equals(&name))
        .optional()
        .await?;

    if club.is_some() {
        return Ok(ApiJson(FormResult::err(CreateClubErrors {
            name_in_use: false,
        })));
    }

    let uuid = insert!(&mut tx, Club)
        .return_primary_key()
        .single(&Club {
            uuid: Uuid::new_v4(),
            name,
        })
        .await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(SingleUuid { uuid })))
}
