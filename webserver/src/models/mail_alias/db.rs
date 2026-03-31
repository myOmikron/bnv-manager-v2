use galvyn::rorm::Model;
use galvyn::rorm::Patch;
use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::prelude::ForeignModel;
use uuid::Uuid;

use crate::models::account::db::ClubAccountModel;
use crate::models::domain::db::DomainModel;

#[derive(Debug, Model)]
#[rorm(rename = "MailAlias")]
pub struct MailAliasModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    pub local_part: MaxStr<255>,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub domain: ForeignModel<DomainModel>,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub account: ForeignModel<ClubAccountModel>,
    /// "pending", "approved", "rejected"
    pub status: MaxStr<32>,
    /// Mailcow alias ID, set when alias is created in Mailcow
    pub mailcow_id: Option<i64>,
    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
    #[rorm(auto_create_time, auto_update_time)]
    pub modified_at: time::OffsetDateTime,
}

#[derive(Debug, Patch)]
#[rorm(model = "MailAliasModel")]
pub struct MailAliasModelInsert {
    pub uuid: Uuid,
    pub local_part: MaxStr<255>,
    pub domain: ForeignModel<DomainModel>,
    pub account: ForeignModel<ClubAccountModel>,
    pub status: MaxStr<32>,
}
