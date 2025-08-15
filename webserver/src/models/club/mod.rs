//! Clubs related models are in this module.

use rorm::db::Executor;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use tracing::instrument;
use uuid::Uuid;

use crate::models::club::db::ClubModel;

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
}

/// New-type for the primary key of the club
#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ClubUuid(pub Uuid);

impl Club {
    /// Retrieve a club by uuid
    #[instrument(skip(exe))]
    pub async fn get_by_uuid(
        exe: impl Executor<'_>,
        uuid: ClubUuid,
    ) -> anyhow::Result<Option<Club>> {
        Ok(rorm::query(exe, ClubModel)
            .condition(ClubModel.uuid.equals(uuid.0))
            .optional()
            .await?
            .map(Club::from))
    }
}

impl From<ClubModel> for Club {
    fn from(value: ClubModel) -> Self {
        Self {
            uuid: ClubUuid(value.uuid),
            name: value.name,
            description: value.description,
            modified_at: value.modified_at,
            created_at: value.created_at,
        }
    }
}
