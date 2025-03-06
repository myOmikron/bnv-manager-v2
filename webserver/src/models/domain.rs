use rorm::prelude::ForeignModel;
use rorm::Model;

use crate::models::club::Club;

#[derive(Model)]
pub struct Domain {
    #[rorm(primary_key, max_length = 255)]
    pub domain: String,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<Club>,
}
