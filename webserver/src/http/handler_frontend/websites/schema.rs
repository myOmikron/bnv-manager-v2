//! The schema for handling websites

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::schemars::SchemaDateTime;

/// The request to create a website
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CreateWebsiteRequest {
    /// The name of the website
    pub name: String,
}

/// The full representation of a domain that is attached to a website
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FullWebsiteDomain {
    /// The identifier of a specific domain
    pub uuid: Uuid,
    /// The attached domain
    pub domain: String,
}

/// The full representation of a website
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FullWebsite {
    /// The unique key of a website
    pub uuid: Uuid,
    /// Descriptive name of the website
    pub name: String,
    /// A list of domains for this website
    pub domains: Vec<FullWebsiteDomain>,
    /// The point in time the website was created
    pub created_at: SchemaDateTime,
    /// The current state of deployment
    pub deploy_state: DeployState,
    /// The last time the website was deployed
    pub last_deployment: Option<SchemaDateTime>,
}

/// The simple representation of a website
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SimpleWebsite {
    /// The unique key of a website
    pub uuid: Uuid,
    /// Descriptive name of the website
    pub name: String,
    /// The point in time the website was created
    pub created_at: SchemaDateTime,
    /// The current state of deployment
    pub deploy_state: DeployState,
    /// The last time the website was deployed
    pub last_deployment: Option<SchemaDateTime>,
}

/// A list of websites
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ListWebsites {
    /// The list of websites
    pub websites: Vec<SimpleWebsite>,
}

/// The request to update websites
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct UpdateWebsiteRequest {
    /// The name of the website
    pub name: String,
}

/// The request to add a domain to a website
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct AddDomainToWebsiteRequest {
    /// The domain to add
    pub domain: String,
}

/// The request to add a domain to a website
#[derive(Debug, Copy, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "field")]
#[allow(missing_docs)]
pub enum DomainField {
    AlreadyRegistered,
}

/// The request to add a domain to a website
#[derive(Debug, Copy, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "field")]
#[allow(missing_docs)]
pub enum AddDomainToWebsiteForm {
    Domain(DomainField),
}

/// The current deploy state
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum DeployState {
    /// The current state is deployed
    Deployed,
    /// There are pending changes
    PendingChanges,
    /// Deployment failed
    DeploymentFailed,
}

/// The path parameters to remove a domain from a website
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[allow(missing_docs)]
pub struct RemoveDomainPath {
    pub website_uuid: Uuid,
    pub domain_uuid: Uuid,
}
