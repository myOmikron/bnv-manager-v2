//! An extractor module for extracting the uuid of the user from the session

use std::fmt;
use std::ops;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use tower_sessions::Session;
use tracing::instrument;
use tracing::trace;
use uuid::Uuid;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
use crate::http::SESSION_USER;
use crate::models::ClubAdmin;
use crate::models::ClubUser;
use crate::models::User;

/// The extractor the user from the session
pub struct SessionUser {
    /// The model for the current session's user
    pub user: DebuggableUser,
    /// The role of the current session's user
    pub role: UserRoleWithClub,
}

/// Wrapper around [`User`] used in `SessionUser` to add a custom `Debug` implementation suitable for tracing
pub struct DebuggableUser(pub User);
impl fmt::Debug for DebuggableUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("uuid", &self.0.uuid)
            .field("username", &self.0.username)
            .field("display_name", &self.0.display_name)
            .field("preferred_lang", &self.0.preferred_lang)
            .finish()
    }
}
impl ops::Deref for DebuggableUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<DebuggableUser> for User {
    fn from(value: DebuggableUser) -> Self {
        value.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    #[instrument(level = "trace", skip_all)]
    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = match Session::from_request_parts(req, state).await {
            Ok(session) => session,
            Err((_, error_msg)) => return Err(ApiError::new_internal_server_error(error_msg)),
        };

        let Some(user_uuid) = session.get::<Uuid>(SESSION_USER).await? else {
            trace!("{SESSION_USER} is missing in session");
            return Err(ApiError::Unauthenticated);
        };
        let mut tx = GLOBAL.db.start_transaction().await?;
        let user = query!(&mut tx, User)
            .condition(User::F.uuid.equals(user_uuid))
            .optional()
            .await?
            .ok_or(ApiError::Unauthenticated)?;

        let role;
        let ca = query!(&mut tx, (ClubAdmin::F.club,))
            .condition(ClubAdmin::F.user.equals(user.uuid))
            .optional()
            .await?
            .map(|x| x.0);

        if let Some(ca) = ca {
            role = UserRoleWithClub::ClubAdmin { club: *ca.key() };
        } else {
            let cu = query!(&mut tx, (ClubUser::F.club,))
                .condition(ClubUser::F.user.equals(user.uuid))
                .optional()
                .await?
                .map(|x| x.0);

            if let Some(cu) = cu {
                role = UserRoleWithClub::User { club: *cu.key() };
            } else {
                role = UserRoleWithClub::Administrator;
            }
        }

        tx.commit().await?;

        Ok(SessionUser {
            user: DebuggableUser(user),
            role,
        })
    }
}
