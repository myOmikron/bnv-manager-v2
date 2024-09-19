use axum::extract::State;
use tracing::{instrument, warn};

use conf_updater_common::ApiFailure;

use crate::server::AppState;

#[instrument(skip(state))]
pub(crate) async fn refresh(State(state): State<AppState>) -> Result<(), ApiFailure> {
    // TODO: Implement this endpoint
    warn!("Endpoint not implemented");
    Ok(())
}
