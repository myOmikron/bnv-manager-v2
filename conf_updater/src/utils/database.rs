use crate::models::patches::NewUser;
use crate::models::{Domain, User};
use conf_updater_common::WebsiteUser;
use rorm::db::Executor;
use rorm::{insert, query, update, FieldAccess, Model};
use uuid::Uuid;

/// Ensure that the user exists locally and update its CN, DN and UID if necessary, otherwise create it
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

/// Make sure that all domains either belong to the same website or do not exist
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
