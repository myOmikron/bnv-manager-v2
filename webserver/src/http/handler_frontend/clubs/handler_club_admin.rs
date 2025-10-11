//! Club admin endpoints

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::re_exports::axum::extract::Query;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::Page;
use galvyn::delete;
use galvyn::get;
use galvyn::rorm::Database;
use tracing::instrument;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::accounts::SimpleMemberAccountSchema;
use crate::http::handler_frontend::clubs::PageParams;
use crate::http::handler_frontend::clubs::schema;
use crate::http::handler_frontend::invites::GetInvite;
use crate::models::account::AccountUuid;
use crate::models::account::ClubAccount;
use crate::models::club::Club;
use crate::models::club::ClubUuid;
use crate::models::invite::Invite;
use crate::modules::mailcow::Mailcow;

#[get("/")]
#[instrument(name = "Api::club_admin::get_club")]
pub async fn get_club(
    Path(club_uuid): Path<ClubUuid>,
    SessionUser { uuid: account_uuid }: SessionUser,
) -> ApiResult<ApiJson<schema::ClubSchema>> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, club_uuid)
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    tx.commit().await?;

    Ok(ApiJson(schema::ClubSchema::from(club)))
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
) -> ApiResult<ApiJson<Page<SimpleMemberAccountSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, club_uuid)
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    let page = club.members_page(&mut tx, limit, offset, search).await?;

    tx.commit().await?;

    Ok(ApiJson(Page {
        items: page
            .items
            .into_iter()
            .map(SimpleMemberAccountSchema::from)
            .collect(),
        limit: page.limit,
        offset: page.offset,
        total: page.total,
    }))
}

#[get("/members/invites")]
#[instrument(name = "Api::club_admin::get_club_member_invites")]
pub async fn get_club_member_invites(
    Path(club_uuid): Path<ClubUuid>,
) -> ApiResult<ApiJson<Vec<GetInvite>>> {
    let mut tx = Database::global().start_transaction().await?;

    let invites = Invite::find_by_club(&mut tx, club_uuid)
        .await?
        .into_iter()
        .filter_map(|x| x.email.is_some().then_some(GetInvite::from(x)))
        .collect();

    tx.commit().await?;

    Ok(ApiJson(invites))
}

#[delete("/{member_uuid}")]
#[instrument(name = "Api::club_admin::delete_member")]
pub async fn delete_member(
    Path((club_uuid, account_uuid)): Path<(ClubUuid, AccountUuid)>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let account = ClubAccount::get_by_uuid(&mut tx, account_uuid)
        .await?
        .ok_or(ApiError::bad_request("Account not found"))?;

    Mailcow::global()
        .sdk
        .delete_mailbox(vec![account.email.clone().into_inner()])
        .await
        .map_err(ApiError::map_server_error(
            "Could not delete mailbox in mailcow",
        ))?;

    account.delete(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}
