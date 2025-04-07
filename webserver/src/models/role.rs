use rorm::Model;

pub const ROLE_USER: &str = "user";
pub const ROLE_CLUB_ADMIN: &str = "club-admin";
pub const ROLE_ADMIN: &str = "admin";

/// The available roles
pub const ROLES: [&str; 3] = [ROLE_USER, ROLE_CLUB_ADMIN, ROLE_ADMIN];

/// The role a user can have
#[derive(Model)]
pub struct Role {
    /// Name of the role
    #[rorm(primary_key, max_length = 255)]
    pub name: String,
}
