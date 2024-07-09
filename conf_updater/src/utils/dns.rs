use std::net::IpAddr;

use dns_lookup::lookup_host;
use itertools::Itertools;
use regex::Regex;

use conf_updater_common::{ApiFailure, DomainFailureType, FailedDomain};

use crate::config::MiscConfig;

/// Regular expression for check of domain names
///
/// As per RFC 2181, there are no restrictions on the content of entries in the DNS,
/// except for the length. However, reasonable domain names may only correspond
/// to the following pattern. Note that this pattern is not complete, as it will allow
/// more domains due to not having a restriction on the length of any but the
/// last label. This regular expression will allow lowercase domain names only.
///
/// Explanation:
/// The first group matches any number of (optional) subdomains with trailing dot.
/// The second group captures the top-level domain with the restriction of length of
/// up to 63 characters. See the tests at the end of the file for checked domains.
static DOMAIN_NAME_REGEX: &str = r"^([a-z0-9]+(-[a-z0-9-]+)*\.)+([a-z]{2}([a-z0-9-]{0,60}[a-z])?)$";

/// Check a name for domain name usage, see [DOMAIN_NAME_REGEX]
pub(crate) fn test_domain_name(name: &str) -> bool {
    Regex::new(DOMAIN_NAME_REGEX).unwrap().is_match(name)
}

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

#[cfg(test)]
mod test {
    use crate::utils::dns::test_domain_name;

    #[test]
    fn check_domain_names() {
        let valid_domains = vec![
            "example.com",
            "test.sub.example.com",
            "foo.bar.baz.qux",
            "valid.domain",
            "invalid.invalid",
            "xn--example.com",
            "example.co.uk",
            "i.example.com",
            "a.b.c.example.com",
            "example.xn--otu796d",
            "xn--ngbrx.com",
            "g.co",
            "3.14159265358979323846264338327950288419716939937510582097494459.net",
            "xn--otu796d.xn--otu796d.xn--otu796d.xn--otu796d.xn--otu796d.xn--otu796d",
        ];
        for valid_domain in valid_domains {
            assert!(test_domain_name(valid_domain))
        }
    }
}
