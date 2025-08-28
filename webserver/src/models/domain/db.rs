use rorm::Model;
use rorm::fields::types::MaxStr;
use rorm::prelude::ForeignModel;
use uuid::Uuid;

use crate::models::club::db::ClubModel;

#[derive(Debug, Model)]
#[rorm(rename = "Domain")]
pub struct DomainModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(unique)]
    pub domain: MaxStr<255>,
    pub club: ForeignModel<ClubModel>,
    #[rorm(default = "false")]
    pub is_primary: bool,
}
