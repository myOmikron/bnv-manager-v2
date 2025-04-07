use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;

use crate::http::extractors::session_account::SessionAccount;
use crate::http::handler::me::schema::Me;

#[galvyn::get("/me")]
pub async fn get_me(
    SessionAccount {
        uuid,
        username,
        display_name,
        permissions,
    }: SessionAccount,
) -> ApiResult<ApiJson<Me>> {
    Ok(ApiJson(Me {
        uuid,
        username,
        display_name,
        permissions,
    }))
}
