use std::fmt::Debug;
use std::io;
use std::process::Command;

use tracing::info;
use uuid::Uuid;

/// Check that the `certbot` utility is executable as found in $PATH
pub(crate) fn check_available() -> bool {
    Command::new("certbot").arg("--help").output().is_ok()
}

/// Check that the `certbot` has been configured with an account, returning error output on failure
pub(crate) fn check_account() -> Result<(), String> {
    let output = match Command::new("certbot").arg("show_account").output() {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    if output.status.success() {
        Ok(())
    } else {
        match std::str::from_utf8(&*output.stderr) {
            Ok(v) => Err(v.into()),
            Err(_) => {
                return Err(
                    "unexpected error parsing UTF-8 sequence from certbot show_account call"
                        .to_string(),
                )
            }
        }
    }
}

/// Obtain Let's Encrypt TLS certificates through HTTP challenge with certbot's nginx plugin
///
/// **WARNING!**
/// The values in the `domains` vector and the `cert_name` are passed as arguments to the
/// `certbot` utility. Make sure to properly sanitize these values before calling this
/// function. However, the arguments are not passed through a shell, but given
/// literally to `certbot`. This means that shell syntax like quotes, escaped
/// characters, word splitting, glob patterns, variable substitution, etc. have no effect.
pub(crate) fn obtain_certificates(
    cert_name: Uuid,
    test_certificate: bool,
    domains: Vec<String>,
) -> io::Result<Result<(), String>> {
    if domains.len() == 0 {
        return Ok(Err("empty domain list".to_string()));
    }
    let mut command = Command::new("certbot");
    command
        .arg("certonly")
        .arg("-n") // non-interactive
        .arg("--nginx")
        .arg("--cert-name")
        .arg(cert_name.as_hyphenated().to_string());
    if test_certificate {
        command.arg("--test-cert");
    }
    for domain in &domains {
        command.arg("-d"); // the next argument following after it is a domain name
        command.arg(domain);
    }
    info!(
        "Executing {:#?} with {} domains for {} {}...",
        command.get_program(),
        domains.len(),
        cert_name.as_hyphenated().to_string(),
        if test_certificate { "(test cert) " } else { "" }
    );
    let output = command.output()?;
    if output.status.success() {
        Ok(Ok(()))
    } else {
        let stderr = std::str::from_utf8(&*output.stderr).map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "unexpected error parsing UTF-8 sequence from certbot obtain call",
            )
        })?;
        Ok(Err(stderr.into()))
    }
}
