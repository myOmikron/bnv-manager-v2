//! Globals are defined in this module

use std::ops::Deref;
use std::sync::OnceLock;

use rorm::Database;

use crate::modules::ldap::LdapConn;

/// Set of global managers and handles
pub static GLOBAL: GlobalOnceCell = GlobalOnceCell::new();

/// The global available parameters
pub struct Globals {
    /// The database handle
    pub db: Database,
    /// The ldap connection
    pub ldap: LdapConn,
}

/// Simple [`OnceLock`] which panics in case of error.
pub struct GlobalOnceCell(OnceLock<Globals>);
impl GlobalOnceCell {
    /// Creates a new empty cell
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    /// Initialise the cell
    ///
    /// ## Panics
    /// If called twice
    pub fn init(&self, value: Globals) {
        self.0
            .set(value)
            .ok()
            .expect("`GlobalLock.init` has been called twice")
    }
}
impl Deref for GlobalOnceCell {
    type Target = Globals;

    /// Retrieved the initialised value
    ///
    /// ## Panics
    /// If called before [`GlobalOnceCell::init`]
    fn deref(&self) -> &Self::Target {
        #[allow(clippy::expect_used)]
        self.0
            .get()
            .expect("`GlobalLock.init` has not been called yet. Please open an issues.")
    }
}
