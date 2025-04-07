use rorm::field;
use rorm::prelude::BackRef;
use rorm::prelude::ForeignModel;
use rorm::Model;
use rorm::Patch;
use uuid::Uuid;

use crate::models::club::Club;
use crate::models::role::Role;

/// User representation
#[derive(Model)]
pub struct Account {
    /// Primary key of the account
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The backref to the roles of the account
    pub roles: BackRef<field!(AccountRole.account)>,

    /// An optional linked password of the user
    #[rorm(on_update = "Cascade", on_delete = "SetNull")]
    pub password: Option<ForeignModel<AccountPassword>>,

    /// The name of the user
    #[rorm(max_length = 255, unique)]
    pub username: String,

    /// The name of the user
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// Whether the user is disabled
    #[rorm(default = false)]
    pub disabled: bool,

    /// The point in time the user was created
    #[rorm(auto_create_time)]
    pub created_at: time::OffsetDateTime,
}

/// M2M between Accounts and Roles
#[derive(Model)]
pub struct AccountRole {
    /// Primary key
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The linked account
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub account: ForeignModel<Account>,

    /// The linked role
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub role: ForeignModel<Role>,

    /// The club associated with this account
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub club: Option<ForeignModel<Club>>,
}

/// Representation of a user password
#[derive(Model)]
pub struct AccountPassword {
    /// Primary key of a user password
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// User hash
    #[rorm(max_length = 1024)]
    pub password: String,
}

#[derive(Patch)]
#[rorm(model = "Account")]
pub(crate) struct AccountInsert {
    pub uuid: Uuid,
    pub username: String,
    pub display_name: String,
    pub password: Option<ForeignModel<AccountPassword>>,
}
