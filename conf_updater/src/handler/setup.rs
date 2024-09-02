use axum::extract::State;
use axum::Json;
use itertools::Itertools;
use tracing::instrument;

use conf_updater_common::{ApiFailure, ProvisioningRequest};

use crate::server::AppState;
use crate::utils::certbot::{obtain_certificate, verify_cert};
use crate::utils::database::{ensure_existing_user, ensure_website_domains};
use crate::utils::dns::{ensure_resolvable_domains, validate_domain_names};
use crate::utils::nginx::{reload_server, verify_config, write_nginx_conf};
use crate::utils::web_space::create_web_space;

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
    ensure_website_domains(&payload.domains, &mut tx).await?;
    let website_owner = ensure_existing_user(&payload.user, &mut tx).await?;
    tx.commit().await?;

    let use_test_cert = state.config.certbot.test_certs || payload.test_certificate.unwrap_or(false);
    obtain_certificate(&payload.website, use_test_cert, &all_domains)?;
    verify_cert(&payload.website, &all_domains, &state.config.certbot)?;

    // TODO:
    //   2. create the Domain entries for all these
    //   3. create the Website referencing the Domains
    /*
    let website = ensure_existing_website(
        payload.website,
        payload.test_certificate.unwrap_or(false),
        website_owner.uuid,
        &mut tx,
    )
    .await?;
    tx.commit().await?;
    */

    // TODO:
    //   4. create the web root directory, add a simple index.html
    //   5. give ownership of the new web root to the owner user (requires POSIX user ID mapping), group goes to www-data
    create_web_space(&user_id.to_string(), &payload.website, &state.config)?;

    // Configure nginx by creating a new config file for it (or deleting and re-creating an existing file)
    write_nginx_conf(&payload.website, &user_id.to_string(), &payload.domains, &payload.forwarded_domains, &all_domains, &state.config)?;

    // Check the nginx configuration & reload the server
    verify_config()?;
    reload_server()?;

    // TODO:
    //   7. try to reach all domain names via HTTPS and expect 200 or 3xx (attention when using test certs)
    //   8. configure auto-update mechanism that also changes file permissions if required (?)

    Ok(())
}
