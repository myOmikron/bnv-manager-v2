use futures_util::StreamExt;
use futures_util::TryStreamExt;
use rorm::db::executor;
use rorm::db::sql::value::Value;
use rorm::db::Executor;
use rorm::Model;
use swaggapi::get;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::extractors::api_json::ApiJson;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::users::schema::SimpleUser;
use crate::models::User;
use crate::models::UserRole;

/// Retrieve the users of a club
#[get("/")]
pub async fn get_club_users_ca(
    SessionUser { user }: SessionUser,
) -> ApiResult<ApiJson<Vec<SimpleUser>>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let Some(club) = user.0.club.map(|x| *x.key()) else {
        // CLub admins should always have their clubs set
        return Err(ApiError::BadRequest);
    };

    let user_role: &'static str = UserRole::User.into();
    // TODO: Fix when rorm supports Option<ForeignModel> in queries
    let users: Vec<SimpleUser> = tx
        .execute::<executor::Stream>(
            format!(
                r#"SELECT "uuid", "username", "display_name", "club" FROM "{}" WHERE "club" = $1 AND "role" = $2;"#,
                User::TABLE,
            ),
            vec![Value::Uuid(club), Value::String(user_role)],
        )
        .map(|x| {
            match x {
                Ok(x) => Ok(SimpleUser {
                    uuid: x.get("uuid")?,
                    username: x.get("username")?,
                    display_name: x.get("display_name")?,
                    role: UserRole::User,
                }),
                Err(err) => Err(err)
            }
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(users))
}
