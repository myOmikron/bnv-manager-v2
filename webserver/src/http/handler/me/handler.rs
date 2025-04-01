use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::Module;
use rorm::Database;
use uuid::Uuid;

use crate::http::handler::me::schema::Me;
use crate::http::SESSION_USER;
use crate::models::user::User;

#[galvyn::get("/me")]
pub async fn get_me(session: Session) -> ApiResult<ApiJson<Me>> {
    let mut tx = Database::global().start_transaction().await?;

    let user_uuid: Uuid = session
        .get(SESSION_USER)
        .await?
        .ok_or(ApiError::server_error("invalid session"))?;

    let me = rorm::query(&mut tx, User)
        .condition(User.uuid.equals(user_uuid))
        .optional()
        .await?
        .ok_or(ApiError::server_error("invalid session"))?;

    tx.commit().await?;

    Ok(ApiJson(Me {
        uuid: me.uuid,
        admin: me.admin,
        username: me.username,
        display_name: me.display_name,
    }))
}
