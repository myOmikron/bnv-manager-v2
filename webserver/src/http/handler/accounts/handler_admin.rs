use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use rorm::Database;

use crate::http::handler::accounts::schema;
use crate::models::role::Role;

#[get("/superadmins")]
pub async fn get_all_superadmins__admin() -> ApiResult<ApiJson<Vec<schema::SimpleAccount>>> {
    let mut tx = Database::global().start_transaction().await?;

    let accounts = Role::find_all_superadmins(&mut tx)
        .await?
        .into_iter()
        .map(|account| schema::SimpleAccount {
            uuid: account.uuid,
            username: account.username,
            display_name: account.display_name,
        })
        .collect();

    tx.commit().await?;

    Ok(ApiJson(accounts))
}
