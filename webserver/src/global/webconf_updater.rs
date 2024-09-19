//! The webconf updater

use std::time::Duration;

use reqwest::Client;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tracing::error;
use tracing::instrument;
use url::Url;
use uuid::Uuid;

/// The web config updater
#[derive(Debug)]
pub struct GlobalWebconfUpdater {
    url: Url,
    token: String,
    client: Client,
}

impl GlobalWebconfUpdater {
    /// Create a new instance of the [GlobalWebconfUpdater]
    pub fn new(url: Url, token: String) -> Self {
        // unwrap is okay as we don't set any parameters that can fail
        #[allow(clippy::unwrap_used)]
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        Self {
            // unwrap is okay as we construct as static url
            #[allow(clippy::unwrap_used)]
            url: url.join("/api/setupWebServerConfigs").unwrap(),
            token,
            client,
        }
    }

    #[instrument(skip(self), err)]
    pub async fn apply_changes(
        &self,
        change: WebconfChanges,
    ) -> Result<WebconfUpdateResult, reqwest::Error> {
        let res = self
            .client
            .post(self.url.clone())
            .bearer_auth(&self.token)
            .json::<Vec<Request>>(&vec![Request {
                domains: change.domains,
                forwarded_domains: vec![],
                user_uuid: change.user,
                website_uuid: change.website,
            }])
            .send()
            .await?;

        if res.status() != 200 {
            let txt = res.text().await?;
            error!("Received non 200 response code: {txt}");
            return Ok(WebconfUpdateResult::Fail);
        }

        let res: Response = res.json().await?;

        if !res.success {
            error!("Received response: {res:?}");
            return Ok(WebconfUpdateResult::Fail);
        }

        Ok(WebconfUpdateResult::Success)
    }
}

/// The changes to send to the webconf updater
#[derive(Debug)]
pub struct WebconfChanges {
    /// The user that owns the website
    pub user: Uuid,
    /// The identifier of the website
    pub website: Uuid,
    /// The domains to configure
    pub domains: Vec<String>,
}

/// The result of a webconf update request
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
#[allow(missing_docs)]
#[serde(tag = "res")]
pub enum WebconfUpdateResult {
    Success,
    Fail,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Request {
    domains: Vec<String>,
    forwarded_domains: Vec<String>,
    user_uuid: Uuid,
    website_uuid: Uuid,
}

#[derive(Debug, Deserialize)]
struct Response {
    message: Option<String>,
    success: bool,
    code: u16,
}
