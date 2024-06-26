use conf_updater_common::WebsiteUser;

use crate::models::User;

impl From<User> for WebsiteUser {
    fn from(value: User) -> Self {
        WebsiteUser {
            id: value.uuid,
            cn: value.cn,
            dn: value.dn,
        }
    }
}
