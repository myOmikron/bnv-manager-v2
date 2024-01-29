//! This module holds all ldap related code

use std::time::Duration;

use ldap3::{Ldap, LdapConnAsync, LdapConnSettings};
use tracing::{debug, debug_span, info, instrument, Instrument};
use url::Url;

/// The connection to the LDAP server

pub struct LdapConn {
    ldap: Ldap,
    bind_dn: String,
    bind_pw: String,
}

impl LdapConn {
    /// Create a new instance of a ldap connection
    #[instrument(level = "debug", skip_all, name = "LdapConn::new")]
    pub async fn new(
        uri: &Url,
        bind_dn: String,
        bind_pw: String,
        do_not_verify_certs: bool,
    ) -> Result<Self, ldap3::LdapError> {
        let (conn, mut ldap) = LdapConnAsync::with_settings(
            LdapConnSettings::new()
                .set_conn_timeout(Duration::from_secs(5))
                .set_no_tls_verify(do_not_verify_certs),
            uri.as_str(),
        )
        .instrument(debug_span!("ldap-connection"))
        .await?;

        info!("Connected to LDAP");
        ldap3::drive!(conn);

        debug!("Try to bind with given credentials");
        ldap.simple_bind(&bind_dn, &bind_pw).await?;
        debug!("Successfully performed bind operation");

        Ok(LdapConn {
            ldap,
            bind_dn,
            bind_pw,
        })
    }
}

impl LdapConn {
    /// Retrieve a handle
    ///
    /// The handle is already logged-in
    #[instrument(skip_all)]
    pub async fn get_handle(&self) -> Result<Ldap, ldap3::LdapError> {
        let mut ldap = self.ldap.clone();
        debug!("Performing bind with new ldap handle");
        ldap.simple_bind(&self.bind_dn, &self.bind_pw).await?;
        debug!("Successfully performed bind on new ldap handle");
        Ok(ldap)
    }
}
