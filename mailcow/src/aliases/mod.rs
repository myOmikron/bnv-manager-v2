//! Endpoints for managing aliases in mailcow

use tracing::instrument;

use crate::MailcowClient;
use crate::error::MailcowResult;

pub mod schema;

impl MailcowClient {
    /// Retrieves all aliases from the Mailcow API
    #[instrument(name = "MailcowClient::get_all_aliases", skip(self))]
    pub async fn get_all_aliases(&self) -> MailcowResult<Vec<schema::MailcowAlias>> {
        self.get("/api/v1/get/alias/all").send().await
    }

    /// Creates a new alias
    #[instrument(name = "MailcowClient::create_alias", skip(self))]
    pub async fn create_alias(&self, req: schema::CreateAliasRequest) -> MailcowResult<()> {
        self.post("/api/v1/add/alias")
            .body(&req)
            .send::<serde::de::IgnoredAny>()
            .await?;

        Ok(())
    }

    /// Deletes aliases by their IDs
    #[instrument(name = "MailcowClient::delete_aliases", skip(self))]
    pub async fn delete_aliases(&self, ids: Vec<u64>) -> MailcowResult<()> {
        let ids = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>();
        self.post("/api/v1/delete/alias")
            .body(&ids)
            .send::<serde::de::IgnoredAny>()
            .await?;

        Ok(())
    }
}
