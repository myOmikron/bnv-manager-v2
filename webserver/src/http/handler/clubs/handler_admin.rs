use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::delete;
use galvyn::get;
use galvyn::post;
use rorm::Database;
use tracing::instrument;

use crate::http::handler::clubs::CreateClubError;
use crate::http::handler::clubs::CreateClubRequest;
use crate::http::handler::clubs::schema;
use crate::models::club::Club;
use crate::models::club::ClubUuid;

#[get("/")]
#[instrument(name = "Api::get_clubs_admin")]
pub async fn get_clubs_admin() -> ApiResult<ApiJson<Vec<schema::Club>>> {
    let mut tx = Database::global().start_transaction().await?;

    let clubs = Club::find_all(&mut tx)
        .await?
        .into_iter()
        .map(|x| schema::Club {
            uuid: x.uuid,
            name: x.name,
            description: x.description,
            modified_at: SchemaDateTime(x.modified_at),
            created_at: SchemaDateTime(x.created_at),
            admin_count: x.admin_count,
            member_count: x.member_count,
        })
        .collect();

    tx.commit().await?;

    Ok(ApiJson(clubs))
}

#[post("/")]
#[instrument(name = "Api::create_club_admin")]
pub async fn create_club_admin(
    ApiJson(CreateClubRequest { name, description }): ApiJson<CreateClubRequest>,
) -> ApiResult<ApiJson<FormResult<ClubUuid, CreateClubError>>> {
    let mut tx = Database::global().start_transaction().await?;

    let existing = Club::find_by_name(&mut tx, &name).await?;

    if existing.is_some() {
        return Ok(ApiJson(FormResult::err(CreateClubError {
            name_already_exists: true,
        })));
    }

    let uuid = Club::create(&mut tx, name, description).await?.uuid;

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(uuid)))
}

#[delete("/{uuid}")]
#[instrument(name = "Api::delete_club_admin")]
pub async fn delete_club_admin(Path(SingleUuid { uuid }): Path<SingleUuid>) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, ClubUuid(uuid)).await?;
    if let Some(club) = club {
        club.delete(&mut tx).await?;
    }

    tx.commit().await?;

    Ok(())
}
