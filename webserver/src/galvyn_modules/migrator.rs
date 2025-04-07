//! Database migrations that must be run within the rust context

use galvyn::core::InitError;
use galvyn::core::PreInitError;
use galvyn::rorm::Database;

use crate::models::role::Role;
use crate::models::role::ROLES;

/// Database migrator
pub struct Migrator;

impl galvyn::core::Module for Migrator {
    type Setup = ();
    type PreInit = ();

    async fn pre_init(_setup: Self::Setup) -> Result<Self::PreInit, PreInitError> {
        Ok(())
    }

    type Dependencies = (Database,);

    async fn init(
        _pre_init: Self::PreInit,
        dependencies: &mut Self::Dependencies,
    ) -> Result<Self, InitError> {
        let mut tx = dependencies.0.start_transaction().await?;

        // Check if all roles are initialized
        for role in ROLES {
            let existing = rorm::query(&mut tx, Role)
                .condition(Role.name.equals(role))
                .optional()
                .await?;

            if existing.is_none() {
                rorm::insert(&mut tx, Role)
                    .return_nothing()
                    .single(&Role {
                        name: role.to_string(),
                    })
                    .await?;
            }
        }

        tx.commit().await?;

        Ok(Self)
    }
}
