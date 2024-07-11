use rorm::prelude::*;
use uuid::Uuid;

use super::User;
use super::Website;

#[derive(Debug, Patch)]
#[rorm(model = "User")]
pub(crate) struct NewUser {
    pub(crate) uuid: Uuid,
    pub(crate) cn: String,
    pub(crate) dn: String,
    pub(crate) posix_uid: i64,
}

#[derive(Patch)]
#[rorm(model = "Website")]
pub(crate) struct NewWebsite {
    pub(crate) uuid: Uuid,
    pub(crate) owner: ForeignModel<User>,
    pub(crate) test_cert: bool,
}
