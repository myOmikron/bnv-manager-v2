use std::time::{Duration, SystemTime};

use openssl::error::ErrorStack;
use openssl::x509;
use tracing::warn;

/// Determine if a certificate at location `path` is still valid by checking the
/// `NotAfter` attribute relative to current system time is greater than `remaining_seconds`
pub(crate) fn is_valid(path: String, remaining_seconds: u64) -> std::io::Result<bool> {
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

fn map_err(error_stack: ErrorStack, hint: &str) -> std::io::Error {
    warn!(
        "Failed to handle certificate. Hint: {}. Error: {}",
        hint, &error_stack
    );
    return std::io::Error::new(std::io::ErrorKind::InvalidData, error_stack);
}
