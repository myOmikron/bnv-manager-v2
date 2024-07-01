use std::net::IpAddr;

use dns_lookup::lookup_host;
use itertools::Itertools;
use rorm::{FieldAccess, insert, Model, query, update};
use rorm::db::Executor;
use rorm::prelude::ForeignModelByField;
use tracing::info;
use uuid::Uuid;

use conf_updater_common::{ApiFailure, DomainFailureType, FailedDomain, WebsiteUser};

use crate::config::MiscConfig;
use crate::models::{Domain, User, Website};
use crate::models::patches::{NewUser, NewWebsite};

/// Check that all domains, both the normal and forwards, are unique (i.e., no duplicates)
pub(crate) fn check_unique_domains(domains: &Vec<String>, forwarded_domains: &Vec<String>) -> bool {
    let merged_domains = domains.iter().merge(forwarded_domains);
    let n_domains = domains.len() + forwarded_domains.len();
    let unique_domains: Vec<&String> = merged_domains.unique().collect();
    unique_domains.len() == n_domains
}

/// Ensure that the user exists locally and update its CN if necessary, otherwise create it
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
        if existing_user.cn != user.cn {
            update!(guard.get_transaction(), User)
                .condition(User::F.uuid.equals(user.id))
                .set(User::F.cn, user.cn.clone())
                .await?;
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
