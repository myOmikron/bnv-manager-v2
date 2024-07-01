use std::net::IpAddr;

use dns_lookup::lookup_host;
use itertools::Itertools;

use conf_updater_common::{ApiFailure, DomainFailureType, FailedDomain};

use crate::config::MiscConfig;

/// Ensure that all domain names resolve to this server's allowed IPs
pub(crate) fn ensure_resolvable_domains(
    domains: &Vec<String>,
    config: &MiscConfig,
) -> Result<(), ApiFailure> {
    let mut failed_domains = vec![];
    for domain in domains {
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
