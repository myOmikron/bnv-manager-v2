//! Mail alias related code lives in this module

use futures_util::TryStreamExt;
use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::rorm;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::prelude::ForeignModelByField;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::models::account::AccountUuid;
use crate::models::club::ClubUuid;
use crate::models::domain::DomainUuid;
use crate::models::mail_alias::db::MailAliasModel;
use crate::models::mail_alias::db::MailAliasModelInsert;

pub(in crate::models) mod db;

/// Wrapper for the primary key of a [MailAlias]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MailAliasUuid(pub Uuid);

/// Status of a mail alias
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum MailAliasStatus {
    /// Waiting for admin approval
    Pending,
    /// Approved and active in Mailcow
    Approved,
    /// Rejected by admin
    Rejected,
}

impl MailAliasStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "approved" => Some(Self::Approved),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}

/// A mail alias
#[derive(Debug, Clone)]
pub struct MailAlias {
    /// Primary key
    pub uuid: MailAliasUuid,
    /// Local part of the alias (before the @)
    pub local_part: MaxStr<255>,
    /// Domain of the alias
    pub domain: DomainUuid,
    /// Account that owns the alias
    pub account: AccountUuid,
    /// Current status
    pub status: MailAliasStatus,
    /// Mailcow alias ID (set when created in Mailcow)
    pub mailcow_id: Option<i64>,
    /// The point in time the alias was created
    pub created_at: time::OffsetDateTime,
    /// The point in time the alias was last modified
    pub modified_at: time::OffsetDateTime,
}

impl MailAlias {
    /// Find a mail alias by its uuid
    #[instrument(skip(exe))]
    pub async fn find_by_uuid(
        exe: impl Executor<'_>,
        MailAliasUuid(uuid): MailAliasUuid,
    ) -> anyhow::Result<Option<Self>> {
        Ok(rorm::query(exe, MailAliasModel)
            .condition(MailAliasModel.uuid.equals(uuid))
            .optional()
            .await?
            .map(MailAlias::from))
    }

    /// Find all aliases for a given account
    #[instrument(name = "MailAlias::find_all_by_account", skip(exe))]
    pub async fn find_all_by_account(
        exe: impl Executor<'_>,
        AccountUuid(account_uuid): AccountUuid,
    ) -> anyhow::Result<Vec<Self>> {
        Ok(rorm::query(exe, MailAliasModel)
            .condition(MailAliasModel.account.equals(account_uuid))
            .order_desc(MailAliasModel.created_at)
            .stream()
            .map_ok(MailAlias::from)
            .try_collect()
            .await?)
    }

    /// Find all aliases for a given club (via domain association)
    ///
    /// Fetches the club's domain UUIDs first, then filters aliases by those domains.
    #[instrument(name = "MailAlias::find_all_by_club", skip(exe))]
    pub async fn find_all_by_club(
        exe: impl Executor<'_>,
        club_uuid: ClubUuid,
    ) -> anyhow::Result<Vec<Self>> {
        let mut guard = exe.ensure_transaction().await?;

        let domains =
            crate::models::domain::Domain::find_all_by_club(guard.get_transaction(), club_uuid)
                .await?;
        let domain_uuids: std::collections::HashSet<Uuid> =
            domains.iter().map(|d| d.uuid.0).collect();

        let all_aliases: Vec<Self> = rorm::query(guard.get_transaction(), MailAliasModel)
            .order_desc(MailAliasModel.created_at)
            .stream()
            .map_ok(MailAlias::from)
            .try_collect()
            .await?;

        guard.commit().await?;

        Ok(all_aliases
            .into_iter()
            .filter(|a| domain_uuids.contains(&a.domain.0))
            .collect())
    }

    /// Find all pending aliases for a given club
    #[instrument(name = "MailAlias::find_pending_by_club", skip(exe))]
    pub async fn find_pending_by_club(
        exe: impl Executor<'_>,
        club_uuid: ClubUuid,
    ) -> anyhow::Result<Vec<Self>> {
        let mut guard = exe.ensure_transaction().await?;

        let domains =
            crate::models::domain::Domain::find_all_by_club(guard.get_transaction(), club_uuid)
                .await?;
        let domain_uuids: std::collections::HashSet<Uuid> =
            domains.iter().map(|d| d.uuid.0).collect();

        #[allow(clippy::expect_used)]
        let pending: MaxStr<32> =
            MaxStr::new("pending".to_string()).expect("Status string is always < 32 chars");

        let all_aliases: Vec<Self> = rorm::query(guard.get_transaction(), MailAliasModel)
            .condition(MailAliasModel.status.equals(&pending))
            .order_desc(MailAliasModel.created_at)
            .stream()
            .map_ok(MailAlias::from)
            .try_collect()
            .await?;

        guard.commit().await?;

        Ok(all_aliases
            .into_iter()
            .filter(|a| domain_uuids.contains(&a.domain.0))
            .collect())
    }

    /// Check if an alias with the given address already exists
    #[instrument(name = "MailAlias::exists_by_address", skip(exe))]
    pub async fn exists_by_address(
        exe: impl Executor<'_>,
        local_part: &MaxStr<255>,
        DomainUuid(domain_uuid): DomainUuid,
    ) -> anyhow::Result<bool> {
        Ok(rorm::query(exe, MailAliasModel)
            .condition(rorm::and![
                MailAliasModel.local_part.equals(&**local_part),
                MailAliasModel.domain.equals(domain_uuid),
            ])
            .optional()
            .await?
            .is_some())
    }

    /// Create a new mail alias with status "pending"
    #[instrument(skip(exe))]
    pub async fn create(
        exe: impl Executor<'_>,
        params: CreateMailAliasParams,
    ) -> anyhow::Result<Result<Self, CreateMailAliasError>> {
        let mut guard = exe.ensure_transaction().await?;

        // Check if alias already exists
        if Self::exists_by_address(guard.get_transaction(), &params.local_part, params.domain)
            .await?
        {
            return Ok(Err(CreateMailAliasError::AliasAlreadyExists));
        }

        #[allow(clippy::expect_used)]
        let status = MaxStr::new(MailAliasStatus::Pending.as_str().to_string())
            .expect("Status string is always < 32 chars");

        let model = rorm::insert(guard.get_transaction(), MailAliasModel)
            .single(&MailAliasModelInsert {
                uuid: Uuid::new_v4(),
                local_part: params.local_part,
                domain: ForeignModelByField(params.domain.0),
                account: ForeignModelByField(params.account.0),
                status,
            })
            .await?;

        guard.commit().await?;

        Ok(Ok(MailAlias::from(model)))
    }

    /// Approve this alias (sets status to approved and stores mailcow_id)
    #[instrument(skip(exe))]
    pub async fn approve(&self, exe: impl Executor<'_>, mailcow_id: i64) -> anyhow::Result<()> {
        #[allow(clippy::expect_used)]
        let approved: MaxStr<32> =
            MaxStr::new("approved".to_string()).expect("Status string is always < 32 chars");

        rorm::update(exe, MailAliasModel)
            .set(MailAliasModel.status, approved)
            .set(MailAliasModel.mailcow_id, Some(mailcow_id))
            .condition(MailAliasModel.uuid.equals(self.uuid.0))
            .await?;

        Ok(())
    }

    /// Reject this alias
    #[instrument(skip(exe))]
    pub async fn reject(&self, exe: impl Executor<'_>) -> anyhow::Result<()> {
        #[allow(clippy::expect_used)]
        let rejected: MaxStr<32> =
            MaxStr::new("rejected".to_string()).expect("Status string is always < 32 chars");

        rorm::update(exe, MailAliasModel)
            .set(MailAliasModel.status, rejected)
            .condition(MailAliasModel.uuid.equals(self.uuid.0))
            .await?;

        Ok(())
    }

    /// Delete this alias
    #[instrument(skip(exe))]
    pub async fn delete(self, exe: impl Executor<'_>) -> anyhow::Result<()> {
        rorm::delete(exe, MailAliasModel)
            .condition(MailAliasModel.uuid.equals(self.uuid.0))
            .await?;

        Ok(())
    }
}

/// Parameters for creating a new mail alias
#[derive(Debug, Clone)]
pub struct CreateMailAliasParams {
    /// Local part of the alias address
    pub local_part: MaxStr<255>,
    /// Domain UUID
    pub domain: DomainUuid,
    /// Account UUID of the requesting member
    pub account: AccountUuid,
}

/// Errors when creating a mail alias
#[derive(Debug, Clone, Error)]
#[allow(missing_docs)]
pub enum CreateMailAliasError {
    #[error("Alias address already exists")]
    AliasAlreadyExists,
}

impl From<MailAliasModel> for MailAlias {
    fn from(value: MailAliasModel) -> Self {
        Self {
            uuid: MailAliasUuid(value.uuid),
            local_part: value.local_part,
            domain: DomainUuid(value.domain.0),
            account: AccountUuid(value.account.0),
            #[allow(clippy::unwrap_used)]
            status: MailAliasStatus::from_str(&value.status).unwrap(),
            mailcow_id: value.mailcow_id,
            created_at: value.created_at,
            modified_at: value.modified_at,
        }
    }
}
