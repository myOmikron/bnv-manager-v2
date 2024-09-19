/*
impl From<User> for WebsiteUser {
    fn from(value: User) -> Self {
        WebsiteUser {
            id: value.uuid,
            cn: value.cn,
            dn: value.dn,
            posix_uid: value.posix_uid.try_into().unwrap_or_else(|_| {
                error!(
                    "POSIX UID of user {} (DN: {}) is out of u32 range: {}",
                    value.uuid, value.dn, value.posix_uid
                );
                4294967295
            }),
        }
    }
}
*/
