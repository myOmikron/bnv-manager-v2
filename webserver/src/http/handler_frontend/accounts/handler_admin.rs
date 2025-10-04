//! Administration endpoints for accounts.

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use galvyn::rorm::Database;
use tracing::instrument;

use crate::http::handler_frontend::accounts::SimpleAccountSchema;
use crate::models::account::AdministrativeAccount;

#[get("/superadmins")]
#[instrument(name = "Api::admin::get_all_superadmins")]
pub async fn get_all_superadmins() -> ApiResult<ApiJson<Vec<SimpleAccountSchema>>> {
    let mut tx = Database::global().start_transaction().await?;

    let accounts = AdministrativeAccount::get_all(&mut tx)
        .await?
        .into_iter()
        .map(SimpleAccountSchema::from)
        .collect();

    tx.commit().await?;

    Ok(ApiJson(accounts))
}
