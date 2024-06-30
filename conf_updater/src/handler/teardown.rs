use axum::extract::State;
use tracing::{instrument, warn};

use conf_updater_common::{ApiFailure, DomainFailureType, FailedDomain};

use crate::server::AppState;

#[instrument(skip(state))]
pub(crate) async fn teardown(State(state): State<AppState>) -> Result<(), ApiFailure> {
    // TODO: Implement this endpoint. The below error is a sample only!
    warn!("Endpoint not implemented");
    Err(ApiFailure::DomainCheckFailure(vec![
        FailedDomain {
            domain: "foo".to_string(),
            error: DomainFailureType::DoesNotResolve,
            message: "".to_string(),
        },
        FailedDomain {
            domain: "bar".to_string(),
            error: DomainFailureType::DoesNotResolve,
            message: "no message here".to_string(),
        },
    ]))
}
