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

/// Ensure that all domain names resolve to this server's allowed IPs
pub(crate) fn ensure_unique_resolvable_domains(
    domains: &Vec<String>,
    forwarded_domains: &Vec<String>,
    config: &MiscConfig,
) -> Result<(), ApiFailure> {
    let merged_domains = domains.iter().merge(forwarded_domains);
    let n_domains = domains.len() + forwarded_domains.len();
    let unique_domains: Vec<&String> = merged_domains.unique().collect();
    if unique_domains.len() != n_domains {
        return Err(ApiFailure::BadRequest(
            "duplicate domain names (not unique)".to_string(),
        ));
    }

    let mut failed_domains = vec![];
    for domain in unique_domains {
        // The DNS lookup uses the host's configuration, so take a look at /etc/resolv.conf
        match lookup_host(domain) {
            Ok(ip_addrs) => {
                let mut failures = 0;
                for ip_addr in ip_addrs {
                    match ip_addr {
                        IpAddr::V4(ip) => {
                            if !config.global_ipv4.contains(&ip) {
                                failures += 1;
                            }
                        }
                        IpAddr::V6(ip) => {
                            if !config.global_ipv6.contains(&ip) {
                                failures += 1;
                            }
                        }
                    }
                }
                if failures > 0 {
                    failed_domains.push(FailedDomain {
                        domain: domain.clone(),
                        error: DomainFailureType::WrongResolve,
                        message: format!("domain {} resolves to the wrong address", domain),
                    })
                }
            }
            Err(e) => {
                failed_domains.push(FailedDomain {
                    domain: domain.clone(),
                    error: DomainFailureType::DoesNotResolve,
                    message: e.to_string(),
                });
            }
        };
    }
    if failed_domains.is_empty() {
        Ok(())
    } else {
        Err(ApiFailure::DomainCheckFailure(failed_domains))
    }
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
