//! Administrative endpoints for managing clubs.

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::re_exports::axum::extract::Query;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::core::stuff::schema::Page;
use galvyn::core::stuff::schema::SingleUuid;
use galvyn::delete;
use galvyn::get;
use galvyn::post;
use rorm::Database;
use tracing::instrument;

use crate::http::handler_frontend::accounts::SimpleAccount;
use crate::http::handler_frontend::clubs::CreateClubError;
use crate::http::handler_frontend::clubs::CreateClubRequest;
use crate::http::handler_frontend::clubs::PageParams;
use crate::http::handler_frontend::clubs::schema;
use crate::http::handler_frontend::domains;
use crate::http::handler_frontend::invites::GetInvite;
use crate::models::club::Club;
use crate::models::club::ClubUuid;
use crate::models::domain::Domain;
use crate::models::invite::Invite;
use crate::models::role::Role;

#[get("/")]
#[instrument(name = "Api::admin::get_clubs")]
pub async fn get_clubs() -> ApiResult<ApiJson<Vec<schema::Club>>> {
    let mut tx = Database::global().start_transaction().await?;

    let clubs = Club::find_all(&mut tx)
        .await?
        .into_iter()
        .map(schema::Club::from)
        .collect();

    tx.commit().await?;

    Ok(ApiJson(clubs))
}

#[post("/")]
#[instrument(name = "Api::admin::create_club")]
pub async fn create_club(
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
#[instrument(name = "Api::admin::delete_club")]
pub async fn delete_club(Path(SingleUuid { uuid }): Path<SingleUuid>) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, ClubUuid(uuid)).await?;
    if let Some(club) = club {
        club.delete(&mut tx).await?;
    }

    tx.commit().await?;

    Ok(())
}

#[get("/{uuid}")]
#[instrument(name = "Api::admin::get_club")]
pub async fn get_club(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<schema::Club>> {
    let club = Club::find_by_uuid(Database::global(), ClubUuid(uuid))
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    Ok(ApiJson(schema::Club::from(club)))
}

#[get("/{uuid}/members")]
#[instrument(name = "Api::admin::get_club_members")]
pub async fn get_club_members(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    Query(PageParams {
        limit,
        offset,
        search,
    }): Query<PageParams>,
) -> ApiResult<ApiJson<Page<SimpleAccount>>> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, ClubUuid(uuid))
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

#[get("/{uuid}/admins")]
#[instrument(name = "Api::admin::get_club_admins")]
pub async fn get_club_admins(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    Query(PageParams {
        limit,
        offset,
        search,
    }): Query<PageParams>,
) -> ApiResult<ApiJson<Page<SimpleAccount>>> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, ClubUuid(uuid))
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    let page = club.admins_page(&mut tx, limit, offset, search).await?;

    tx.commit().await?;

    Ok(ApiJson(Page {
        items: page.items.into_iter().map(SimpleAccount::from).collect(),
        limit: page.limit,
        offset: page.offset,
        total: page.total,
    }))
}

#[get("/{uuid}/admins/invites")]
#[instrument(name = "Api::admin::get_club_admin_invites")]
pub async fn get_club_admin_invites(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<Vec<GetInvite>>> {
    let mut tx = Database::global().start_transaction().await?;

    let invites = Invite::find_by_club(&mut tx, ClubUuid(uuid))
        .await?
        .into_iter()
        .filter_map(|x| {
            x.roles
                .contains(&Role::ClubAdmin {
                    club_uuid: ClubUuid(uuid),
                })
                .then(|| GetInvite::from(x))
        })
        .collect();

    tx.commit().await?;

    Ok(ApiJson(invites))
}

#[get("/{uuid}/members/invites")]
#[instrument(name = "Api::admin::get_club_member_invites")]
pub async fn get_club_member_invites(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<Vec<GetInvite>>> {
    let mut tx = Database::global().start_transaction().await?;

    let invites = Invite::find_by_club(&mut tx, ClubUuid(uuid))
        .await?
        .into_iter()
        .filter_map(|x| {
            x.roles
                .contains(&Role::ClubMember {
                    club_uuid: ClubUuid(uuid),
                })
                .then(|| GetInvite::from(x))
        })
        .collect();

    tx.commit().await?;

    Ok(ApiJson(invites))
}

#[get("/{uuid}/domains")]
#[instrument(name = "Api::admin::get_club_domains")]
pub async fn get_club_domains(
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<Vec<domains::Domain>>> {
    let mut tx = Database::global().start_transaction().await?;

    let domains = Domain::find_all_by_club(&mut tx, ClubUuid(uuid))
        .await?
        .into_iter()
        .map(domains::Domain::from)
        .collect();

    tx.commit().await?;

    Ok(ApiJson(domains))
}
