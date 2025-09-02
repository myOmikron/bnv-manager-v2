//! Club admin endpoints

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::re_exports::axum::extract::Query;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::Page;
use galvyn::get;
use rorm::Database;
use tracing::instrument;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::accounts::SimpleAccount;
use crate::http::handler_frontend::clubs::PageParams;
use crate::http::handler_frontend::clubs::schema;
use crate::models::club::Club;
use crate::models::club::ClubUuid;

#[get("/")]
#[instrument(name = "Api::club_admin::get_club")]
pub async fn get_club(
    Path(club_uuid): Path<ClubUuid>,
    SessionUser { uuid: account_uuid }: SessionUser,
) -> ApiResult<ApiJson<schema::Club>> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, club_uuid)
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    tx.commit().await?;

    Ok(ApiJson(schema::Club::from(club)))
}

#[get("/members")]
#[instrument(name = "Api::club_admin::get_club_members")]
pub async fn get_club_members(
    Path(club_uuid): Path<ClubUuid>,
    Query(PageParams {
        limit,
        offset,
        search,
    }): Query<PageParams>,
) -> ApiResult<ApiJson<Page<SimpleAccount>>> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, club_uuid)
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    let page = club.members_page(&mut tx, limit, offset, search).await?;

    tx.commit().await?;

    Ok(ApiJson(Page {
        items: page.items.into_iter().map(SimpleAccount::from).collect(),
        limit: page.limit,
        offset: page.offset,
        total: page.total,
    }))
}
