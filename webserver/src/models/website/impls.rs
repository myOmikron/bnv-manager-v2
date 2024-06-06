use rorm::db::Executor;
use rorm::fields::types::Json;
use rorm::insert;
use rorm::prelude::ForeignModel;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use rorm::Patch;
use uuid::Uuid;

use crate::http::handler_frontend::websites::schema::DeployState;
use crate::models::website::Website;
use crate::models::User;

impl Website {
    /// Create a new website
    pub async fn create_website(
        name: String,
        user: Uuid,
        executor: impl Executor<'_>,
    ) -> Result<Uuid, rorm::Error> {
        let mut guard = executor.ensure_transaction().await?;

        query!(guard.get_transaction(), User)
            .condition(User::F.uuid.equals(user))
            .one()
            .await?;

        let uuid = insert!(guard.get_transaction(), Website)
            .return_primary_key()
            .single(&WebsiteInsert {
                uuid: Uuid::new_v4(),
                name,
                owner: ForeignModelByField::Key(user),
                deploy_state: Json(DeployState::PendingChanges),
            })
            .await?;

        Ok(uuid)
    }
}

#[derive(Patch)]
#[rorm(model = "Website")]
struct WebsiteInsert {
    uuid: Uuid,
    name: String,
    owner: ForeignModel<User>,
    deploy_state: Json<DeployState>,
}
