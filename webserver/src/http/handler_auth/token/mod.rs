//! Translate an authentication code to a token and claims

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use base64ct::LineEnding;
use galvyn::core::Module;
use galvyn::core::re_exports::axum::Form;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::post;
use galvyn::rorm::Database;
use jsonwebtoken::Algorithm;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use rsa::pkcs1::EncodeRsaPrivateKey;
use tracing::instrument;

use crate::config::ORIGIN;
use crate::http::handler_auth::token::schema::Claims;
use crate::http::handler_auth::token::schema::EmailClaim;
use crate::http::handler_auth::token::schema::ProfileClaim;
use crate::http::handler_auth::token::schema::TokenRequest;
use crate::http::handler_auth::token::schema::TokenResponse;
use crate::models::oidc_provider::OidcAuthenticationToken;
use crate::modules::oidc::Oidc;

pub mod schema;

#[post("/token")]
#[instrument(name = "Api::auth::token")]
pub async fn get_token(
    Form(TokenRequest {
        grant_type,
        code,
        redirect_uri,
    }): Form<TokenRequest>,
) -> ApiResult<ApiJson<TokenResponse>> {
    let mut tx = Database::global().start_transaction().await?;

    if grant_type != "authorization_code" {
        return Err(ApiError::bad_request("Unsupported grant_type"));
    }

    let token = OidcAuthenticationToken::get_by_code(&mut tx, code).await?;
    let Some(token) = token else {
        return Err(ApiError::bad_request("Invalid authorization token"));
    };

    if token.redirect_url
        != redirect_uri
            .parse()
            .map_err(|_| ApiError::bad_request("Bad redirect_url"))?
    {
        return Err(ApiError::bad_request("Invalid redirect_uri"));
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(ApiError::map_server_error("Error calculating system time"))?
        .as_secs() as usize;
    let exp = now + 300;

    #[allow(clippy::expect_used)]
    let mut claims = Claims {
        iss: ORIGIN.to_string(),
        sub: token.account.uuid().0.to_string(),
        aud: token.client_id.0.to_string(),
        iat: now,
        exp,
        nonce: token.nonce.map(|x| x.to_string()),
        ..Default::default()
    };

    if token.scopes.iter().any(|x| x == "profile") {
        claims.profile_claim = Some(ProfileClaim {
            preferred_username: token.account.username.to_string(),
            name: token.account.display_name.to_string(),
        });
    }

    if token.scopes.iter().any(|x| x == "email") {
        claims.email_claim = Some(EmailClaim {
            email: token.account.email.to_string(),
            email_verified: true,
        });
    }

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(Oidc::global().kid.clone());

    let encoding_key = EncodingKey::from_rsa_pem(
        Oidc::global()
            .private_key
            .to_pkcs1_pem(LineEnding::LF)
            .map_err(ApiError::map_server_error("Couldn't convert to pem"))?
            .as_bytes(),
    )
    .map_err(ApiError::map_server_error("Couldn't parse key"))?;

    let id_token = jsonwebtoken::encode(&header, &claims, &encoding_key)
        .map_err(ApiError::map_server_error("Couldn't encode JWT"))?;

    #[allow(clippy::expect_used)]
    {
        claims.aud = ORIGIN
            .get()
            .join("api/v1/auth/userinfo")
            .expect("valid url")
            .to_string();
    }

    let access_token = jsonwebtoken::encode(&header, &claims, &encoding_key)
        .map_err(ApiError::map_server_error("Couldn't encode JWT"))?;

    OidcAuthenticationToken::delete_by_code(&mut tx, &token.code).await?;

    tx.commit().await?;

    Ok(ApiJson(TokenResponse {
        access_token,
        id_token,
        token_type: "Bearer".to_string(),
        expires_in: 300,
    }))
}
