use futures_util::TryStreamExt;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::core::Module;
use rorm::Database;

use crate::http::handler::users::schema::AdminAccount;
use crate::models::account::Account;
use crate::models::account::AccountRole;
use crate::models::role::ROLE_ADMIN;

#[galvyn::get("/users/admins")]
pub async fn get_admins() -> ApiResult<ApiJson<Vec<AdminAccount>>> {
    let mut tx = Database::global().start_transaction().await?;

    let users = rorm::query(&mut tx, (AccountRole.account.query_as(Account),))
        .condition(AccountRole.role.name.equals(ROLE_ADMIN))
        .stream()
        .map_ok(|(user,)| AdminAccount {
            uuid: user.uuid,
            username: user.username,
            display_name: user.display_name,
            disabled: user.disabled,
            created_at: SchemaDateTime(user.created_at),
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(users))
}
