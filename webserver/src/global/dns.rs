//! DNS resolution

use std::net::IpAddr;

use hickory_resolver::config::LookupIpStrategy;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::config::ResolverOpts;
use hickory_resolver::name_server::GenericConnector;
use hickory_resolver::name_server::TokioRuntimeProvider;
use hickory_resolver::AsyncResolver;
use tracing::debug_span;
use tracing::instrument;
use tracing::Instrument;

use crate::http::handler_frontend::ws::schema::ResolveResult;

/// The global DNS resolver
#[derive(Debug)]
pub struct GlobalDns {
    resolver: AsyncResolver<GenericConnector<TokioRuntimeProvider>>,
}

impl GlobalDns {
    /// Construct a new dns resolver
    pub fn new() -> Self {
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;

        Self {
            resolver: AsyncResolver::tokio(ResolverConfig::google(), resolver_opts),
        }
    }

    /// Resolve a incoming domain
    #[instrument(skip(self))]
    pub async fn resolve(
        &self,
        domain: &str,
    ) -> Result<ResolveResult, hickory_resolver::error::ResolveError> {
        let res = self
            .resolver
            .lookup_ip(domain)
            .instrument(debug_span!("DNS lookup"))
            .await?;

        let mut resolved = ResolveResult {
            ipv4: None,
            ipv6: None,
        };

        for results in res {
            match results {
                IpAddr::V4(addr) => resolved.ipv4 = Some(addr),
                IpAddr::V6(addr) => resolved.ipv6 = Some(addr),
            }
        }

        Ok(resolved)
    }
}

impl Default for GlobalDns {
    fn default() -> Self {
        Self::new()
    }
}
