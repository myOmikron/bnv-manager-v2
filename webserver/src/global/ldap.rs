//! The global ldap manager

use std::time::Duration;

use ldap3::ldap_escape;
use ldap3::Ldap;
use ldap3::LdapConnAsync;
use ldap3::LdapConnSettings;
use ldap3::LdapError;
use ldap3::Scope;
use ldap3::SearchEntry;
use log::debug;
use log::info;
use log::warn;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;

use crate::config::LdapConfig;

const LDAP_TIMEOUT: Duration = Duration::from_secs(20);

/// The global ldap manager
#[derive(Debug)]
pub struct GlobalLdap {
    conn: Ldap,
    ldap_config: LdapConfig,
}

impl GlobalLdap {
    /// Initialize the ldap connection
    #[instrument(ret, err)]
    pub async fn new(ldap_config: LdapConfig) -> Result<Self, LdapError> {
        let mut conn = Self::init_conn(&ldap_config).await?;

        info!("Connected to ldap");

        conn.with_timeout(LDAP_TIMEOUT)
            .simple_bind(&ldap_config.admin_bind_dn, &ldap_config.admin_bind_pw)
            .await?
            .success()?;

        info!("Bind with admin successful");

        Ok(Self { conn, ldap_config })
    }

    /// Try binding with the given DN and user
    #[instrument(skip(self), ret, err)]
    pub async fn auth_dn(&self, dn: &str, password: &str) -> Result<bool, LdapError> {
        let mut conn = self.conn.clone();

        let success = conn
            .with_timeout(LDAP_TIMEOUT)
            .simple_bind(dn, password)
            .await?;

        Ok(success.success().is_ok())
    }

    /// Retrieve a DN for a given username
    #[instrument(skip(self), ret, err)]
    pub async fn get_user(&self, username: String) -> Result<Option<LdapUserData>, LdapError> {
        let mut conn = self.admin_bind().await?;

        let filter = self
            .ldap_config
            .user_search_filter
            .replace('*', &ldap_escape(username));

        debug!("Using filter: {filter}");

        let (results, _) = conn
            .with_timeout(LDAP_TIMEOUT)
            .search(
                &self.ldap_config.user_search_base,
                Scope::Subtree,
                &filter,
                ["dn", "givenName", "sn"],
            )
            .await?
            .success()?;

        let res = results.into_iter().next().map(SearchEntry::construct);

        match res {
            None => Ok(None),
            Some(search_entry) => {
                let Some(given_name) = search_entry.attrs.get("givenName").map(|x| x.join(" "))
                else {
                    return Ok(None);
                };
                let Some(Some(sn)) = search_entry.attrs.get("sn").map(|x| x.iter().next()) else {
                    return Ok(None);
                };

                return Ok(Some(LdapUserData {
                    dn: search_entry.dn,
                    display_name: format!("{given_name} {sn}"),
                }));
            }
        }
    }

    async fn admin_bind(&self) -> Result<Ldap, LdapError> {
        let mut conn = self.conn.clone();

        conn.with_timeout(LDAP_TIMEOUT)
            .simple_bind(
                &self.ldap_config.admin_bind_dn,
                &self.ldap_config.admin_bind_pw,
            )
            .await?
            .success()?;

        Ok(conn)
    }

    async fn init_conn(ldap_config: &LdapConfig) -> Result<Ldap, LdapError> {
        let (conn, ldap) = LdapConnAsync::with_settings(
            LdapConnSettings::new()
                .set_conn_timeout(LDAP_TIMEOUT)
                .set_starttls(ldap_config.start_tls)
                .set_no_tls_verify(ldap_config.no_tls_verify),
            &ldap_config.uri,
        )
        .await?;

        debug!("Opened ldap connection");

        tokio::spawn(async move {
            if let Err(err) = conn.drive().await {
                warn!("Ldap error: {err}");
            }
        });

        Ok(ldap)
    }
}

/// The data retrieve from a ldap user
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct LdapUserData {
    /// The distinguished name of the
    pub dn: String,
    /// The display name of the user
    pub display_name: String,
}
