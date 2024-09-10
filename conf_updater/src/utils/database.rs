use rorm::{and, delete, FieldAccess, insert, Model, query, update};
use rorm::db::Executor;
use rorm::prelude::ForeignModelByField;
use tracing::instrument;
use uuid::Uuid;

use conf_updater_common::WebsiteUser;

use crate::models::{Domain, User, Website};
use crate::models::patches::{NewDomain, NewUser, NewWebsite};

/// Ensure that the user exists locally and update its CN, DN and UID if necessary, otherwise create it
#[instrument(level = "trace", skip(exe))]
pub(crate) async fn ensure_existing_user(
    user: &WebsiteUser,
    exe: impl Executor<'_>,
) -> Result<User, rorm::Error> {
    let mut guard = exe.ensure_transaction().await?;
    return if let Some(existing_user) = query!(guard.get_transaction(), User)
        .condition(User::F.uuid.equals(user.id))
        .optional()
        .await?
    {
        // TODO: verify this updating functionality, especially with the check in line 56!
        // TODO: changing these user attributes needs more backend work (e.g. moving directories, changing file ownerships, ...) as well
        if existing_user.cn != user.cn {
            update!(guard.get_transaction(), User)
                .condition(User::F.uuid.equals(user.id))
                .set(User::F.cn, user.cn.clone())
                .await?;
        }
        if existing_user.dn != user.dn {
            update!(guard.get_transaction(), User)
                .condition(User::F.uuid.equals(user.id))
                .set(User::F.dn, user.dn.clone())
                .await?;
        }
        if existing_user.posix_uid != user.posix_uid as i64 {
            update!(guard.get_transaction(), User)
                .condition(User::F.uuid.equals(user.id))
                .set(User::F.posix_uid, user.posix_uid as i64)
                .await?;
        }
        if existing_user.cn != user.cn || existing_user.dn != user.dn {
            let updated_user = query!(guard.get_transaction(), User)
                .condition(User::F.uuid.equals(user.id))
                .optional()
                .await?
                .unwrap();
            guard.commit().await?;
            Ok(updated_user)
        } else {
            Ok(existing_user)
        }
    } else {
        let user = insert!(guard.get_transaction(), NewUser)
            .single(&NewUser {
                uuid: user.id,
                cn: user.cn.clone(),
                dn: user.dn.clone(),
                posix_uid: user.posix_uid as i64,
            })
            .await?;
        guard.commit().await?;
        Ok(user)
    };
}

/// Find the user that owns a particular website by its UUID
#[instrument(level = "trace", skip(exe))]
pub(crate) async fn find_website_owner(
    website: &Uuid,
    exe: impl Executor<'_>,
) -> Result<User, rorm::Error> {
    let mut guard = exe.ensure_transaction().await?;
    let owner = query!(guard.get_transaction(), User).condition(Website::F.uuid.equals(website.clone())).one().await?;
    guard.commit().await?;
    Ok(owner)
}

/// Drop a website and all its belonging domains (no errors if the website doesn't exist)
#[instrument(level = "trace", skip(exe))]
pub(crate) async fn drop_website(
    website: &Uuid,
    exe: impl Executor<'_>,
) -> Result<(), rorm::Error> {
    let mut guard = exe.ensure_transaction().await?;
    delete!(guard.get_transaction(), Domain).condition(Domain::F.website.equals(website.clone())).await?;
    delete!(guard.get_transaction(), Website).condition(Website::F.uuid.equals(website.clone())).await?;
    guard.commit().await?;
    Ok(())
}

/// Make sure that all domains either belong to the same website or do not exist
#[instrument(level = "trace", skip(exe))]
pub(crate) async fn ensure_website_domains(
    domains: &Vec<String>,
    exe: impl Executor<'_>,
) -> Result<bool, rorm::Error> {
    let mut guard = exe.ensure_transaction().await?;
    let mut website_uuid: Uuid = Uuid::from_u128_le(0);
    for domain_name in domains {
        // Since domain names must be unique, this will return at most one record
        if let Some(existing_domain) = query!(guard.get_transaction(), Domain)
            .condition(Domain::F.domain.equals(domain_name))
            .optional()
            .await?
        {
            let uuid: &Uuid = existing_domain.website.key();
            if website_uuid.as_u128() == 0 {
                website_uuid = uuid.clone();
            } else if website_uuid != *uuid {
                return Ok(false);
            }
        };
    }
    Ok(true)
}

/// Make sure that a website with the given UUID exists or create it if it doesn't
#[instrument(level = "trace", skip(exe))]
pub(crate) async fn ensure_existing_website(
    website: &Uuid,
    test_certificate: bool,
    website_owner: &User,
    exe: impl Executor<'_>,
) -> Result<Website, rorm::Error> {
    let mut guard = exe.ensure_transaction().await?;
    if let Some(existing_website) = query!(guard.get_transaction(), Website)
        .condition(Website::F.uuid.equals(website.clone()))
        .optional()
        .await?
    {
        // The website owner is not checked for an existing website. We do not support changing it. Create a new website instead.
        if test_certificate != existing_website.test_cert {
            update!(guard.get_transaction(), Website)
                .condition(Website::F.uuid.equals(website.clone()))
                .set(Website::F.test_cert, test_certificate)
                .await?;
            let existing_website = query!(guard.get_transaction(), Website)
                .condition(Website::F.uuid.equals(website.clone()))
                .one()
                .await?;
            guard.commit().await?;
            Ok(existing_website)
        } else {
            Ok(existing_website)
        }
    } else {
        let website = insert!(guard.get_transaction(), NewWebsite)
            .single(&NewWebsite {
                uuid: website.clone(),
                owner: ForeignModelByField::Key(website_owner.uuid),
                test_cert: test_certificate,
            })
            .await?;
        guard.commit().await?;
        Ok(website.into())
    }
}

/// Update the domains for a website
///
/// Here, partial means: you should call this once for
/// hosted domains and once for forwarded domains separately.
/// The returned Ok value shows if something was changed.
#[instrument(level = "trace", skip(exe))]
pub(crate) async fn set_partial_domains(
    domains: &Vec<String>,
    website: &Website,
    forwarded: bool,
    exe: impl Executor<'_>,
) -> Result<bool, rorm::Error> {
    let mut changed = false;
    let mut guard = exe.ensure_transaction().await?;

    let existing_domains = query!(guard.get_transaction(), Domain).condition(and!(
        Domain::F.forwarded.equals(forwarded),
        Domain::F.website.equals(&website.uuid)
    )).all().await?;
    for existing_domain in existing_domains {
        if !domains.contains(&existing_domain.domain) {
            changed = true;
            delete!(guard.get_transaction(), Domain).single(&existing_domain).await?;
        }
    }

    let remaining_existing_domains: Vec<String> = query!(guard.get_transaction(), Domain).condition(and!(
        Domain::F.forwarded.equals(forwarded),
        Domain::F.website.equals(&website.uuid)
    )).all().await?.iter().map(|d| d.domain.clone()).collect();
    for domain in domains {
        if !remaining_existing_domains.contains(&domain) {
            changed = true;
            insert!(guard.get_transaction(), Domain).single(&NewDomain {
                domain: domain.clone(),
                forwarded,
                website: ForeignModelByField::Key(website.uuid),
            }).await?;
        }
    }

    Ok(changed)
}
