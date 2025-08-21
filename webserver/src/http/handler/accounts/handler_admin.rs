//! Administration endpoints for accounts.

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use rorm::Database;
use tracing::instrument;

use crate::http::handler::accounts::schema;
use crate::models::role::Role;

#[get("/superadmins")]
#[instrument(name = "Api::admin::get_all_superadmins")]
pub async fn get_all_superadmins() -> ApiResult<ApiJson<Vec<schema::SimpleAccount>>> {
    let mut tx = Database::global().start_transaction().await?;

    let accounts = Role::find_all_superadmins(&mut tx)
        .await?
        .into_iter()
        .map(schema::SimpleAccount::from)
        .collect();

    tx.commit().await?;

    Ok(ApiJson(accounts))
}
