//! Administrative endpoints for domains

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use galvyn::rorm::Database;
use tracing::instrument;

use crate::http::handler_frontend::domains::schema;
use crate::models::domain::Domain;

/// Retrieve all domains that aren't associated with a club
#[get("/unassociated")]
#[instrument(name = "Api::admin::get_unassociated_domains")]
pub async fn get_unassociated_domains() -> ApiResult<ApiJson<Vec<schema::Domain>>> {
    let mut tx = Database::global().start_transaction().await?;

    let domains = Domain::find_all_unassociated(&mut tx)
        .await?
        .into_iter()
        .map(|x| schema::Domain {
            uuid: x.uuid,
            domain: x.domain,
        })
        .collect();

    tx.commit().await?;

    Ok(ApiJson(domains))
}
