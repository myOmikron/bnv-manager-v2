//! Authentication-related endpoints

use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Query;
use galvyn::core::re_exports::axum::response::Redirect;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use galvyn::post;
use galvyn::rorm::Database;
use tracing::error;
use tracing::info;
use tracing::instrument;

use crate::http::extractors::session_user::SESSION_USER;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_auth::auth::schema::AuthQuery;
use crate::http::handler_auth::auth::schema::SignInRequest;
use crate::models::account::Account;
use crate::models::oidc_provider::CreateOidcAuthenticationToken;
use crate::models::oidc_provider::OidcAuthenticationToken;
use crate::models::oidc_provider::OidcProvider;
use crate::utils::links::Link;

pub mod schema;

/// Access the original auth query using this variable
pub const SESSION_OIDC_AUTH: &str = "oidc-auth";

#[get("/auth")]
#[instrument(name = "Api::auth::auth")]
pub async fn auth(Query(auth_query): Query<AuthQuery>, session: Session) -> ApiResult<Redirect> {
    let mut tx = Database::global().start_transaction().await?;

    // Validate parameters
    if auth_query.response_type.as_str() != "code" {
        return Err(ApiError::bad_request("Invalid response type"));
    }
    if let Some(response_mode) = &auth_query.response_mode
        && response_mode != "query"
    {
        return Err(ApiError::bad_request("Invalid response mode"));
    }

    let provider = OidcProvider::find_by_client_id(&mut tx, auth_query.client_id)
        .await?
        .ok_or(ApiError::bad_request("Invalid client_id"))?;

    if provider.redirect_uri != auth_query.redirect_uri {
        return Err(ApiError::bad_request("Invalid redirect_uri"));
    }

    tx.commit().await?;

    // Insert into the session for later use
    session.insert(SESSION_OIDC_AUTH, auth_query).await?;

    // Check login state
    let user = session.get::<SessionUser>(SESSION_USER).await?;
    // Show login page to user
    if user.is_none() {
        return Ok(Redirect::temporary(Link::oidc_auth().as_str()));
    }

    // Redirect to finish-auth as the user is already logged in
    Ok(Redirect::temporary(Link::oidc_finish().as_str()))
}

#[post("/sign-in")]
#[instrument(name = "Api::auth::sign_in", skip(password))]
pub async fn sign_in(
    session: Session,
    ApiJson(SignInRequest { username, password }): ApiJson<SignInRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let account = Account::find_by_username(&mut tx, &username)
        .await?
        .ok_or(ApiError::bad_request("Username not found"))?;

    tx.commit().await?;

    if !account.check_password(password)? {
        return Err(ApiError::bad_request("Invalid password"));
    }

    session
        .insert(
            SESSION_USER,
            SessionUser {
                uuid: account.uuid(),
            },
        )
        .await?;

    Ok(())
}

/// Allowed scopes
pub const ALLOWED_SCOPES: &[&str] = &["openid", "profile", "email"];

#[get("/finish-auth")]
#[instrument(name = "Api::auth::finish-auth")]
pub async fn finish_auth(session: Session) -> ApiResult<Redirect> {
    let mut tx = Database::global().start_transaction().await?;
    let auth_query: AuthQuery = session
        .remove(SESSION_OIDC_AUTH)
        .await?
        .ok_or(ApiError::bad_request("No auth query"))?;

    let session_user: SessionUser = session
        .get(SESSION_USER)
        .await?
        .ok_or(ApiError::bad_request("Missing session user"))?;

    let provider = OidcProvider::find_by_client_id(&mut tx, auth_query.client_id)
        .await?
        .ok_or(ApiError::server_error(
            "Provider deleted between start and finish auth",
        ))?;

    let requested_scopes: Vec<_> = auth_query.scope.split(" ").collect();
    info!(requested_scopes = ?requested_scopes);
    if !requested_scopes.contains(&"openid") {
        return Err(ApiError::bad_request("Missing required scope openid"));
    }
    for scope in &requested_scopes {
        if !ALLOWED_SCOPES.contains(scope) {
            error!(scope = *scope, "Invalid scope requested");
            return Err(ApiError::bad_request("Invalid scope requested"));
        }
    }

    // Create a new token
    let auth_token = OidcAuthenticationToken::create(
        &mut tx,
        CreateOidcAuthenticationToken {
            provider: provider.client_id,
            redirect_url: provider.redirect_uri,
            account: session_user.uuid,
            nonce: auth_query.nonce,
            scopes: requested_scopes
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        },
    )
    .await?;

    tx.commit().await?;

    let mut redirect_uri = auth_query.redirect_uri;
    redirect_uri.set_query(Some(&format!(
        "code={code}{state}",
        code = auth_token.code,
        state = auth_query
            .state
            .map(|state| format!("&state={state}"))
            .unwrap_or_default()
    )));
    Ok(Redirect::temporary(redirect_uri.as_str()))
}

#[post("/sign-out")]
#[instrument(name = "Api::auth::sign_out")]
pub async fn sign_out(session: Session) -> ApiResult<()> {
    session.remove::<SessionUser>(SESSION_USER).await?;
    Ok(())
}
