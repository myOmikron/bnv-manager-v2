use rorm::Model;
use rorm::prelude::ForeignModel;
use uuid::Uuid;

use crate::models::account::db::AccountModel;
use crate::models::club::db::ClubModel;

#[derive(Debug, Model)]
#[rorm(rename = "ClubMember")]
pub struct ClubMemberModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub account: ForeignModel<AccountModel>,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<ClubModel>,
}

#[derive(Debug, Model)]
#[rorm(rename = "ClubAdmin")]
pub struct ClubAdminModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub account: ForeignModel<AccountModel>,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<ClubModel>,
}

#[derive(Debug, Model)]
#[rorm(rename = "SuperAdmin")]
pub struct SuperAdminModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub account: ForeignModel<AccountModel>,
}
