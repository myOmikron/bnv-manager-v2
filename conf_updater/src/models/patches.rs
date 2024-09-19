use rorm::prelude::*;
use uuid::Uuid;

use super::Domain;
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

#[derive(Debug, Patch)]
#[rorm(model = "Website")]
pub(crate) struct NewWebsite {
    pub(crate) uuid: Uuid,
    pub(crate) owner: ForeignModel<User>,
    pub(crate) test_cert: bool,
}

#[derive(Debug, Patch)]
#[rorm(model = "Domain")]
pub(crate) struct NewDomain {
    pub(crate) domain: String,
    pub(crate) forwarded: bool,
    pub(crate) website: ForeignModelByField<field!(Website::F.uuid)>,
}
