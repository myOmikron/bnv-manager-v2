use std::io;
use std::path::PathBuf;
use std::process::Command;

use thiserror::Error;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use conf_updater_common::{ApiFailure, CertbotFailureDetails};

use crate::config::CertbotConfig;
use crate::utils::certificates::{contains_domains, is_valid};

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
pub(crate) fn obtain_certificate(
    cert_name: &Uuid,
    test_certificate: bool,
    domains: &Vec<String>,
) -> Result<(), CertbotError> {
    if domains.len() == 0 {
        return Err(CertbotError::EmptyDomainList(*cert_name));
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
    for domain in domains {
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
        Ok(())
    } else {
        let stderr = std::str::from_utf8(&*output.stderr)
            .map_err(|_| CertbotError::ExecutionError(*cert_name))?;
        Err(CertbotError::ErrorMessage(*cert_name, stderr.into()))
    }
}

pub(crate) fn verify_cert(
    website: &Uuid,
    all_domains: &Vec<String>,
    conf: &CertbotConfig,
) -> Result<(), ApiFailure> {
    let cert_base_dir = PathBuf::from(&conf.cert_dir).join(website.as_hyphenated().to_string());
    let cert_file = cert_base_dir.join("fullchain.pem");
    if !contains_domains(&cert_file, all_domains).map_err(|e| {
        debug!(
            "IOError trying to read {} for {}: {}",
            cert_file.display(),
            website.as_hyphenated(),
            e
        );
        ApiFailure::InternalServerError
    })? {
        error!(
            "Newly obtained cert for {} does not contain all {} expected domains: {:#?}",
            website,
            all_domains.len(),
            all_domains
        );
        return Err(ApiFailure::InternalServerError);
    }
    if !is_valid(&cert_file, 75 * 24 * 3600).map_err(|e| {
        debug!(
            "IOError trying to read {} for {}: {}",
            cert_file.display(),
            website.as_hyphenated(),
            e
        );
        ApiFailure::InternalServerError
    })? {
        error!(
            "Newly obtained cert for {} is not valid for at least 75 days",
            website,
        );
        return Err(ApiFailure::InternalServerError);
    }
    Ok(())
}

#[derive(Debug, Error)]
pub(crate) enum CertbotError {
    #[error("empty domain list")]
    EmptyDomainList(Uuid),
    #[error("{0}")]
    IOError(#[from] io::Error),
    #[error("Failed to execute")]
    ExecutionError(Uuid),
    #[error("Certbot operation failed: {0}: {1}")]
    ErrorMessage(Uuid, String),
}

impl From<CertbotError> for ApiFailure {
    fn from(value: CertbotError) -> Self {
        match value {
            CertbotError::EmptyDomainList(uuid) => ApiFailure::BadRequest(format!(
                "{} requires at least one domain",
                uuid.as_hyphenated()
            )),
            CertbotError::IOError(v) => {
                warn!("IOError: {}", v);
                ApiFailure::InternalServerError
            }
            CertbotError::ExecutionError(_) => ApiFailure::InternalServerError,
            CertbotError::ErrorMessage(uuid, error_message) => {
                ApiFailure::CertbotCertError(CertbotFailureDetails {
                    website: uuid,
                    failed_domains: vec![], // TODO: parse the failed domains from the error message
                    full_error: error_message,
                })
            }
        }
    }
}
