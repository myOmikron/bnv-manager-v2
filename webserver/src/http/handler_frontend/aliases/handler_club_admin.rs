//! Endpoints for club admins to manage aliases.

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::delete;
use galvyn::get;
use galvyn::post;
use galvyn::rorm::Database;
use mailcow::aliases::schema::CreateAliasRequest;
use tracing::instrument;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::aliases::MailAliasSchema;
use crate::models::account::ClubAccount;
use crate::models::club::ClubUuid;
use crate::models::domain::Domain;
use crate::models::mail_alias::MailAlias;
use crate::models::mail_alias::MailAliasStatus;
use crate::models::mail_alias::MailAliasUuid;
use crate::modules::mailcow::Mailcow;

#[get("/")]
#[instrument(name = "Api::club_admin::get_club_aliases")]
pub async fn get_club_aliases(
    Path(club_uuid): Path<ClubUuid>,
) -> ApiResult<ApiJson<Vec<MailAliasSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let aliases = MailAlias::find_all_by_club(&mut tx, club_uuid).await?;
    let domains = Domain::find_all_by_club(&mut tx, club_uuid).await?;

    let schemas = build_alias_schemas(aliases, &domains);

    tx.commit().await?;

    Ok(ApiJson(schemas))
}

#[get("/pending")]
#[instrument(name = "Api::club_admin::get_pending_aliases")]
pub async fn get_pending_aliases(
    Path(club_uuid): Path<ClubUuid>,
) -> ApiResult<ApiJson<Vec<MailAliasSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let aliases = MailAlias::find_pending_by_club(&mut tx, club_uuid).await?;
    let domains = Domain::find_all_by_club(&mut tx, club_uuid).await?;

    let schemas = build_alias_schemas(aliases, &domains);

    tx.commit().await?;

    Ok(ApiJson(schemas))
}

#[post("/{alias_uuid}/approve")]
#[instrument(name = "Api::club_admin::approve_alias")]
pub async fn approve_alias(
    Path((club_uuid, alias_uuid)): Path<(ClubUuid, MailAliasUuid)>,
    SessionUser { uuid: _ }: SessionUser,
) -> ApiResult<ApiJson<MailAliasSchema>> {
    let mut tx = Database::global().start_transaction().await?;

    let alias = MailAlias::find_by_uuid(&mut tx, alias_uuid)
        .await?
        .ok_or(ApiError::bad_request("Alias not found"))?;

    if alias.status != MailAliasStatus::Pending {
        return Err(ApiError::bad_request("Alias is not pending"));
    }

    // Verify domain belongs to this club
    let domain = Domain::find_by_uuid(&mut tx, alias.domain)
        .await?
        .ok_or(ApiError::bad_request("Domain not found"))?;

    if domain.associated_club != Some(club_uuid) {
        return Err(ApiError::bad_request(
            "Alias domain does not belong to this club",
        ));
    }

    // Get the member's email as the goto target
    let account = ClubAccount::get_by_uuid(&mut tx, alias.account)
        .await?
        .ok_or(ApiError::bad_request("Account not found"))?;

    let alias_address = format!("{}@{}", &*alias.local_part, &*domain.domain);

    // Create alias in Mailcow
    Mailcow::global()
        .sdk
        .create_alias(CreateAliasRequest {
            address: alias_address,
            goto: account.email.to_string(),
            active: "1".to_string(),
        })
        .await
        .map_err(ApiError::map_server_error(
            "Could not create alias in mailcow",
        ))?;

    // Find the mailcow_id by fetching all aliases and matching
    let all_aliases =
        Mailcow::global()
            .sdk
            .get_all_aliases()
            .await
            .map_err(ApiError::map_server_error(
                "Could not fetch aliases from mailcow",
            ))?;

    let mailcow_alias = all_aliases
        .into_iter()
        .find(|a| {
            a.address == format!("{}@{}", &*alias.local_part, &*domain.domain)
                && a.goto == *account.email
        })
        .ok_or(ApiError::server_error(
            "Alias was created but could not be found in mailcow",
        ))?;

    alias.approve(&mut tx, mailcow_alias.id as i64).await?;

    tx.commit().await?;

    // Re-fetch the updated alias
    let mut tx2 = Database::global().start_transaction().await?;
    let updated_alias = MailAlias::find_by_uuid(&mut tx2, alias_uuid)
        .await?
        .ok_or(ApiError::server_error("Alias not found after approval"))?;
    tx2.commit().await?;

    Ok(ApiJson(MailAliasSchema::new(
        updated_alias,
        domain.domain.to_string(),
        account.display_name.to_string(),
    )))
}

#[post("/{alias_uuid}/reject")]
#[instrument(name = "Api::club_admin::reject_alias")]
pub async fn reject_alias(
    Path((club_uuid, alias_uuid)): Path<(ClubUuid, MailAliasUuid)>,
    SessionUser { uuid: _ }: SessionUser,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let alias = MailAlias::find_by_uuid(&mut tx, alias_uuid)
        .await?
        .ok_or(ApiError::bad_request("Alias not found"))?;

    if alias.status != MailAliasStatus::Pending {
        return Err(ApiError::bad_request("Alias is not pending"));
    }

    // Verify domain belongs to this club
    let domain = Domain::find_by_uuid(&mut tx, alias.domain)
        .await?
        .ok_or(ApiError::bad_request("Domain not found"))?;

    if domain.associated_club != Some(club_uuid) {
        return Err(ApiError::bad_request(
            "Alias domain does not belong to this club",
        ));
    }

    alias.reject(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}

#[delete("/{alias_uuid}")]
#[instrument(name = "Api::club_admin::delete_alias")]
pub async fn delete_alias(
    Path((club_uuid, alias_uuid)): Path<(ClubUuid, MailAliasUuid)>,
    SessionUser { uuid: _ }: SessionUser,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let alias = MailAlias::find_by_uuid(&mut tx, alias_uuid)
        .await?
        .ok_or(ApiError::bad_request("Alias not found"))?;

    // Verify domain belongs to this club
    let domain = Domain::find_by_uuid(&mut tx, alias.domain)
        .await?
        .ok_or(ApiError::bad_request("Domain not found"))?;

    if domain.associated_club != Some(club_uuid) {
        return Err(ApiError::bad_request(
            "Alias domain does not belong to this club",
        ));
    }

    // If approved, also delete from Mailcow
    if let Some(mailcow_id) = alias.mailcow_id {
        Mailcow::global()
            .sdk
            .delete_aliases(vec![mailcow_id as u64])
            .await
            .map_err(ApiError::map_server_error(
                "Could not delete alias in mailcow",
            ))?;
    }

    alias.delete(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}

/// Helper to build MailAliasSchema from aliases with domain lookups
fn build_alias_schemas(
    aliases: Vec<MailAlias>,
    domains: &[crate::models::domain::Domain],
) -> Vec<MailAliasSchema> {
    aliases
        .into_iter()
        .map(|alias| {
            let domain_name = domains
                .iter()
                .find(|d| d.uuid == alias.domain)
                .map(|d| d.domain.to_string())
                .unwrap_or_default();

            MailAliasSchema::new(alias, domain_name, String::new())
        })
        .collect()
}
