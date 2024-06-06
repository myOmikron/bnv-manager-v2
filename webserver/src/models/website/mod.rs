//! The database models of a website

use rorm::field;
use rorm::fields::types::Json;
use rorm::prelude::BackRef;
use rorm::prelude::ForeignModel;
use rorm::Model;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::http::handler_frontend::websites::schema::DeployState;
use crate::models::User;

mod impls;

/// The representation of a website
#[derive(Model)]
pub struct Website {
    /// The primary key of a website
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The descriptive name of a website
    #[rorm(max_length = 255)]
    pub name: String,

    /// The owner of the website
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub owner: ForeignModel<User>,

    /// The domains that resolve to this website
    pub domains: BackRef<field!(WebsiteDomain::F.website)>,

    /// The current deploy state
    pub deploy_state: Json<DeployState>,

    /// The last point in time the website was deployed
    pub last_deployment: Option<OffsetDateTime>,

    /// The point in time the website was created
    #[rorm(auto_create_time)]
    pub created_at: OffsetDateTime,
}

/// A domain for a website
#[derive(Model)]
pub struct WebsiteDomain {
    /// The primary key of a website
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The domain
    #[rorm(max_length = 255)]
    pub domain: String,

    /// The corresponding website this domain should resolve to
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub website: ForeignModel<Website>,
}
