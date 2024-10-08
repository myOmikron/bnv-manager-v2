use axum::extract::State;
use axum::Json;
use itertools::Itertools;
use tracing::instrument;

use conf_updater_common::{ApiFailure, ProvisioningRequest};

use crate::server::AppState;
use crate::utils::certbot::{obtain_certificate, verify_cert};
use crate::utils::database::{ensure_existing_user, ensure_existing_website, ensure_website_domains, set_partial_domains};
use crate::utils::dns::{ensure_resolvable_domains, validate_domain_names};
use crate::utils::nginx::{reload_server, verify_config, write_nginx_conf};
use crate::utils::web_space::create_webspace;

#[instrument(skip(state))]
pub(crate) async fn setup(
    State(state): State<AppState>,
    payload: Option<Json<ProvisioningRequest>>,
) -> Result<(), ApiFailure> {
    let Some(payload) = payload else {
        return Err(ApiFailure::BadRequest(
            "missing payload or fields".to_string(),
        ));
    };

    let user_id = payload.user.posix_uid;
    if user_id < 1000 {
        return Err(ApiFailure::BadRequest("invalid posix user ID".to_string()));
    }

    // Create a vector of all domains for further DNS & certificate operations
    let mut all_domains = payload.domains.clone();
    all_domains.extend(payload.forwarded_domains.clone());
    if !all_domains.iter().all_unique() {
        return Err(ApiFailure::BadRequest(
            "duplicate domain names (not unique)".to_string(),
        ));
    }

    // Check the domain names before starting any further work
    let invalid_domains = validate_domain_names(&all_domains);
    if !invalid_domains.is_empty() {
        return Err(ApiFailure::DomainCheckFailure(invalid_domains));
    }
    ensure_resolvable_domains(&all_domains, &state.config.misc)?;

    let mut tx = state.db.start_transaction().await?;
    if !ensure_website_domains(&payload.domains, &mut tx).await? {
        return Err(ApiFailure::BadRequest("at least one domain was already registered elsewhere".to_string()));
    };
    let website_owner = ensure_existing_user(&payload.user, &mut tx).await?;
    tx.commit().await?;

    let use_test_cert = state.config.certbot.test_certs || payload.test_certificate.unwrap_or(false);
    obtain_certificate(&payload.website, use_test_cert, &all_domains)?;
    verify_cert(&payload.website, &all_domains, &state.config.certbot)?;

    let mut tx = state.db.start_transaction().await?;
    let website = ensure_existing_website(
        &payload.website,
        payload.test_certificate.unwrap_or(false),
        &website_owner,
        &mut tx,
    ).await?;
    set_partial_domains(&payload.domains, &website, false, &mut tx).await?;
    set_partial_domains(&payload.forwarded_domains, &website, true, &mut tx).await?;
    tx.commit().await?;

    // Create the webspace if it doesn't exist and change the permissions correctly
    create_webspace(user_id, &payload.user.id, &payload.website, &state.config)?;

    // Configure nginx by creating a new config file for it (or deleting and re-creating an existing file)
    write_nginx_conf(&payload.website, &payload.user.id, &payload.domains, &payload.forwarded_domains, &all_domains, &state.config)?;

    // Check the nginx configuration & reload the server
    verify_config()?;
    reload_server()?;

    // TODO:
    //   7. try to reach all domain names via HTTPS and expect 200 or 3xx (attention when using test certs)
    //   8. configure auto-update mechanism that also changes file permissions if required (?)

    Ok(())
}
