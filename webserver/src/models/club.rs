use rorm::prelude::ForeignModel;
use rorm::Model;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Model)]
pub struct Club {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    #[rorm(max_length = 255)]
    pub name: String,
}

#[derive(Model)]
pub struct ClubAdmin {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub user: ForeignModel<User>,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<Club>,
}

#[derive(Model)]
pub struct ClubUser {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub user: ForeignModel<User>,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<Club>,
}
