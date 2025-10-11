//! Handler for mailboxes

use tracing::instrument;

use crate::MailcowClient;
use crate::error::MailcowResult;

impl MailcowClient {
    /// Delete mailboxes
    ///
    /// **mailboxes**: List of mails to delete
    #[instrument(skip(self), name = "MailcowClient::delete_mailbox")]
    pub async fn delete_mailbox(&self, mailboxes: Vec<String>) -> MailcowResult<()> {
        self.post("/api/v1/delete/mailbox")
            .body(mailboxes)
            .send::<serde::de::IgnoredAny>()
            .await?;

        Ok(())
    }
}
