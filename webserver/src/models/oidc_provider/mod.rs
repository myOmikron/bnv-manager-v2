//! OIDC related models

use futures_util::TryStreamExt;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use rorm::and;
use rorm::db::Executor;
use rorm::fields::types::Json;
use rorm::fields::types::MaxStr;
use rorm::prelude::ForeignModelByField;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use time::Duration;
use time::OffsetDateTime;
use tracing::instrument;
use url::Url;
use uuid::Uuid;

use crate::models::account::AccountUuid;
use crate::models::oidc_provider::db::OidcAuthenticationTokenModel;
use crate::models::oidc_provider::db::OidcProviderModel;

pub(in crate::models) mod db;

/// Representation of an oidc provider
#[derive(Debug)]
pub struct OidcProvider {
    /// Human-readable name for identifying the provider
    pub name: MaxStr<255>,
    /// Client id of the provider
    pub client_id: OidcProviderUuid,
    /// Client secret of the provider
    pub client_secret: MaxStr<64>,
    /// The redirect url that should be valid
    pub redirect_uri: Url,
}

/// Client id of an oidc provider
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OidcProviderUuid(pub Uuid);

impl OidcProvider {
    /// Create a new oidc provider
    #[instrument(name = "OidcProvider::create", skip(exe))]
    pub async fn create(
        exe: impl Executor<'_>,
        name: MaxStr<255>,
        redirect_url: Url,
    ) -> anyhow::Result<Self> {
        let client_secret = MaxStr::new(Alphanumeric.sample_string(&mut rand::thread_rng(), 64))?;

        Ok(Self::from(
            rorm::insert(exe, OidcProviderModel)
                .single(&OidcProviderModel {
                    uuid: Uuid::new_v4(),
                    name,
                    client_secret,
                    redirect_uri: MaxStr::new(redirect_url.to_string())?,
                })
                .await?,
        ))
    }

    /// Find an oidc provider by its client id
    #[instrument(name = "OidcProvider::find_by_client_id", skip(exe))]
    pub async fn find_by_client_id(
        exe: impl Executor<'_>,
        client_id: OidcProviderUuid,
    ) -> anyhow::Result<Option<OidcProvider>> {
        Ok(rorm::query(exe, OidcProviderModel)
            .condition(OidcProviderModel.uuid.equals(client_id.0))
            .optional()
            .await?
            .map(OidcProvider::from))
    }

    /// Find all OIDC providers
    #[instrument(name = "OidcProvider::find_all", skip(exe))]
    pub async fn find_all(exe: impl Executor<'_>) -> anyhow::Result<Vec<Self>> {
        Ok(rorm::query(exe, OidcProviderModel)
            .order_asc(OidcProviderModel.name)
            .stream()
            .map_ok(OidcProvider::from)
            .try_collect()
            .await?)
    }
}

/// A short-lived authentication token
#[derive(Debug)]
pub struct OidcAuthenticationToken {
    /// The code
    pub code: MaxStr<64>,
    /// The provider that is linked to this token
    pub provider: OidcProviderUuid,
    /// The point in time the token will expire
    pub expires_at: OffsetDateTime,
    /// The redirect url linked to the token
    pub redirect_url: Url,
    /// Linked account id
    pub account_id: AccountUuid,
    /// Optional nonce to protect against replay attacks
    pub nonce: Option<MaxStr<255>>,
    /// Scopes the client has requested
    pub scopes: Vec<String>,
}

impl OidcAuthenticationToken {
    /// Create a new authentication token
    #[instrument(name = "OidcAuthenticationToken::create", skip(exe))]
    pub async fn create(
        exe: impl Executor<'_>,
        CreateOidcAuthenticationToken {
            provider,
            redirect_url,
            account,
            nonce,
            scopes,
        }: CreateOidcAuthenticationToken,
    ) -> anyhow::Result<Self> {
        let code = MaxStr::new(Alphanumeric.sample_string(&mut rand::thread_rng(), 64))?;

        Ok(Self::from(
            rorm::insert(exe, OidcAuthenticationTokenModel)
                .single(&OidcAuthenticationTokenModel {
                    uuid: Uuid::new_v4(),
                    provider: ForeignModelByField(provider.0),
                    redirect_url: MaxStr::new(redirect_url.to_string())?,
                    code,
                    expires_at: OffsetDateTime::now_utc() + Duration::minutes(10),
                    account: ForeignModelByField(account.0),
                    nonce,
                    scopes: Json(scopes),
                })
                .await?,
        ))
    }

    /// Retrieve an authentication token
    #[instrument(name = "OidcAuthenticationToken::find_by_code", skip(exe))]
    pub async fn get_by_code(
        exe: impl Executor<'_>,
        code: MaxStr<64>,
    ) -> anyhow::Result<Option<OidcAuthenticationToken>> {
        let now = OffsetDateTime::now_utc();
        Ok(rorm::query(exe, OidcAuthenticationTokenModel)
            .condition(and![
                OidcAuthenticationTokenModel.expires_at.greater_than(now),
                OidcAuthenticationTokenModel.code.equals(&*code)
            ])
            .optional()
            .await?
            .map(OidcAuthenticationToken::from))
    }

    /// Delete an authentication token by its code
    #[instrument(name = "OidcAuthenticationToken::delete_by_code", skip(exe))]
    pub async fn delete_by_code(exe: impl Executor<'_>, code: &MaxStr<64>) -> anyhow::Result<()> {
        rorm::delete(exe, OidcAuthenticationTokenModel)
            .condition(OidcAuthenticationTokenModel.code.equals(&**code))
            .await?;

        Ok(())
    }
}

/// Request to create a oidc authentication token
#[derive(Debug)]
pub struct CreateOidcAuthenticationToken {
    /// The provider this token should be linked to
    pub provider: OidcProviderUuid,
    /// Redirect url
    pub redirect_url: Url,
    /// The account associated with the request
    pub account: AccountUuid,
    /// Optional nonce to avoid replay attacks
    pub nonce: Option<MaxStr<255>>,
    /// Scopes the client has requested
    pub scopes: Vec<String>,
}

impl From<OidcProviderModel> for OidcProvider {
    fn from(model: OidcProviderModel) -> Self {
        // Unwrap is fine as the creation of the model also takes an url
        #[allow(clippy::unwrap_used)]
        Self {
            name: model.name,
            client_id: OidcProviderUuid(model.uuid),
            client_secret: model.client_secret,
            redirect_uri: Url::parse(&model.redirect_uri).unwrap(),
        }
    }
}

impl From<OidcAuthenticationTokenModel> for OidcAuthenticationToken {
    fn from(value: OidcAuthenticationTokenModel) -> Self {
        // Unwrap is fine as the creation of the model also takes an url
        #[allow(clippy::unwrap_used)]
        Self {
            code: value.code,
            provider: OidcProviderUuid(value.provider.0),
            expires_at: value.expires_at,
            redirect_url: Url::parse(&value.redirect_url).unwrap(),
            account_id: AccountUuid(value.account.0),
            nonce: value.nonce,
            scopes: value.scopes.0,
        }
    }
}
