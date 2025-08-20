//! Clubs related models are in this module.

use std::borrow::Cow;

use futures_util::TryStreamExt;
use galvyn::core::stuff::schema::Page;
use rorm::conditions;
use rorm::conditions::BinaryOperator;
use rorm::conditions::Condition;
use rorm::conditions::DynamicCollection;
use rorm::db::Executor;
use rorm::db::transaction::Transaction;
use rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use tracing::instrument;
use uuid::Uuid;

use crate::models::account::Account;
use crate::models::account::db::AccountModel;
use crate::models::club::db::ClubModel;
use crate::models::club::db::ClubModelInsert;
use crate::models::role::db::ClubAdminModel;
use crate::models::role::db::ClubMemberModel;

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

    /// Retrieve all clubs
    #[instrument(name = "Club::find_all", skip(exe))]
    pub async fn find_all(exe: impl Executor<'_>) -> anyhow::Result<Vec<Club>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut cm = rorm::query(guard.get_transaction(), ClubModel)
            .order_asc(ClubModel.name)
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

    /// Retrieve a club by uuid
    #[instrument(name = "Club::find_by_uuid", skip(exe))]
    pub async fn find_by_uuid(
        exe: impl Executor<'_>,
        uuid: ClubUuid,
    ) -> anyhow::Result<Option<Club>> {
        let mut guard = exe.ensure_transaction().await?;

        let cm = rorm::query(guard.get_transaction(), ClubModel)
            .condition(ClubModel.uuid.equals(uuid.0))
            .optional()
            .await?;

        let club = if let Some(cm) = cm {
            Some(Self::populate(guard.get_transaction(), cm).await?)
        } else {
            None
        };

        guard.commit().await?;

        Ok(club)
    }

    /// Retrieve a club by name
    #[instrument(name = "Club::find_by_name", skip(exe))]
    pub async fn find_by_name(
        exe: impl Executor<'_>,
        name: &MaxStr<255>,
    ) -> anyhow::Result<Option<Club>> {
        let mut guard = exe.ensure_transaction().await?;

        let cm = rorm::query(guard.get_transaction(), ClubModel)
            .condition(ClubModel.name.equals(&**name))
            .optional()
            .await?;

        let club = if let Some(cm) = cm {
            Some(Self::populate(guard.get_transaction(), cm).await?)
        } else {
            None
        };

        guard.commit().await?;

        Ok(club)
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

    /// Retrieve all members of a club
    #[instrument(name = "Club::members", skip(exe, self))]
    pub async fn members_page(
        &self,
        exe: impl Executor<'_>,
        limit: u64,
        offset: u64,
        search: Option<MaxStr<255>>,
    ) -> anyhow::Result<Page<Account>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut conditions = vec![ClubMemberModel.club.equals(self.uuid.0).boxed()];
        if let Some(search) = search {
            conditions.push(
                conditions::Binary {
                    operator: BinaryOperator::Like,
                    fst_arg: conditions::Column(ClubMemberModel.account.username),
                    snd_arg: conditions::Value::String(Cow::Owned(format!(
                        "%{}%",
                        search
                            .replace('_', "\\_")
                            .replace('%', "\\%")
                            .replace('\\', "\\\\"),
                    ))),
                }
                .boxed(),
            );
        }
        let cond_collection = DynamicCollection::and(conditions);

        let account_models = rorm::query(
            guard.get_transaction(),
            ClubMemberModel.account.query_as(AccountModel),
        )
        .order_asc(ClubMemberModel.account.username)
        .condition(&cond_collection)
        .offset(offset)
        .limit(limit)
        .stream()
        .map_ok(Account::from)
        .try_collect::<Vec<_>>()
        .await?;

        let total = rorm::query(guard.get_transaction(), ClubMemberModel.uuid.count())
            .condition(ClubMemberModel.club.equals(self.uuid.0))
            .one()
            .await?;

        guard.commit().await?;

        Ok(Page {
            items: account_models,
            limit,
            offset,
            total,
        })
    }

    /// Retrieve all admins of a club
    #[instrument(name = "Club::admins_page", skip(exe, self))]
    pub async fn admins_page(
        &self,
        exe: impl Executor<'_>,
        limit: u64,
        offset: u64,
        search: Option<MaxStr<255>>,
    ) -> anyhow::Result<Page<Account>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut conditions = vec![ClubAdminModel.club.equals(self.uuid.0).boxed()];
        if let Some(search) = search {
            conditions.push(
                conditions::Binary {
                    operator: BinaryOperator::Like,
                    fst_arg: conditions::Column(ClubAdminModel.account.username),
                    snd_arg: conditions::Value::String(Cow::Owned(format!(
                        "%{}%",
                        search
                            .replace('_', "\\_")
                            .replace('%', "\\%")
                            .replace('\\', "\\\\"),
                    ))),
                }
                .boxed(),
            );
        }
        let cond_collection = DynamicCollection::and(conditions);

        let account_models = rorm::query(
            guard.get_transaction(),
            ClubAdminModel.account.query_as(AccountModel),
        )
        .order_asc(ClubAdminModel.account.username)
        .condition(&cond_collection)
        .offset(offset)
        .limit(limit)
        .stream()
        .map_ok(Account::from)
        .try_collect::<Vec<_>>()
        .await?;

        let total = rorm::query(guard.get_transaction(), ClubAdminModel.uuid.count())
            .condition(ClubAdminModel.club.equals(self.uuid.0))
            .one()
            .await?;

        guard.commit().await?;

        Ok(Page {
            items: account_models,
            limit,
            offset,
            total,
        })
    }
}

impl Club {
    async fn populate(
        tx: &mut Transaction,
        mut club_model: ClubModel,
    ) -> Result<Self, anyhow::Error> {
        ClubModel.admins.populate(&mut *tx, &mut club_model).await?;
        ClubModel
            .members
            .populate(&mut *tx, &mut club_model)
            .await?;

        Ok(Club {
            uuid: ClubUuid(club_model.uuid),
            name: club_model.name,
            description: club_model.description,
            modified_at: club_model.modified_at,
            created_at: club_model.created_at,
            member_count: club_model.members.cached.unwrap().len() as u64,
            admin_count: club_model.admins.cached.unwrap().len() as u64,
        })
    }
}
