use futures_util::TryStreamExt;
use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::SchemaDateTime;
use rorm::Database;

use crate::http::handler::users::schema::AdminUser;
use crate::models::user::User;

#[galvyn::get("/users/admins")]
pub async fn admin_get_admins() -> ApiResult<ApiJson<Vec<AdminUser>>> {
    let mut tx = Database::global().start_transaction().await?;

    let users = rorm::query(&mut tx, User)
        .condition(User.admin.equals(true))
        .stream()
        .map_ok(|user| AdminUser {
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
