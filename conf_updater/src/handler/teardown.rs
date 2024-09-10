use axum::extract::State;
use axum::Json;
use tracing::instrument;

use conf_updater_common::{ApiFailure, RemovalRequest};

use crate::server::AppState;
use crate::utils::database::{drop_website, find_website_owner};
use crate::utils::nginx::{drop_config, reload_server, verify_config};
use crate::utils::web_space::delete_webspace;

#[instrument(skip(state))]
pub(crate) async fn teardown(
    State(state): State<AppState>,
    payload: Option<Json<RemovalRequest>>,
) -> Result<(), ApiFailure> {
    let Some(payload) = payload else {
        return Err(ApiFailure::BadRequest("missing payload".to_string()));
    };

    // Remove the website from the database
    let mut tx = state.db.start_transaction().await?;
    let website_owner = find_website_owner(&payload.website, &mut tx).await?.uuid;
    drop_website(&payload.website, &mut tx).await?;
    tx.commit().await?;

    // Remove the config & check the nginx configuration & reload the server
    drop_config(&payload.website, &state.config.misc)?;
    verify_config()?;
    reload_server()?;

    // Delete the webspace and all its belonging files
    delete_webspace(&payload.website, &website_owner, &state.config)?;

    Ok(())
}
