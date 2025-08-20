use std::time::Duration;

use galvyn::core::Module;
use galvyn::rorm::Database;
use tracing::Span;
use tracing::error;

use crate::models::invite::Invite;
use crate::utils::worker::Worker;

const GC_INTERVAL: Duration = Duration::from_secs(60 * 60);

pub struct GarbageCollectorWorker;

impl Worker for GarbageCollectorWorker {
    async fn run(self) {
        let mut timer = tokio::time::interval(GC_INTERVAL);

        loop {
            let span = tracing::info_span!("GarbageCollectorWorker::run");
            let _enter = span.enter();

            if let Err(err) = self.run_once(span.clone()).await {
                error!(?err, "GarbageCollectorWorker::run_once failed");
            }

            timer.tick().await;
        }
    }
}

impl GarbageCollectorWorker {
    async fn run_once(&self, _span: Span) -> anyhow::Result<()> {
        let mut tx = Database::global().start_transaction().await?;

        Invite::clear_expired(&mut tx).await?;

        tx.commit().await?;

        Ok(())
    }
}
