use rorm::insert;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::get;
use swaggapi::post;
use uuid::Uuid;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::FormResult;
use crate::http::extractors::api_json::ApiJson;
use crate::http::handler_frontend::clubs::schema::ClubList;
use crate::http::handler_frontend::clubs::schema::CreateClubErrors;
use crate::http::handler_frontend::clubs::schema::CreateClubRequest;
use crate::http::handler_frontend::clubs::schema::FullClub;
use crate::models::Club;
use crate::models::ClubUser;

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

        clubs.push(FullClub {
            uuid: club.uuid,
            name: club.name,
            users: club_users as u64,
        })
    }

    tx.commit().await?;

    Ok(ApiJson(ClubList { clubs }))
}

/// Create a new club
#[post("/create")]
pub async fn create_club(
    ApiJson(CreateClubRequest { name }): ApiJson<CreateClubRequest>,
) -> ApiResult<ApiJson<FormResult<Uuid, CreateClubErrors>>> {
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

    Ok(ApiJson(FormResult::ok(uuid)))
}
