//! Accounts are the login-related models of this platform.
//!
//! They are attached to the related models that grant access to clubs or the super administrative
//! users.

use galvyn::rorm;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::models::account::db::AdministrativeAccountModel;
use crate::models::account::db::ClubAccountModel;
use crate::models::account::db::ClubAdminAccountModel;
use crate::models::club::ClubUuid;

mod club_admin;
mod club_member;
pub(in crate::models) mod db;
mod superadmin;

/// Helper for unifying the different account types
pub enum Account {
    /// Member of a club
    ClubMember(ClubAccount),
    /// Admin of a club
    ClubAdmin(ClubAdminAccount),
    /// Superadmin
    Superadmin(AdministrativeAccount),
}

/// Representation of the login data without any permission attached to it
pub struct AdministrativeAccount {
    /// Primary key of the account
    uuid: AccountUuid,
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

/// Representation of the login data without any permission attached to it
pub struct ClubAdminAccount {
    /// Primary key of the account
    uuid: AccountUuid,
    /// Name to be used for displaying purposes
    pub display_name: MaxStr<255>,
    /// The username that should be used for logging in
    pub username: MaxStr<255>,
    /// The club this account is an admin for
    pub club: ClubUuid,
    /// The last point in time the account was modified
    pub modified_at: time::OffsetDateTime,
    /// The point in time the account was created
    pub created_at: time::OffsetDateTime,
    hashed_password: MaxStr<255>,
}

/// Representation of the login data without any permission attached to it
pub struct ClubAccount {
    /// Primary key of the account
    uuid: AccountUuid,
    /// Name to be used for displaying purposes
    pub display_name: MaxStr<255>,
    /// The username that should be used for logging in
    pub username: MaxStr<255>,
    /// Email of the account
    pub email: MaxStr<255>,
    /// The club this account is part of
    pub club: ClubUuid,
    /// The last point in time the account was modified
    pub modified_at: time::OffsetDateTime,
    /// The point in time the account was created
    pub created_at: time::OffsetDateTime,
    hashed_password: MaxStr<255>,
}

/// New-type for the account's primary key
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct AccountUuid(pub Uuid);

impl Account {
    /// Retrieve the account by its uuid
    #[instrument(name = "Account::get_by_uuid", skip(exe))]
    pub async fn get_by_uuid(
        exe: impl Executor<'_>,
        uuid: AccountUuid,
    ) -> anyhow::Result<Option<Self>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut account = None;

        if let Some(club_member) = ClubAccount::get_by_uuid(guard.get_transaction(), uuid).await? {
            account = Some(Account::ClubMember(club_member));
        } else if let Some(club_admin) =
            ClubAdminAccount::get_by_uuid(guard.get_transaction(), uuid).await?
        {
            account = Some(Account::ClubAdmin(club_admin));
        } else if let Some(admin) =
            AdministrativeAccount::get_by_uuid(guard.get_transaction(), uuid).await?
        {
            account = Some(Account::Superadmin(admin));
        }

        guard.commit().await?;

        Ok(account)
    }

    /// Retrieve the account by its username
    #[instrument(name = "Account::get_by_username", skip(exe))]
    pub async fn get_by_username(
        exe: impl Executor<'_>,
        username: &MaxStr<255>,
    ) -> anyhow::Result<Option<Self>> {
        let mut guard = exe.ensure_transaction().await?;

        let mut account = None;

        if let Some(club_member) =
            ClubAccount::get_by_username(guard.get_transaction(), username).await?
        {
            account = Some(Account::ClubMember(club_member));
        } else if let Some(club_admin) =
            ClubAdminAccount::get_by_username(guard.get_transaction(), username).await?
        {
            account = Some(Account::ClubAdmin(club_admin));
        } else if let Some(admin) =
            AdministrativeAccount::get_by_username(guard.get_transaction(), username).await?
        {
            account = Some(Account::Superadmin(admin));
        }

        guard.commit().await?;

        Ok(account)
    }

    /// Hash a password
    #[instrument(name = "Account::hash_password", skip_all)]
    pub fn hash_password(password: &MaxStr<72>) -> anyhow::Result<String> {
        Ok(bcrypt::hash(password.as_bytes(), bcrypt::DEFAULT_COST)?)
    }

    /// Update the display name of an account
    #[instrument(name = "Account::update_display_name", skip(self, exe))]
    pub async fn update_display_name(
        &mut self,
        exe: impl Executor<'_>,
        display_name: MaxStr<255>,
    ) -> anyhow::Result<()> {
        match self {
            Account::ClubMember(club_member) => {
                rorm::update(exe, ClubAccountModel)
                    .set(ClubAccountModel.display_name, display_name)
                    .condition(ClubAccountModel.uuid.equals(club_member.uuid.0))
                    .await?;
            }
            Account::ClubAdmin(club_admin) => {
                rorm::update(exe, ClubAdminAccountModel)
                    .set(ClubAdminAccountModel.display_name, display_name)
                    .condition(ClubAdminAccountModel.uuid.equals(club_admin.uuid.0))
                    .await?;
            }
            Account::Superadmin(super_admin) => {
                rorm::update(exe, AdministrativeAccountModel)
                    .set(AdministrativeAccountModel.display_name, display_name)
                    .condition(AdministrativeAccountModel.uuid.equals(super_admin.uuid.0))
                    .await?;
            }
        }

        Ok(())
    }

    /// Check a password
    #[instrument(name = "Account::check_password", skip_all)]
    pub fn check_password(&self, password: &MaxStr<72>) -> anyhow::Result<bool> {
        let hashed_password = match self {
            Account::ClubMember(club_member) => &*club_member.hashed_password,
            Account::ClubAdmin(club_admin) => &*club_admin.hashed_password,
            Account::Superadmin(superadmin) => &*superadmin.hashed_password,
        };

        Ok(bcrypt::verify(&**password, hashed_password)?)
    }

    /// Set a new password for an account
    #[instrument(name = "Account::check_password", skip_all)]
    pub async fn set_password(
        &mut self,
        exe: impl Executor<'_>,
        password: &MaxStr<72>,
    ) -> anyhow::Result<()> {
        let hashed = MaxStr::new(Account::hash_password(password)?)?;

        match self {
            Account::ClubMember(club_member) => {
                rorm::update(exe, ClubAccountModel)
                    .set(ClubAccountModel.hashed_password, hashed.clone())
                    .condition(ClubAccountModel.uuid.equals(club_member.uuid.0))
                    .await?;

                club_member.hashed_password = hashed;
            }
            Account::ClubAdmin(club_admin) => {
                rorm::update(exe, ClubAdminAccountModel)
                    .set(ClubAdminAccountModel.hashed_password, hashed.clone())
                    .condition(ClubAdminAccountModel.uuid.equals(club_admin.uuid.0))
                    .await?;

                club_admin.hashed_password = hashed;
            }
            Account::Superadmin(superadmin) => {
                rorm::update(exe, AdministrativeAccountModel)
                    .set(AdministrativeAccountModel.hashed_password, hashed.clone())
                    .condition(AdministrativeAccountModel.uuid.equals(superadmin.uuid.0))
                    .await?;

                superadmin.hashed_password = hashed;
            }
        }

        Ok(())
    }

    /// Set a new display name for an account
    #[instrument(name = "Account::set_display_name", skip(self, exe))]
    pub async fn set_display_name(
        &mut self,
        exe: impl Executor<'_>,
        display_name: MaxStr<255>,
    ) -> anyhow::Result<()> {
        match self {
            Account::ClubMember(club_member) => {
                rorm::update(exe, ClubAccountModel)
                    .set(ClubAccountModel.display_name, display_name.clone())
                    .condition(ClubAccountModel.uuid.equals(club_member.uuid.0))
                    .await?;

                club_member.display_name = display_name;
            }
            Account::ClubAdmin(club_admin) => {
                rorm::update(exe, ClubAdminAccountModel)
                    .set(ClubAdminAccountModel.display_name, display_name.clone())
                    .condition(ClubAdminAccountModel.uuid.equals(club_admin.uuid.0))
                    .await?;

                club_admin.display_name = display_name;
            }
            Account::Superadmin(superadmin) => {
                rorm::update(exe, AdministrativeAccountModel)
                    .set(
                        AdministrativeAccountModel.display_name,
                        display_name.clone(),
                    )
                    .condition(AdministrativeAccountModel.uuid.equals(superadmin.uuid.0))
                    .await?;

                superadmin.display_name = display_name;
            }
        }

        Ok(())
    }
}
