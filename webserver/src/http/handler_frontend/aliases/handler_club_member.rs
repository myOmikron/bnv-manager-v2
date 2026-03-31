//! Endpoints for club members to manage their aliases.

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::FormResult;
use galvyn::delete;
use galvyn::get;
use galvyn::post;
use galvyn::rorm::Database;
use tracing::instrument;

use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::aliases::DomainOptionSchema;
use crate::http::handler_frontend::aliases::MailAliasSchema;
use crate::http::handler_frontend::aliases::ProposeAliasError;
use crate::http::handler_frontend::aliases::ProposeAliasRequest;
use crate::models::account::ClubAccount;
use crate::models::domain::Domain;
use crate::models::mail_alias::CreateMailAliasParams;
use crate::models::mail_alias::MailAlias;
use crate::models::mail_alias::MailAliasUuid;
use crate::modules::mailcow::Mailcow;

/// Validates that a local part is a valid email local part
fn is_valid_local_part(local_part: &str) -> bool {
    if local_part.is_empty() || local_part.len() > 64 {
        return false;
    }

    local_part
        .chars()
        .all(|c| c.is_alphanumeric() || ".-_+".contains(c))
}

#[get("/")]
#[instrument(name = "Api::club_member::get_my_aliases")]
pub async fn get_my_aliases(
    SessionUser { uuid: account_uuid }: SessionUser,
) -> ApiResult<ApiJson<Vec<MailAliasSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let account = ClubAccount::get_by_uuid(&mut tx, account_uuid)
        .await?
        .ok_or(ApiError::bad_request("Account not found"))?;

    let aliases = MailAlias::find_all_by_account(&mut tx, account_uuid).await?;

    let domains = Domain::find_all_by_club(&mut tx, account.club).await?;

    tx.commit().await?;

    let schemas = aliases
        .into_iter()
        .map(|alias| {
            let domain_name = domains
                .iter()
                .find(|d| d.uuid == alias.domain)
                .map(|d| d.domain.to_string())
                .unwrap_or_default();
            MailAliasSchema::new(alias, domain_name, account.display_name.to_string())
        })
        .collect();

    Ok(ApiJson(schemas))
}

#[get("/domains")]
#[instrument(name = "Api::club_member::get_my_club_domains")]
pub async fn get_my_club_domains(
    SessionUser { uuid: account_uuid }: SessionUser,
) -> ApiResult<ApiJson<Vec<DomainOptionSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let account = ClubAccount::get_by_uuid(&mut tx, account_uuid)
        .await?
        .ok_or(ApiError::bad_request("Account not found"))?;

    let domains = Domain::find_all_by_club(&mut tx, account.club).await?;

    tx.commit().await?;

    Ok(ApiJson(
        domains
            .into_iter()
            .map(|d| DomainOptionSchema {
                uuid: d.uuid,
                domain: d.domain.to_string(),
            })
            .collect(),
    ))
}

#[post("/")]
#[instrument(name = "Api::club_member::propose_alias")]
pub async fn propose_alias(
    SessionUser { uuid: account_uuid }: SessionUser,
    ApiJson(ProposeAliasRequest {
        local_part,
        domain_uuid,
    }): ApiJson<ProposeAliasRequest>,
) -> ApiResult<ApiJson<FormResult<MailAliasSchema, ProposeAliasError>>> {
    if !is_valid_local_part(&local_part) {
        return Ok(ApiJson(FormResult::err(ProposeAliasError {
            invalid_local_part: true,
            alias_already_taken: false,
            domain_not_in_club: false,
        })));
    }

    let mut tx = Database::global().start_transaction().await?;

    let account = ClubAccount::get_by_uuid(&mut tx, account_uuid)
        .await?
        .ok_or(ApiError::bad_request("Account not found"))?;

    // Validate domain belongs to the member's club
    let domain = Domain::find_by_uuid(&mut tx, domain_uuid)
        .await?
        .ok_or(ApiError::bad_request("Domain not found"))?;

    if domain.associated_club != Some(account.club) {
        return Ok(ApiJson(FormResult::err(ProposeAliasError {
            domain_not_in_club: true,
            alias_already_taken: false,
            invalid_local_part: false,
        })));
    }

    let result = MailAlias::create(
        &mut tx,
        CreateMailAliasParams {
            local_part: local_part.clone(),
            domain: domain_uuid,
            account: account_uuid,
        },
    )
    .await?;

    let alias = match result {
        Ok(alias) => alias,
        Err(_) => {
            return Ok(ApiJson(FormResult::err(ProposeAliasError {
                alias_already_taken: true,
                domain_not_in_club: false,
                invalid_local_part: false,
            })));
        }
    };

    tx.commit().await?;

    Ok(ApiJson(FormResult::ok(MailAliasSchema::new(
        alias,
        domain.domain.to_string(),
        account.display_name.to_string(),
    ))))
}

#[delete("/{alias_uuid}")]
#[instrument(name = "Api::club_member::delete_alias")]
pub async fn delete_my_alias(
    SessionUser { uuid: account_uuid }: SessionUser,
    Path(alias_uuid): Path<MailAliasUuid>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let alias = MailAlias::find_by_uuid(&mut tx, alias_uuid)
        .await?
        .ok_or(ApiError::bad_request("Alias not found"))?;

    if alias.account.0 != account_uuid.0 {
        return Err(ApiError::bad_request("Alias does not belong to you"));
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
