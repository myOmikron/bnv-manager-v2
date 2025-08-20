//! Invite related code lives in this module

use std::collections::HashMap;

use anyhow::Context;
use futures_util::TryStreamExt;
use rorm::db::Executor;
use rorm::fields::types::MaxStr;
use rorm::prelude::ForeignModelByField;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::models::account::Account;
use crate::models::account::db::AccountModel;
use crate::models::account::db::AccountModelInsert;
use crate::models::club::ClubUuid;
use crate::models::invite::db::InviteModel;
use crate::models::invite::db::InviteModelInsert;
use crate::models::invite::db::InvitedClubAdminModel;
use crate::models::invite::db::InvitedClubMemberModel;
use crate::models::invite::db::InvitedSuperAdminModel;
use crate::models::role::Role;
use crate::models::role::db::ClubAdminModel;
use crate::models::role::db::ClubMemberModel;
use crate::models::role::db::SuperAdminModel;

pub(in crate::models) mod db;

/// An invitation to the platform.
///
/// When an invitation is created, the username used in it is reserved and may not be used
/// to issue another again.
#[derive(Debug, Clone)]
pub struct Invite {
    /// Primary key of the invite
    pub uuid: InviteUuid,
    /// Reserved username
    pub username: MaxStr<255>,
    /// Display-name of the user
    pub display_name: MaxStr<255>,
    /// A list of roles the user possesses
    pub roles: Vec<Role>,
    /// The point in time the invite expires
    expires_at: time::OffsetDateTime,
    /// The point in time the invite was created
    pub created_at: time::OffsetDateTime,
}

/// Wrapper for the primary key of the [Invite]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct InviteUuid(pub Uuid);

impl Invite {
    /// Find an invitation by its uuid
    #[instrument(skip(exe))]
    pub async fn find_by_uuid(
        exe: impl Executor<'_>,
        InviteUuid(invite_uuid): InviteUuid,
    ) -> anyhow::Result<Option<Invite>> {
        let mut guard = exe.ensure_transaction().await?;

        let Some(invite) = rorm::query(guard.get_transaction(), InviteModel)
            .condition(InviteModel.uuid.equals(invite_uuid))
            .optional()
            .await?
        else {
            return Ok(None);
        };

        let mut roles = vec![];

        // Superadmin
        let super_admin = rorm::query(guard.get_transaction(), InvitedSuperAdminModel)
            .condition(InvitedSuperAdminModel.invite.equals(invite.uuid))
            .optional()
            .await?;

        if super_admin.is_some() {
            roles.push(Role::SuperAdmin);
        }

        // ClubAdmin
        let club_admins = rorm::query(guard.get_transaction(), InvitedClubAdminModel)
            .condition(InvitedClubAdminModel.invite.equals(invite.uuid))
            .stream()
            .map_ok(|x| Role::ClubAdmin {
                club: ClubUuid(x.club.0),
            })
            .try_collect::<Vec<_>>()
            .await?;
        roles.extend(club_admins);

        // ClubMember
        let club_members = rorm::query(guard.get_transaction(), InvitedClubMemberModel)
            .condition(InvitedClubMemberModel.invite.equals(invite.uuid))
            .stream()
            .map_ok(|x| Role::ClubMember {
                club: ClubUuid(x.club.0),
            })
            .try_collect::<Vec<_>>()
            .await?;
        roles.extend(club_members);

        guard.commit().await?;

        Ok(Some(Invite {
            uuid: InviteUuid(invite.uuid),
            username: invite.username,
            display_name: invite.display_name,
            roles,
            expires_at: invite.expires_at,
            created_at: invite.created_at,
        }))
    }

    /// Find all invites for a given club
    ///
    /// This includes members and admins.
    /// Superadmins aren't selected as they are not associated with a club
    #[instrument(name = "Invite::find_by_club", skip(exe))]
    pub async fn find_by_club(
        exe: impl Executor<'_>,
        ClubUuid(club_uuid): ClubUuid,
    ) -> anyhow::Result<Vec<Self>> {
        let mut guard = exe.ensure_transaction().await?;

        // Query invited members
        let mut invites = rorm::query(
            guard.get_transaction(),
            InvitedClubMemberModel.invite.query_as(InviteModel),
        )
        .condition(InvitedClubMemberModel.club.equals(club_uuid))
        .stream()
        .map_ok(|x| {
            (
                x.uuid,
                Invite {
                    uuid: InviteUuid(x.uuid),
                    username: x.username,
                    display_name: x.display_name,
                    roles: vec![Role::ClubMember {
                        club: ClubUuid(club_uuid),
                    }],
                    expires_at: x.expires_at,
                    created_at: x.created_at,
                },
            )
        })
        .try_collect::<HashMap<_, _>>()
        .await?;

        // Query invited admins
        rorm::query(
            guard.get_transaction(),
            InvitedClubAdminModel.invite.query_as(InviteModel),
        )
        .condition(InvitedClubAdminModel.club.equals(club_uuid))
        .all()
        .await?
        .into_iter()
        .for_each(|x| {
            invites
                .entry(x.uuid)
                .and_modify(|existing| {
                    existing.roles.push(Role::ClubAdmin {
                        club: ClubUuid(club_uuid),
                    })
                })
                .or_insert(Invite {
                    uuid: InviteUuid(x.uuid),
                    username: x.username,
                    display_name: x.display_name,
                    roles: vec![Role::ClubAdmin {
                        club: ClubUuid(club_uuid),
                    }],
                    expires_at: x.expires_at,
                    created_at: x.created_at,
                });
        });

        guard.commit().await?;

        Ok(invites.into_values().collect())
    }

    /// Get the point in time the invite expires
    pub fn expires_at(&self) -> time::OffsetDateTime {
        self.expires_at
    }

    /// Migrate an [Invite] instance to an actual account.
    #[instrument(skip(exe))]
    pub async fn accept_invite(
        self,
        exe: impl Executor<'_>,
        AcceptInviteParams { password }: AcceptInviteParams,
    ) -> anyhow::Result<Result<Account, AcceptInviteError>> {
        if self.expires_at < time::OffsetDateTime::now_utc() {
            return Ok(Err(AcceptInviteError::Expired));
        }

        let mut guard = exe.ensure_transaction().await?;

        #[allow(clippy::expect_used)]
        let hashed_password =
            MaxStr::new(Account::hash_password(&password).context("Hashing password failed")?)
                .expect("Resulting hash must be <255 bytes");

        let account = rorm::insert(guard.get_transaction(), AccountModel)
            .single(&AccountModelInsert {
                uuid: Uuid::new_v4(),
                username: self.username,
                display_name: self.display_name,
                hashed_password,
            })
            .await?;

        // Superadmin role
        let super_admin = rorm::query(guard.get_transaction(), InvitedSuperAdminModel)
            .condition(InvitedSuperAdminModel.invite.equals(self.uuid.0))
            .optional()
            .await?;
        if super_admin.is_some() {
            rorm::insert(guard.get_transaction(), SuperAdminModel)
                .single(&SuperAdminModel {
                    uuid: Uuid::new_v4(),
                    account: ForeignModelByField(account.uuid),
                })
                .await?;
        }

        // Club admins
        let club_admins = rorm::query(guard.get_transaction(), InvitedClubAdminModel)
            .condition(InvitedClubAdminModel.invite.equals(self.uuid.0))
            .stream()
            .map_ok(|x| ClubAdminModel {
                uuid: Uuid::new_v4(),
                account: ForeignModelByField(account.uuid),
                club: ForeignModelByField(x.club.0),
            })
            .try_collect::<Vec<_>>()
            .await?;
        rorm::insert(guard.get_transaction(), ClubAdminModel)
            .bulk(club_admins)
            .await?;

        // Club members
        let club_members = rorm::query(guard.get_transaction(), InvitedClubMemberModel)
            .condition(InvitedClubMemberModel.invite.equals(self.uuid.0))
            .stream()
            .map_ok(|x| ClubMemberModel {
                uuid: Uuid::new_v4(),
                account: ForeignModelByField(account.uuid),
                club: ForeignModelByField(x.club.0),
            })
            .try_collect::<Vec<_>>()
            .await?;
        rorm::insert(guard.get_transaction(), ClubMemberModel)
            .bulk(club_members)
            .await?;

        // Delete invite, the related invited roles will be deleted by cascade
        rorm::delete(guard.get_transaction(), InviteModel)
            .condition(InviteModel.uuid.equals(self.uuid.0))
            .await?;

        guard.commit().await?;

        Ok(Ok(Account::from(account)))
    }

    /// Create a new invite.
    ///
    /// Checks also if the chosen username is still available.
    #[instrument(skip(exe))]
    pub async fn create(
        exe: impl Executor<'_>,
        CreateInviteParams {
            username,
            display_name,
            roles,
            expires_at,
        }: CreateInviteParams,
    ) -> anyhow::Result<Result<Invite, CreateInviteError>> {
        let mut guard = exe.ensure_transaction().await?;
        let username = MaxStr::new(username.to_lowercase())?;

        let account = Account::find_by_username(guard.get_transaction(), &username).await?;
        if account.is_some() {
            return Ok(Err(CreateInviteError::UsernameTaken));
        }

        let invite = rorm::query(guard.get_transaction(), InviteModel)
            .condition(InviteModel.username.equals(&*username))
            .optional()
            .await?;

        if invite.is_some() {
            return Ok(Err(CreateInviteError::UsernameTaken));
        }

        let invite = rorm::insert(guard.get_transaction(), InviteModel)
            .single(&InviteModelInsert {
                uuid: Uuid::new_v4(),
                username,
                display_name,
                expires_at,
            })
            .await?;

        for role in &roles {
            match role {
                Role::SuperAdmin => {
                    rorm::insert(guard.get_transaction(), InvitedSuperAdminModel)
                        .single(&InvitedSuperAdminModel {
                            uuid: Uuid::new_v4(),
                            invite: ForeignModelByField(invite.uuid),
                        })
                        .await?;
                }
                Role::ClubAdmin {
                    club: ClubUuid(club_uuid),
                } => {
                    rorm::insert(guard.get_transaction(), InvitedClubAdminModel)
                        .single(&InvitedClubAdminModel {
                            uuid: Uuid::new_v4(),
                            invite: ForeignModelByField(invite.uuid),
                            club: ForeignModelByField(*club_uuid),
                        })
                        .await?;
                }
                Role::ClubMember {
                    club: ClubUuid(club_uuid),
                } => {
                    rorm::insert(guard.get_transaction(), InvitedClubMemberModel)
                        .single(&InvitedClubMemberModel {
                            uuid: Uuid::new_v4(),
                            invite: ForeignModelByField(invite.uuid),
                            club: ForeignModelByField(*club_uuid),
                        })
                        .await?;
                }
            }
        }

        guard.commit().await?;

        Ok(Ok(Invite {
            uuid: InviteUuid(invite.uuid),
            username: invite.username,
            display_name: invite.display_name,
            roles,
            expires_at: invite.expires_at,
            created_at: invite.created_at,
        }))
    }

    /// Clear expired invites
    #[instrument(name = "Invite::clear_expired", skip(exe))]
    pub async fn clear_expired(exe: impl Executor<'_>) -> anyhow::Result<()> {
        rorm::delete(exe, InviteModel)
            .condition(
                InviteModel
                    .expires_at
                    .less_than(time::OffsetDateTime::now_utc()),
            )
            .await?;

        Ok(())
    }
}

/// Parameters to create a new invite
#[derive(Debug, Clone)]
pub struct CreateInviteParams {
    /// The username to use
    pub username: MaxStr<255>,
    /// Name to be displayed, should be a legal name
    pub display_name: MaxStr<255>,
    /// Roles the new account should start with
    pub roles: Vec<Role>,
    /// The point in time the invite expires
    pub expires_at: time::OffsetDateTime,
}

/// Parameters to accept an invitation
#[derive(Debug, Clone)]
pub struct AcceptInviteParams {
    /// The cleartext password of the user
    pub password: MaxStr<72>,
}

/// Errors that can be handled
#[derive(Debug, Clone, Error)]
#[allow(missing_docs)]
pub enum CreateInviteError {
    #[error("Username is already taken")]
    UsernameTaken,
}

#[derive(Debug, Clone, Error)]
#[allow(missing_docs)]
pub enum AcceptInviteError {
    #[error("Invite expired")]
    Expired,
}
