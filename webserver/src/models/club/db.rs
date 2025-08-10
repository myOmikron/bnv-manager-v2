use rorm::Model;
use rorm::Patch;
use rorm::fields::types::MaxStr;
use uuid::Uuid;

#[derive(Debug, Model)]
#[rorm(rename = "Club")]
pub struct ClubModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    pub name: MaxStr<255>,
    pub description: MaxStr<1024>,

    #[rorm(auto_create_time, auto_update_time)]
    pub modified_at: time::OffsetDateTime,
    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Patch)]
#[rorm(model = "ClubModel")]
pub struct ClubModelInsert {
    pub uuid: Uuid,
    pub name: MaxStr<255>,
    pub description: MaxStr<1024>,
}
