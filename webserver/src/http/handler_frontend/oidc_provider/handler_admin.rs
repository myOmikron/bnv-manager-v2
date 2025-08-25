//! Endpoints for managing oidc providers

use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use galvyn::post;
use rorm::Database;
use tracing::instrument;

use crate::http::handler_frontend::oidc_provider::CreateOidcProvider;
use crate::http::handler_frontend::oidc_provider::schema;
use crate::models::oidc_provider::OidcProvider;
use crate::models::oidc_provider::OidcProviderUuid;

#[get("/")]
#[instrument(name = "Api::admin::get_all_oidc_providers")]
pub async fn get_all_oidc_providers() -> ApiResult<ApiJson<Vec<schema::OidcProvider>>> {
    let providers = OidcProvider::find_all(Database::global())
        .await?
        .into_iter()
        .map(schema::OidcProvider::from)
        .collect();

    Ok(ApiJson(providers))
}

#[post("/")]
#[instrument(name = "Api::admin::create_oidc_provider")]
pub async fn create_oidc_provider(
    ApiJson(CreateOidcProvider { name, redirect_uri }): ApiJson<CreateOidcProvider>,
) -> ApiResult<ApiJson<OidcProviderUuid>> {
    let mut tx = Database::global().start_transaction().await?;

    let provider = OidcProvider::create(&mut tx, name, redirect_uri).await?;

    tx.commit().await?;

    Ok(ApiJson(provider.client_id))
}
