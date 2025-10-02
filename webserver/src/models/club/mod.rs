//! Clubs related models are in this module.

use std::borrow::Cow;

use futures_util::TryStreamExt;
use galvyn::core::stuff::schema::Page;
use rorm::and;
use rorm::conditions;
use rorm::conditions::BinaryOperator;
use rorm::conditions::Condition;
use rorm::conditions::DynamicCollection;
use rorm::db::Executor;
use rorm::db::transaction::Transaction;
use rorm::fields::types::MaxStr;
use rorm::prelude::ForeignModelByField;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::models::account::Account;
use crate::models::account::db::AccountModel;
use crate::models::club::db::ClubModel;
use crate::models::club::db::ClubModelInsert;
use crate::models::domain::Domain;
use crate::models::domain::db::DomainModel;
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
    /// The last point in time the club was modified
    pub modified_at: time::OffsetDateTime,
    /// The point in time the club was created
    pub created_at: time::OffsetDateTime,
    /// The number of members in the club
    pub member_count: u64,
    /// The number of admins in the club
    pub admin_count: u64,
    /// The primary domain of the club
    pub primary_domain: MaxStr<255>,
}

/// New-type for the primary key of the club
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ClubUuid(pub Uuid);

impl Club {
    /// Delete a club
    #[instrument(name = "Club::delete", skip(self, exe))]
    pub async fn delete(self, exe: impl Executor<'_>) -> anyhow::Result<()> {
        let mut guard = exe.ensure_transaction().await?;

        rorm::delete(guard.get_transaction(), DomainModel)
            .condition(DomainModel.club.equals(self.uuid.0))
            .await?;

        rorm::delete(guard.get_transaction(), ClubModel)
            .condition(ClubModel.uuid.equals(self.uuid.0))
            .await?;

        guard.commit().await?;

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
        ClubModel
            .domains
            .populate_bulk(guard.get_transaction(), &mut cm)
            .await?;

        #[allow(clippy::expect_used)]
        Ok(cm
            .into_iter()
            .map(|x| Club {
                uuid: ClubUuid(x.uuid),
                name: x.name,
                modified_at: x.modified_at,
                created_at: x.created_at,
                member_count: x.members.cached.expect("Queried beforehand").len() as u64,
                admin_count: x.admins.cached.expect("Queried beforehand").len() as u64,
                primary_domain: x
                    .domains
                    .cached
                    .expect("Queried beforehand")
                    .into_iter()
                    .find(|domain: &DomainModel| domain.is_primary)
                    .map(|x| x.domain)
                    .unwrap_or_default(),
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
        CreateClub {
            name,
            primary_domain,
        }: CreateClub<'_>,
    ) -> anyhow::Result<Club> {
        let mut guard = exe.ensure_transaction().await?;
        let mut club_model = rorm::insert(guard.get_transaction(), ClubModel)
            .single(&ClubModelInsert {
                uuid: Uuid::new_v4(),
                name,
            })
            .await?;

        ClubModel
            .domains
            .populate(guard.get_transaction(), &mut club_model)
            .await?;

        let mut club = Club {
            uuid: ClubUuid(club_model.uuid),
            name: club_model.name,
            modified_at: club_model.modified_at,
            created_at: club_model.created_at,
            member_count: 0,
            admin_count: 0,
            primary_domain: MaxStr::default(),
        };

        club.associate_domain(guard.get_transaction(), primary_domain, true)
            .await?;

        guard.commit().await?;

        Ok(club)
    }

    /// Associate an existing domain with this club
    #[instrument(name = "Club::associate_domain", skip(exe, self))]
    pub async fn associate_domain(
        &mut self,
        exe: impl Executor<'_>,
        domain: &Domain,
        is_primary: bool,
    ) -> anyhow::Result<()> {
        let mut guard = exe.ensure_transaction().await?;

        if is_primary {
            // Set primary to false on other domains for this club
            rorm::update(guard.get_transaction(), DomainModel)
                .set(DomainModel.is_primary, false)
                .condition(and![
                    DomainModel.club.equals(self.uuid.0),
                    DomainModel.is_primary.equals(true)
                ])
                .await?;
        }

        rorm::update(guard.get_transaction(), DomainModel)
            .set(DomainModel.club, Some(ForeignModelByField(self.uuid.0)))
            .set(DomainModel.is_primary, is_primary)
            .condition(DomainModel.uuid.equals(domain.uuid.0))
            .await?;

        guard.commit().await?;

        if is_primary {
            self.primary_domain = domain.domain.clone();
        }

        Ok(())
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
                            .replace('\\', "\\\\")
                            .replace('_', "\\_")
                            .replace('%', "\\%"),
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

/// Parameters for creating a club
#[derive(Debug, Clone)]
pub struct CreateClub<'a> {
    /// Name of the club
    pub name: MaxStr<255>,
    /// Primary domain to associate with the club
    pub primary_domain: &'a Domain,
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

        ClubModel
            .domains
            .populate(&mut *tx, &mut club_model)
            .await?;

        Ok(Club {
            uuid: ClubUuid(club_model.uuid),
            name: club_model.name,
            modified_at: club_model.modified_at,
            created_at: club_model.created_at,
            member_count: club_model.members.cached.unwrap().len() as u64,
            admin_count: club_model.admins.cached.unwrap().len() as u64,
            primary_domain: club_model
                .domains
                .cached
                .unwrap()
                .into_iter()
                .find(|domain: &DomainModel| domain.is_primary)
                .map(|x| x.domain)
                .unwrap_or_default(),
        })
    }
}
