use rorm::Model;
use rorm::Patch;
use rorm::fields::types::MaxStr;
use uuid::Uuid;

#[derive(Debug, Model)]
#[rorm(rename = "Account")]
pub struct AccountModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Usernames **MUST** be lowercase to allow case-insensitive login.
    #[rorm(unique)]
    pub username: MaxStr<255>,

    pub display_name: MaxStr<255>,

    pub hashed_password: MaxStr<255>,

    #[rorm(auto_create_time, auto_update_time)]
    pub modified_at: time::OffsetDateTime,
    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Patch)]
#[rorm(model = "AccountModel")]
pub struct AccountModelInsert {
    pub uuid: Uuid,
    pub username: MaxStr<255>,
    pub display_name: MaxStr<255>,
    pub hashed_password: MaxStr<255>,
}
