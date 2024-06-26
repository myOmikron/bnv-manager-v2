use rorm::Patch;
use uuid::Uuid;

use super::User;

#[derive(Debug, Patch)]
#[rorm(model = "User")]
pub(crate) struct NewUser {
    pub(crate) uuid: Uuid,
    pub(crate) cn: String,
    pub(crate) dn: String,
}
