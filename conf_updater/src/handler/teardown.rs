use axum::extract::State;
use axum::Json;
use tracing::{instrument, warn};

use conf_updater_common::{ApiFailure, DomainFailureType, FailedDomain, RemovalRequest};

use crate::server::AppState;

#[instrument(skip(state))]
pub(crate) async fn teardown(
    State(state): State<AppState>,
    payload: Option<Json<RemovalRequest>>,
) -> Result<(), ApiFailure> {
    let Some(payload) = payload else {
        return Err(ApiFailure::BadRequest("missing payload".to_string()));
    };

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
