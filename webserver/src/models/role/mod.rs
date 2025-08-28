//! Roles are defined in this module

use futures_util::TryStreamExt;
use rorm::db::Executor;
use schemars::JsonSchema;
use tracing::instrument;

use crate::models::account::Account;
use crate::models::account::db::AccountModel;
use crate::models::club::ClubUuid;
use crate::models::role::db::SuperAdminModel;

pub(in crate::models) mod db;

/// The available roles of the manager
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(tag = "type")]
#[allow(missing_docs)]
pub enum Role {
    /// The admin of a club. Can manage users and settings of its club
    ClubAdmin { club_uuid: ClubUuid },
    /// A member of a club.
    ClubMember { club_uuid: ClubUuid },
    /// The super administrator. Has rights to manager clubs.
    SuperAdmin,
}

impl Role {
    /// Returns all superadmins
    #[instrument(skip(exe))]
    pub async fn find_all_superadmins(exe: impl Executor<'_>) -> anyhow::Result<Vec<Account>> {
        Ok(
            rorm::query(exe, SuperAdminModel.account.query_as(AccountModel))
                .stream()
                .map_ok(Account::from)
                .try_collect()
                .await?,
        )
    }
}
