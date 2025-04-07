use rorm::prelude::ForeignModel;
use rorm::Model;
use rorm::Patch;
use uuid::Uuid;

use crate::models::account::Account;

#[derive(Model)]
pub struct Club {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    #[rorm(max_length = 255, unique)]
    pub name: String,

    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
}

#[derive(Patch)]
#[rorm(model = "Club")]
pub struct ClubInsert {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Model)]
pub struct ClubAdmin {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub user: ForeignModel<Account>,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<Club>,
}

#[derive(Model)]
pub struct ClubUser {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub user: ForeignModel<Account>,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<Club>,
}
