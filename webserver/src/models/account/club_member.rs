use galvyn::rorm;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use tracing::instrument;

use crate::models::account::AccountUuid;
use crate::models::account::ClubAccount;
use crate::models::account::db::ClubAccountModel;
use crate::models::club::ClubUuid;

impl ClubAccount {
    /// Get the account by its uuid
    #[instrument(name = "ClubAccount::get_by_uuid", skip(exe))]
    pub async fn get_by_uuid(
        exe: impl Executor<'_>,
        uuid: AccountUuid,
    ) -> anyhow::Result<Option<Self>> {
        Ok(rorm::query(exe, ClubAccountModel)
            .condition(ClubAccountModel.uuid.equals(uuid.0))
            .optional()
            .await?
            .map(Self::from))
    }

    /// Get the account by its username
    #[instrument(name = "ClubAccount::get_by_username", skip(exe))]
    pub async fn get_by_username(
        exe: impl Executor<'_>,
        username: &MaxStr<255>,
    ) -> anyhow::Result<Option<Self>> {
        Ok(rorm::query(exe, ClubAccountModel)
            .condition(ClubAccountModel.username.equals(username))
            .optional()
            .await?
            .map(Self::from))
    }
}

impl ClubAccount {
    /// Retrieve the uuid
    pub fn uuid(&self) -> AccountUuid {
        self.uuid
    }
}

impl From<ClubAccountModel> for ClubAccount {
    fn from(value: ClubAccountModel) -> Self {
        Self {
            uuid: AccountUuid(value.uuid),
            display_name: value.display_name,
            username: value.username.0,
            email: value.email,
            club: ClubUuid(value.club.0),
            modified_at: value.modified_at,
            created_at: value.created_at,
            hashed_password: value.hashed_password,
        }
    }
}
