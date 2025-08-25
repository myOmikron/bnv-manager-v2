use rorm::Model;
use rorm::fields::types::Json;
use rorm::fields::types::MaxStr;
use rorm::prelude::ForeignModel;
use uuid::Uuid;

use crate::models::account::db::AccountModel;

#[derive(Debug, Model)]
#[rorm(rename = "OidcProvider")]
pub struct OidcProviderModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    pub name: MaxStr<255>,
    pub client_secret: MaxStr<64>,
    pub redirect_uri: MaxStr<255>,
}

#[derive(Debug, Model)]
#[rorm(rename = "OidcAuthenticationToken")]
pub struct OidcAuthenticationTokenModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub provider: ForeignModel<OidcProviderModel>,
    pub redirect_url: MaxStr<255>,
    #[rorm(unique)]
    pub code: MaxStr<64>,
    pub expires_at: time::OffsetDateTime,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub account: ForeignModel<AccountModel>,
    pub nonce: Option<MaxStr<255>>,
    pub scopes: Json<Vec<String>>,
}
