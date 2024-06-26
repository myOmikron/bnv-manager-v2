use chrono::{DateTime, Utc};
use rorm::prelude::*;
use uuid::Uuid;

mod r#impl;
pub(crate) mod patches;

/// A single domain name and to which website that domain belongs (N:1 relation)
#[derive(Debug, Model)]
pub struct Domain {
    /// Primary key of a domain
    #[rorm(id)]
    pub id: i64,

    /// The full domain name (FQDN)
    #[rorm(unique, max_length = 255)]
    pub domain: String,

    /// Specification if the domain is actually hosted (false) or just forwarded to a hosted website (true)
    pub forwarded: bool,

    /// Reference to the website this domain belongs to
    pub website: ForeignModelByField<field!(Website::F.uuid)>,
}

/// Declaration of the deployment of a single, combined website by a user
#[derive(Debug, Model)]
pub struct Website {
    /// Website's UUID as supplied by the manager backend, which is a primary key there as well
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Back reference of all domains associated with this website
    pub domains: BackRef<field!(Domain::F.website)>,

    /// Reference to the owner of this website
    pub owner: ForeignModelByField<field!(User::F.uuid)>,

    /// Specification if a test certificate (staging Let's Encrypt CA) should be used
    pub test_cert: bool,

    /// Timestamp of the last update of this record
    #[rorm(auto_create_time, auto_update_time)]
    pub modified: DateTime<Utc>,
}

/// Effectively a copy of the most recent state of user from the manager backend
#[derive(Debug, Model)]
pub struct User {
    /// UUID of a user as obtained by the manager backend, where it is a primary key as well
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// LDAP common name of a website user; must be unique across all users (as enforced by the manager backend)
    #[rorm(index, max_length = 64)]
    pub cn: String,

    /// LDAP distinguished name of a website user; uniqueness enforced by the LDAP server
    #[rorm(unique, max_length = 255)]
    pub dn: String,

    /// Back reference of the websites owned by this user
    pub websites: BackRef<field!(Website::F.owner)>,
}
