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
use crate::models::domain::Domain;
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

#[get("/mailbox-stats")]
#[instrument(name = "Api::club_admin::get_mailbox_stats")]
pub async fn get_mailbox_stats(
    Path(club_uuid): Path<ClubUuid>,
) -> ApiResult<ApiJson<Vec<schema::MailboxStatsSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let club = Club::find_by_uuid(&mut tx, club_uuid)
        .await?
        .ok_or(ApiError::bad_request("Club not found"))?;

    let member_emails: std::collections::HashSet<String> = club
        .members_page(&mut tx, i64::MAX as u64, 0, None)
        .await?
        .items
        .into_iter()
        .map(|m| m.email.into_inner())
        .collect();

    tx.commit().await?;

    let domain: String = club.primary_domain.into_inner();

    let sdk = &Mailcow::global().sdk;

    let domain_quota = sdk
        .get_all_domains()
        .await
        .map_err(ApiError::map_server_error(
            "Could not retrieve domains from mailcow",
        ))?
        .into_iter()
        .find(|d| d.domain_name == domain)
        .map(|d| d.def_quota_for_mbox)
        .unwrap_or(0);

    let mut mailboxes: Vec<_> = sdk
        .get_all_mailboxes(&domain)
        .await
        .map_err(ApiError::map_server_error(
            "Could not retrieve mailboxes from mailcow",
        ))?
        .into_iter()
        .filter(|m| member_emails.contains(&m.username))
        .collect();

    mailboxes.sort_by(|a, b| b.quota_used.cmp(&a.quota_used));

    let stats = mailboxes
        .into_iter()
        .take(10)
        .map(|m| schema::MailboxStatsSchema {
            email: m.username,
            quota_used: m.quota_used,
            quota: if m.quota == 0 { domain_quota } else { m.quota },
            messages: m.messages,
        })
        .collect();

    Ok(ApiJson(stats))
}

#[get("/domain-stats")]
#[instrument(name = "Api::club_admin::get_domain_stats")]
pub async fn get_domain_stats(
    Path(club_uuid): Path<ClubUuid>,
) -> ApiResult<ApiJson<Vec<schema::DomainStatsSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let club_domains = Domain::find_all_by_club(&mut tx, club_uuid).await?;

    tx.commit().await?;

    let domain_names: std::collections::HashSet<String> = club_domains
        .into_iter()
        .map(|d| d.domain.into_inner())
        .collect();

    let all_mailcow_domains = Mailcow::global()
        .sdk
        .get_all_domains()
        .await
        .map_err(ApiError::map_server_error(
            "Could not retrieve domains from mailcow",
        ))?;

    let stats = all_mailcow_domains
        .into_iter()
        .filter(|d| domain_names.contains(&d.domain_name))
        .map(|d| schema::DomainStatsSchema {
            domain: d.domain_name,
            bytes_used: d.bytes_total,
            quota: d.max_quota_for_domain,
            mailboxes_used: d.mboxes_in_domain,
            mailboxes_max: d.max_num_mboxes_for_domain,
            messages: d.msgs_total,
        })
        .collect();

    Ok(ApiJson(stats))
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

    if account.club != club_uuid {
        return Err(ApiError::bad_request(
            "Cannot delete account of a different club",
        ));
    }

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
