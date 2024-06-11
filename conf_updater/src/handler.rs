use axum::extract::State;

use conf_updater_common::ApiFailure;

use crate::server::AppState;

pub(crate) async fn setup(State(state): State<AppState>) -> Result<(), ApiFailure> {
    Ok(())
}

pub(crate) async fn teardown(State(state): State<AppState>) -> Result<(), ApiFailure> {
    Ok(())
}
