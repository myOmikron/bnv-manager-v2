use std::process::Command;

/// Check that the `certbot` utility is executable as found in $PATH
pub(crate) fn check_available() -> bool {
    Command::new("certbot").arg("--help").output().is_ok()
}
