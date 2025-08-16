//! Clubs related models are in this module.

use rorm::db::Executor;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use tracing::instrument;
use uuid::Uuid;

use crate::models::club::db::ClubModel;
use crate::models::club::db::ClubModelInsert;

pub(in crate::models) mod db;

/// Representation of a club
#[derive(Debug, Clone)]
pub struct Club {
    /// Primary key of a club
    pub uuid: ClubUuid,
    /// Name of the club
    pub name: MaxStr<255>,
    /// Description for a club
    pub description: MaxStr<1024>,
    /// The last point in time the club was modified
    pub modified_at: time::OffsetDateTime,
    /// The point in time the club was created
    pub created_at: time::OffsetDateTime,
    /// The number of members in the club
    pub member_count: u64,
    /// The number of admins in the club
    pub admin_count: u64,
}

/// New-type for the primary key of the club
#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ClubUuid(pub Uuid);

impl Club {
    /// Delete a club
    #[instrument(name = "Club::delete", skip(self, exe))]
    pub async fn delete(self, exe: impl Executor<'_>) -> anyhow::Result<()> {
        rorm::delete(exe, ClubModel)
            .condition(ClubModel.uuid.equals(self.uuid.0))
            .await?;

        Ok(())
    }

    /// Retrieve a club by uuid
    #[instrument(name = "Club::find_by_uuid", skip(exe))]
    pub async fn find_by_uuid(
        exe: impl Executor<'_>,
        uuid: ClubUuid,
    ) -> anyhow::Result<Option<Club>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut cm = rorm::query(guard.get_transaction(), ClubModel)
            .condition(ClubModel.uuid.equals(uuid.0))
            .optional()
            .await?;

        if let Some(ref mut cm) = cm {
            ClubModel
                .admins
                .populate(guard.get_transaction(), cm)
                .await?;
            ClubModel
                .members
                .populate(guard.get_transaction(), cm)
                .await?;
        }

        guard.commit().await?;

        #[allow(clippy::expect_used)]
        Ok(cm.map(|x| Self {
            uuid: ClubUuid(x.uuid),
            name: x.name,
            description: x.description,
            modified_at: x.modified_at,
            created_at: x.created_at,
            member_count: x.members.cached.unwrap().len() as u64,
            admin_count: x.admins.cached.unwrap().len() as u64,
        }))
    }

    /// Retrieve all clubs
    #[instrument(name = "Club::find_all", skip(exe))]
    pub async fn find_all(exe: impl Executor<'_>) -> anyhow::Result<Vec<Club>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut cm = rorm::query(guard.get_transaction(), ClubModel)
            .all()
            .await?;

        ClubModel
            .admins
            .populate_bulk(guard.get_transaction(), &mut cm)
            .await?;
        ClubModel
            .members
            .populate_bulk(guard.get_transaction(), &mut cm)
            .await?;

        #[allow(clippy::expect_used)]
        Ok(cm
            .into_iter()
            .map(|x| Club {
                uuid: ClubUuid(x.uuid),
                name: x.name,
                description: x.description,
                modified_at: x.modified_at,
                created_at: x.created_at,
                member_count: x.members.cached.expect("Queried beforehand").len() as u64,
                admin_count: x.admins.cached.expect("Queried beforehand").len() as u64,
            })
            .collect())
    }

    /// Retrieve a club by name
    #[instrument(name = "Club::find_by_name", skip(exe))]
    pub async fn find_by_name(
        exe: impl Executor<'_>,
        name: &MaxStr<255>,
    ) -> anyhow::Result<Option<Club>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut cm = rorm::query(guard.get_transaction(), ClubModel)
            .condition(ClubModel.name.equals(&**name))
            .optional()
            .await?;

        if let Some(ref mut cm) = cm {
            ClubModel
                .admins
                .populate(guard.get_transaction(), cm)
                .await?;
            ClubModel
                .members
                .populate(guard.get_transaction(), cm)
                .await?;
        }

        guard.commit().await?;

        #[allow(clippy::expect_used)]
        Ok(cm.map(|x| Self {
            uuid: ClubUuid(x.uuid),
            name: x.name,
            description: x.description,
            modified_at: x.modified_at,
            created_at: x.created_at,
            member_count: x.members.cached.unwrap().len() as u64,
            admin_count: x.admins.cached.unwrap().len() as u64,
        }))
    }

    /// Create a new club
    #[instrument(name = "Club::create", skip(exe))]
    pub async fn create(
        exe: impl Executor<'_>,
        name: MaxStr<255>,
        description: MaxStr<1024>,
    ) -> anyhow::Result<Club> {
        let club_model = rorm::insert(exe, ClubModel)
            .single(&ClubModelInsert {
                uuid: Uuid::new_v4(),
                name,
                description,
            })
            .await?;

        Ok(Club {
            uuid: ClubUuid(club_model.uuid),
            name: club_model.name,
            description: club_model.description,
            modified_at: club_model.modified_at,
            created_at: club_model.created_at,
            member_count: 0,
            admin_count: 0,
        })
    }
}
