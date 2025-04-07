use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use rorm::db::Executor;
use tracing::warn;

use crate::http::handler::users::schema::Permissions;
use crate::models::account::Account;
use crate::models::role::ROLE_ADMIN;
use crate::models::role::ROLE_CLUB_ADMIN;
use crate::models::role::ROLE_USER;

impl Account {
    pub async fn get_permissions(&mut self, exe: impl Executor<'_>) -> ApiResult<Permissions> {
        let mut guard = exe.ensure_transaction().await?;

        let roles = if let Some(roles) = &self.roles.cached {
            roles.clone()
        } else {
            Account
                .roles
                .populate(guard.get_transaction(), self)
                .await?;

            self.roles.cached.clone().unwrap_or_default()
        };

        let mut admin = false;
        let mut club_admin = vec![];
        let mut club_user = vec![];

        for role in roles {
            match role.role.0.as_str() {
                ROLE_ADMIN => {
                    admin = true;
                }
                ROLE_CLUB_ADMIN => {
                    club_admin.push(
                        role.club
                            .ok_or(ApiError::server_error("Clubadmin without associated club"))?
                            .0,
                    );
                }
                ROLE_USER => club_user.push(
                    role.club
                        .ok_or(ApiError::server_error("Clubuser without associated club"))?
                        .0,
                ),
                _ => warn!("Encountered invalid account role: {}", role.role.0),
            }
        }

        guard.commit().await?;

        Ok(Permissions {
            admin,
            club_admin,
            club_user,
        })
    }
}
