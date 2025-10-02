use rorm::Model;
use rorm::Patch;
use rorm::fields::types::MaxStr;
use rorm::prelude::ForeignModel;
use uuid::Uuid;

use crate::models::club::db::ClubModel;

#[derive(Debug, Model)]
pub struct InviteModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    /// Usernames **MUST** be lowercase to allow case-insensitive logins.
    #[rorm(unique)]
    pub username: MaxStr<255>,
    pub display_name: MaxStr<255>,

    pub expires_at: time::OffsetDateTime,

    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Patch)]
#[rorm(model = "InviteModel")]
pub struct InviteModelInsert {
    pub uuid: Uuid,
    pub username: MaxStr<255>,
    pub display_name: MaxStr<255>,
    pub expires_at: time::OffsetDateTime,
}

#[derive(Debug, Model)]
#[rorm(rename = "InvitedClubMember")]
pub struct InvitedClubMemberModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub invite: ForeignModel<InviteModel>,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<ClubModel>,
    #[rorm(unique)]
    pub email: MaxStr<255>,
}

#[derive(Debug, Model)]
#[rorm(rename = "InvitedClubAdmin")]
pub struct InvitedClubAdminModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub invite: ForeignModel<InviteModel>,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: ForeignModel<ClubModel>,
}

#[derive(Debug, Model)]
#[rorm(rename = "InvitedSuperAdmin")]
pub struct InvitedSuperAdminModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub invite: ForeignModel<InviteModel>,
}
