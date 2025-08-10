//! Utility struct to create links that can be used externally.

use url::Url;

use crate::config::ORIGIN;
use crate::models::invite::InviteUuid;

/// Create links using this struct
pub struct Link;

impl Link {
    /// Create a link for an invitation
    pub fn invite(InviteUuid(invite_uuid): InviteUuid) -> Url {
        #[allow(clippy::expect_used)]
        ORIGIN
            .join(&format!("/links/invite/{invite_uuid}"))
            .expect("UUID in urls are fine")
    }
}
