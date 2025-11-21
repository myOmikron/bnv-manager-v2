//! Handler for mailboxes

use tracing::instrument;

use crate::MailcowClient;
use crate::error::MailcowResult;

pub mod schema;

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

    /// Sets a new app password for an existing mailbox
    #[instrument(skip(self, req), name = "MailcowClient::create_app_password")]
    pub async fn create_app_password(
        &self,
        req: schema::CreateAppPasswordRequest,
    ) -> MailcowResult<()> {
        let protocols = vec![
            "imap_access".to_string(),
            "dav_access".to_string(),
            "smtp_access".to_string(),
            "eas_access".to_string(),
            "pop3_access".to_string(),
            "sieve_access".to_string(),
        ];

        self.post("/api/v1/add/app-passwd")
            .body(&schema::InnerCreateAppPasswordRequest {
                active: "1".to_string(),
                username: req.username,
                app_name: req.app_name,
                app_passwd: req.app_passwd,
                app_passwd2: req.app_passwd2,
                protocols,
            })
            .send::<serde::de::IgnoredAny>()
            .await?;

        Ok(())
    }
}
