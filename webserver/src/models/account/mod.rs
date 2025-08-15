//! Accounts are the login-related models of this platform.
//!
//! They are attached to the related models that grant access to clubs or the super administrative
//! users.

use anyhow::anyhow;
use futures_util::TryStreamExt;
use rorm::and;
use rorm::db::Executor;
use rorm::fields::types::MaxStr;
use rorm::prelude::ForeignModelByField;
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::models::account::db::AccountModel;
use crate::models::club::ClubUuid;
use crate::models::role::Role;
use crate::models::role::db::ClubAdminModel;
use crate::models::role::db::ClubMemberModel;
use crate::models::role::db::SuperAdminModel;

pub(in crate::models) mod db;

/// Representation of the login data without any permission attached to it
pub struct Account {
    /// Primary key of the account
    pub uuid: AccountUuid,
    /// Name to be used for displaying purposes
    pub display_name: MaxStr<255>,
    /// The username that should be used for logging in
    pub username: MaxStr<255>,
    /// The last point in time the account was modified
    pub modified_at: time::OffsetDateTime,
    /// The point in time the account was created
    pub created_at: time::OffsetDateTime,
    hashed_password: MaxStr<255>,
}

/// New-type for the account's primary key
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AccountUuid(pub Uuid);

impl Account {
    /// Find an account by its primary key
    #[instrument(skip(exe))]
    pub async fn find_by_uuid(
        exe: impl Executor<'_>,
        AccountUuid(uuid): AccountUuid,
    ) -> anyhow::Result<Option<Self>> {
        rorm::query(exe, AccountModel)
            .condition(AccountModel.uuid.equals(uuid))
            .optional()
            .await
            .map(|x| x.map(Account::from))
            .map_err(|err| anyhow!("Database error: {err}"))
    }

    /// Find an account by its username
    #[instrument(skip(exe))]
    pub async fn find_by_username(
        exe: impl Executor<'_>,
        username: &str,
    ) -> anyhow::Result<Option<Self>> {
        rorm::query(exe, AccountModel)
            .condition(AccountModel.username.equals(username))
            .optional()
            .await
            .map(|x| x.map(Account::from))
            .map_err(|err| anyhow!("Database error: {err}"))
    }
}
impl Account {
    /// Set a new password for an account
    #[instrument(skip(self, exe))]
    pub async fn set_password(
        &self,
        exe: impl Executor<'_>,
        password: MaxStr<72>,
    ) -> anyhow::Result<()> {
        #[allow(clippy::expect_used)]
        let hashed_password = MaxStr::new(Self::hash_password(&password)?)
            .expect("Resulting hash must be <255 bytes");

        rorm::update(exe, AccountModel)
            .set(AccountModel.hashed_password, hashed_password)
            .condition(AccountModel.uuid.equals(self.uuid.0))
            .await?;

        Ok(())
    }

    /// Check whether the given password matches the stored one
    #[instrument(skip(self))]
    pub fn check_password(&self, password: MaxStr<72>) -> anyhow::Result<bool> {
        bcrypt::verify(&*password, &self.hashed_password).map_err(|err| anyhow!("{err}"))
    }

    /// Add a new role to the account
    #[instrument(skip(self, exe))]
    pub async fn add_role(&self, exe: impl Executor<'_>, role: Role) -> anyhow::Result<()> {
        let mut guard = exe.ensure_transaction().await?;

        // Check whether the user already has the desired role
        let existing = self.roles(guard.get_transaction()).await?;
        if existing.contains(&role) {
            return Ok(());
        }

        match role {
            Role::SuperAdmin => {
                rorm::insert(guard.get_transaction(), SuperAdminModel)
                    .single(&SuperAdminModel {
                        uuid: Uuid::new_v4(),
                        account: ForeignModelByField(self.uuid.0),
                    })
                    .await?;
            }
            Role::ClubAdmin(ClubUuid(uuid)) => {
                rorm::insert(guard.get_transaction(), ClubAdminModel)
                    .single(&ClubAdminModel {
                        uuid: Uuid::new_v4(),
                        account: ForeignModelByField(self.uuid.0),
                        club: ForeignModelByField(uuid),
                    })
                    .await?;
            }
            Role::ClubMember(ClubUuid(uuid)) => {
                rorm::insert(guard.get_transaction(), ClubMemberModel)
                    .single(&ClubMemberModel {
                        uuid: Uuid::new_v4(),
                        account: ForeignModelByField(self.uuid.0),
                        club: ForeignModelByField(uuid),
                    })
                    .await?;
            }
        }

        guard.commit().await?;

        Ok(())
    }

    /// Remove an existing role from the account
    #[instrument(skip(self, exe))]
    pub async fn remove_role(&self, exe: impl Executor<'_>, role: Role) -> anyhow::Result<()> {
        let mut guard = exe.ensure_transaction().await?;

        match role {
            Role::SuperAdmin => {
                rorm::delete(guard.get_transaction(), SuperAdminModel)
                    .condition(SuperAdminModel.account.equals(self.uuid.0))
                    .await?;
            }
            Role::ClubAdmin(ClubUuid(club_uuid)) => {
                rorm::delete(guard.get_transaction(), ClubAdminModel)
                    .condition(and![
                        ClubAdminModel.club.equals(club_uuid),
                        ClubAdminModel.account.equals(self.uuid.0),
                    ])
                    .await?;
            }
            Role::ClubMember(ClubUuid(club_uuid)) => {
                rorm::delete(guard.get_transaction(), ClubMemberModel)
                    .condition(and![
                        ClubMemberModel.club.equals(club_uuid),
                        ClubMemberModel.account.equals(self.uuid.0),
                    ])
                    .await?;
            }
        }

        guard.commit().await?;

        Ok(())
    }

    /// Retrieve the current roles of the user
    #[instrument(skip(self, exe))]
    pub async fn roles(&self, exe: impl Executor<'_>) -> anyhow::Result<Vec<Role>> {
        let mut roles = vec![];

        let mut guard = exe.ensure_transaction().await?;

        // Superadmin
        let is_super_admin = rorm::query(guard.get_transaction(), SuperAdminModel)
            .condition(SuperAdminModel.account.equals(self.uuid.0))
            .optional()
            .await?;

        if is_super_admin.is_some() {
            roles.push(Role::SuperAdmin);
        }

        // ClubAdmin
        let admin_roles: Vec<Role> = rorm::query(guard.get_transaction(), ClubAdminModel)
            .condition(ClubAdminModel.account.equals(self.uuid.0))
            .stream()
            .map_ok(|x| Role::ClubAdmin(ClubUuid(x.club.0)))
            .try_collect()
            .await?;
        roles.extend(admin_roles);

        // ClubMember
        let member_roles: Vec<Role> = rorm::query(guard.get_transaction(), ClubMemberModel)
            .condition(ClubMemberModel.account.equals(self.uuid.0))
            .stream()
            .map_ok(|x| Role::ClubMember(ClubUuid(x.club.0)))
            .try_collect()
            .await?;
        roles.extend(member_roles);

        guard.commit().await?;

        Ok(roles)
    }
}

impl Account {
    pub(in crate::models) fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        const HASH_COST: u32 = 12;
        bcrypt::hash(password, HASH_COST)
    }
}

impl From<AccountModel> for Account {
    fn from(value: AccountModel) -> Self {
        Self {
            uuid: AccountUuid(value.uuid),
            display_name: value.display_name,
            username: value.username,
            modified_at: value.modified_at,
            created_at: value.created_at,
            hashed_password: value.hashed_password,
        }
    }
}
