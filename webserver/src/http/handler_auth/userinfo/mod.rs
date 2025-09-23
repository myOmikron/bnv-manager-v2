use base64ct::LineEnding;
use galvyn::core::Module;
use galvyn::core::re_exports::axum::http::HeaderMap;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::get;
use galvyn::rorm::Database;
use jsonwebtoken::Validation;
use rsa::pkcs8::EncodePublicKey;
use tracing::info;
use tracing::instrument;

use crate::modules::oidc::Oidc;

mod schema;

#[get("/userinfo")]
#[instrument(name = "Api::auth::userinfo")]
pub async fn get_userinfo(headers: HeaderMap) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let Some(header) = headers.get("Authorization") else {
        return Err(ApiError::bad_request("Missing Authorization header"));
    };

    let token = header
        .to_str()
        .map_err(ApiError::map_server_error("Invalid header value"))?
        .strip_prefix("Bearer ")
        .ok_or(ApiError::bad_request("Missing Bearer prefix"))?;

    let token = jsonwebtoken::decode::<crate::http::handler_auth::token::schema::Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(
            Oidc::global()
                .private_key
                .to_public_key()
                .to_public_key_pem(LineEnding::LF)
                .map_err(ApiError::map_server_error("Couldn't convert to pem"))?
                .as_bytes(),
        )
        .map_err(ApiError::map_server_error("Couldn't parse key"))?,
        &Validation::default(),
    )
    .map_err(ApiError::map_server_error("Invalid token"))?;

    info!(token_data = ?token);

    tx.commit().await?;
    Ok(())
}
