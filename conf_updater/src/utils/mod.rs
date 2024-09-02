use std::io;

use thiserror::Error;
use tracing::{debug, error};

use conf_updater_common::ApiFailure;

pub(crate) mod certbot;
pub(crate) mod certificates;
pub(crate) mod database;
pub(crate) mod dns;
pub(crate) mod nginx;
pub(crate) mod web_space;

/// Errors while calling external programs for use as Err type for Result
#[derive(Debug, Error)]
pub enum ProgramError {
    /// Any kind of IO error
    #[error("{0}")]
    Io(#[from] io::Error),
    /// The program did not complete successfully (not exit code 0),
    /// then the first string is the program name or identifier or collection
    /// of arguments, while the second string is the program's full `stderr`
    #[error("Failed to execute program")]
    Failure(String, String),
    /// The program did not complete successfully (not exit code 0),
    /// but reading its `stderr` into a UTF-8 string also did not work
    #[error("Failed to read program stderr into UTF-8 string")]
    Utf8Error(#[from] core::str::Utf8Error),
}

/// For simplicity, [ProgramError] can directly be converted into [ApiFailure];
/// this provides a default implementation for all enums that does not leak meaningful
/// information to the calling party (always plain `InternalServerError`) but adds logging.
impl From<ProgramError> for ApiFailure {
    fn from(value: ProgramError) -> ApiFailure {
        match value {
            ProgramError::Io(err) => {
                error!("IOError: {}", err);
                ApiFailure::InternalServerError
            }
            ProgramError::Failure(cmd, err) => {
                error!("Executing '{}' did not complete successfully", cmd);
                debug!("Returned output from previous program call: {}", err);
                ApiFailure::InternalServerError
            }
            ProgramError::Utf8Error(_) => ApiFailure::InternalServerError,
        }
    }
}

pub(crate) fn try_from_utf8(program: String, stderr: &[u8]) -> Result<String, ProgramError> {
    Ok(std::str::from_utf8(stderr).map_err(|e| {
        error!("Executing '{}' did not complete successfully and did not produce valid UTF8 output after byte {}", program, e.valid_up_to());
        ProgramError::Utf8Error(e)
    })?.to_string())
}
