use axum::extract::State;
use axum::Json;
use tracing::instrument;

use conf_updater_common::{ApiFailure, ProvisioningRequest};

use crate::server::AppState;
use crate::util::{
    ensure_existing_user, ensure_existing_website, ensure_unique_resolvable_domains,
    ensure_website_domains,
};

#[instrument(skip(state))]
pub(crate) async fn setup(
    State(state): State<AppState>,
    payload: Option<Json<ProvisioningRequest>>,
) -> Result<(), ApiFailure> {
    let Some(payload) = payload else {
        return Err(ApiFailure::BadRequest("missing payload".to_string()));
    };

    // Check the domain names before starting any further work
    ensure_unique_resolvable_domains(
        &payload.domains,
        &payload.forwarded_domains,
        &state.config.misc,
    )?;

    let mut tx = state.db.start_transaction().await?;
    let website_owner = ensure_existing_user(&payload.user, &mut tx).await?;
    ensure_website_domains(&payload.domains, &mut tx).await?;
    let website = ensure_existing_website(
        payload.website,
        payload.test_certificate.unwrap_or(false),
        website_owner.uuid,
        &mut tx,
    )
    .await?;
    tx.commit().await?;

    Ok(())
}
