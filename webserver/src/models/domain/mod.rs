//! This module provides the domain model and related functionality for managing domains in the system.
//!
//! The domain model represents a DNS domain that can optionally be associated with a club.
//! It includes queries to find domains based on their association status with clubs.

use futures_util::TryStreamExt;
use rorm::db::Executor;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::models::club::ClubUuid;
use crate::models::domain::db::DomainModel;

pub(in crate::models) mod db;

/// Domain representation
#[derive(Debug, Clone)]
pub struct Domain {
    /// Internal identifier of the domain
    pub uuid: DomainUuid,
    /// Domain
    pub domain: MaxStr<255>,
    /// Optionally associated club
    pub associated_club: Option<ClubUuid>,
}

/// Uuid of a domain
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema, Eq, Hash, PartialEq)]
pub struct DomainUuid(pub Uuid);

impl Domain {
    /// Find all domains that are unassociated
    #[instrument(name = "Domain::find_all_unassociated", skip(exe))]
    pub async fn find_all_unassociated(exe: impl Executor<'_>) -> anyhow::Result<Vec<Self>> {
        Ok(rorm::query(exe, DomainModel)
            .order_asc(DomainModel.domain)
            .condition(DomainModel.club.is_none())
            .stream()
            .map_ok(Domain::from)
            .try_collect()
            .await?)
    }

    /// Find all domains that are associated with a club
    #[instrument(name = "Domain::find_all_associated", skip(exe))]
    pub async fn find_all_associated(exe: impl Executor<'_>) -> anyhow::Result<Vec<Self>> {
        Ok(rorm::query(exe, DomainModel)
            .order_asc(DomainModel.domain)
            .condition(DomainModel.club.is_some())
            .stream()
            .map_ok(Domain::from)
            .try_collect()
            .await?)
    }

    /// Find all domains that are associated with a club
    #[instrument(name = "Domain::find_all_by_club", skip(exe))]
    pub async fn find_all_by_club(
        exe: impl Executor<'_>,
        club: ClubUuid,
    ) -> anyhow::Result<Vec<Self>> {
        Ok(rorm::query(exe, DomainModel)
            .order_asc(DomainModel.domain)
            .condition(DomainModel.club.equals(club.0))
            .stream()
            .map_ok(Domain::from)
            .try_collect()
            .await?)
    }

    /// Delete a domain by its domain
    #[instrument(name = "Domain::delete_by_domain", skip(exe))]
    pub async fn delete_by_domain(
        exe: impl Executor<'_>,
        domain: MaxStr<255>,
    ) -> anyhow::Result<()> {
        rorm::delete(exe, DomainModel)
            .condition(DomainModel.domain.equals(&*domain))
            .await?;

        Ok(())
    }

    /// Create a new domain
    #[instrument(name = "Domain::create", skip(exe))]
    pub async fn create(exe: impl Executor<'_>, domain: MaxStr<255>) -> anyhow::Result<Self> {
        let domain = rorm::insert(exe, DomainModel)
            .single(&DomainModel {
                uuid: Uuid::new_v4(),
                domain,
                club: None,
            })
            .await?;

        Ok(Self::from(domain))
    }
}

impl From<DomainModel> for Domain {
    fn from(value: DomainModel) -> Self {
        Self {
            uuid: DomainUuid(value.uuid),
            domain: value.domain,
            associated_club: value.club.map(|x| ClubUuid(x.0)),
        }
    }
}
