use futures_util::TryStreamExt;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::core::Module;
use rorm::Database;
use uuid::Uuid;

use crate::http::handler::clubs::schema::CreateClubRequest;
use crate::http::handler::clubs::schema::CreateClubResponseError;
use crate::http::handler::clubs::schema::SimpleClub;
use crate::models::club::Club;
use crate::models::club::ClubInsert;

#[galvyn::get("/clubs")]
pub async fn admin_get_clubs() -> ApiResult<ApiJson<Vec<SimpleClub>>> {
    let mut tx = Database::global().start_transaction().await?;

    let clubs = rorm::query(&mut tx, Club)
        .stream()
        .map_ok(|x| SimpleClub {
            uuid: x.uuid,
            name: x.name,
            created_at: SchemaDateTime(x.created_at),
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(clubs))
}

#[galvyn::post("/clubs")]
pub async fn admin_create_club(
    ApiJson(CreateClubRequest { name }): ApiJson<CreateClubRequest>,
) -> ApiResult<ApiJson<FormResult<SingleUuid, CreateClubResponseError>>> {
    let mut tx = Database::global().start_transaction().await?;

    let exists = rorm::query(&mut tx, Club)
        .condition(Club.name.equals(&name))
        .optional()
        .await?;

    if exists.is_some() {
        return Ok(ApiJson(FormResult::err(CreateClubResponseError {
            name_already_occupied: true,
        })));
    }

    let uuid = rorm::insert(&mut tx, Club)
        .return_primary_key()
        .single(&ClubInsert {
            uuid: Uuid::new_v4(),
            name,
        })
        .await?;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(SingleUuid { uuid })))
}
