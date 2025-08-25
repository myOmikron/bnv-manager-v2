//! Administrative endpoints for domains

use std::collections::HashMap;
use std::ops::Not;

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use rorm::Database;
use rorm::fields::types::MaxStr;
use tracing::info;
use tracing::instrument;

use crate::http::handler_frontend::domains::schema;
use crate::models::domain::Domain;
use crate::modules::mailcow::Mailcow;

#[get("/")]
#[instrument(name = "Api::domain::get_unassociated_domains")]
pub async fn get_unassociated_domains() -> ApiResult<ApiJson<Vec<schema::Domain>>> {
    let mut tx = Database::global().start_transaction().await?;

    // Get associated domains
    let associated_domains: HashMap<String, Domain> = Domain::find_all_associated(&mut tx)
        .await?
        .into_iter()
        .map(|x| (x.domain.to_string(), x))
        .collect();

    // Retrieve domains from mailcow
    let mailcow_domains =
        Mailcow::global()
            .sdk
            .get_all_domains()
            .await
            .map_err(ApiError::map_server_error(
                "Couldn't fetch domains from mailcow",
            ))?;
    info!(domains_in_mailcow = ?mailcow_domains);

    let mut domains: HashMap<String, _> = mailcow_domains
        .into_iter()
        .filter_map(|domain| {
            associated_domains
                .contains_key(&domain.domain_name)
                .not()
                .then(|| (domain.domain_name.clone(), domain))
        })
        .collect();

    // Get unassociated domains
    let unassociated_domains = Domain::find_all_unassociated(&mut tx).await?;

    // Find new domains
    let mut to_delete = vec![];
    let mut to_add = vec![];

    for domain in unassociated_domains {
        let existing_domain = domains.remove(&domain.domain.to_string());
        if existing_domain.is_none() {
            to_delete.push(domain);
        }
    }
    for domain in domains {
        to_add.push(domain.1);
    }

    for domain in to_delete {
        Domain::delete_by_domain(&mut tx, domain.domain).await?;
    }

    for domain in to_add {
        Domain::create(
            &mut tx,
            MaxStr::new(domain.domain_name)
                .map_err(ApiError::map_server_error("Domain too long"))?,
        )
        .await?;
    }

    let domains = Domain::find_all_unassociated(&mut tx)
        .await?
        .into_iter()
        .map(schema::Domain::from)
        .collect();

    tx.commit().await?;

    Ok(ApiJson(domains))
}
