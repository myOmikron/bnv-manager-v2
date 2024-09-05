use std::path::Path;
use std::time::{Duration, SystemTime};

use openssl::error::ErrorStack;
use openssl::x509;
use tracing::{instrument, warn};

/// Determine if a certificate at location `path` is still valid by checking the
/// `NotAfter` attribute relative to current system time is greater than `remaining_seconds`
#[instrument(level = "trace")]
pub(crate) fn is_valid<P: AsRef<Path> + std::fmt::Debug>(path: P, remaining_seconds: u64) -> std::io::Result<bool> {
    let content = std::fs::read(&path)?;
    let cert = match x509::X509::from_pem(&content) {
        Ok(v) => v,
        Err(err) => {
            return Err(map_err(err, "read_pem"));
        }
    };

    let unix_time = openssl::asn1::Asn1Time::from_unix(0).map_err(|e| map_err(e, "from_unix"))?;
    let delta_time = unix_time
        .diff(cert.not_after())
        .map_err(|e| map_err(e, "delta_time"))?;
    let expiry_from_unix =
        Duration::from_secs(delta_time.days as u64 * 86400 + delta_time.secs as u64);
    let unix_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("failed to get SystemTime");
    if unix_now >= expiry_from_unix {
        // Already expired certificate
        return Ok(false);
    }
    let remaining_expiry = expiry_from_unix - unix_now;
    let remaining_duration = Duration::from_secs(remaining_seconds);
    Ok(remaining_expiry > remaining_duration)
}

/// Verify that a certificate at location `path` contains at least all requested domains
#[instrument(level = "trace")]
pub(crate) fn contains_domains<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
    domains: &Vec<String>,
) -> std::io::Result<bool> {
    let content = std::fs::read(&path)?;
    let cert = match x509::X509::from_pem(&content) {
        Ok(v) => v,
        Err(err) => {
            return Err(map_err(err, "read_pem"));
        }
    };

    // Create a list of all the certificate's valid subject names and alt names
    let mut all_dns_names = vec![];
    let subject_name_entries = cert.subject_name().entries();
    for subject_name_entry in subject_name_entries {
        if let Ok(name_entry) = subject_name_entry.data().as_utf8() {
            all_dns_names.push(name_entry.to_string());
        }
    }
    if let Some(alt_name_stack) = cert.subject_alt_names() {
        all_dns_names.extend(alt_name_stack.iter().filter_map(|s| {
            if let Some(n) = s.dnsname() {
                return Some(n.to_string());
            } else {
                None
            }
        }))
    }

    let success = domains.iter().all(|domain| all_dns_names.contains(domain));
    Ok(success)
}

fn map_err(error_stack: ErrorStack, hint: &str) -> std::io::Error {
    warn!(
        "Failed to handle certificate. Hint: {}. Error: {}",
        hint, &error_stack
    );
    return std::io::Error::new(std::io::ErrorKind::InvalidData, error_stack);
}
