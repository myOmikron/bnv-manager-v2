use std::fmt::Debug;
use std::str::FromStr;

use axum::async_trait;
use rorm::fields::types::Json;
use rorm::{delete, insert, query, update, FieldAccess, Model};
use time::{Duration, OffsetDateTime};
use tower_sessions::session::{Id, Record};
use tower_sessions::session_store::Error;
use tower_sessions::SessionStore;
use tracing::{error, instrument};

use crate::global::GLOBAL;
use crate::models::session::Session;

#[derive(Debug, Clone, Copy)]
pub struct DBStore;

impl DBStore {
    /// Run a deletion task to clear expired sessions
    pub fn run_deletion_task(&self, period: Duration) {
        tokio::spawn(session_deletion_task(period));
    }
}
#[instrument]
async fn session_deletion_task(period: Duration) {
    let mut interval = tokio::time::interval(period.unsigned_abs());
    loop {
        let db = &GLOBAL.db;
        let now = OffsetDateTime::now_utc();

        if let Err(err) = delete!(db, Session)
            .condition(Session::F.expiration_time.less_than(now))
            .await
        {
            error!("Error while delete expired sessions: {err}");
        }

        interval.tick().await;
    }
}

#[async_trait]
impl SessionStore for DBStore {
    async fn save(&self, session_record: &Record) -> tower_sessions::session_store::Result<()> {
        let mut tx = GLOBAL
            .db
            .start_transaction()
            .await
            .map_err(|err| Error::Backend(err.to_string()))?;

        let session_id = session_record.id.to_string();
        let session_data = session_record.data.clone();

        let existing = query!(&mut tx, (Session::F.session_id,))
            .condition(Session::F.session_id.equals(&session_id))
            .optional()
            .await
            .map_err(|err| Error::Backend(err.to_string()))?;

        if existing.is_some() {
            update!(&mut tx, Session)
                .condition(Session::F.session_id.equals(&session_id))
                .set(Session::F.data, Json(session_data))
                .set(Session::F.expiration_time, session_record.expiry_date)
                .exec()
                .await
                .map_err(|err| Error::Backend(err.to_string()))?;
        } else {
            insert!(&mut tx, Session)
                .return_nothing()
                .single(&Session {
                    session_id,
                    data: Json(session_data),
                    expiration_time: session_record.expiry_date,
                })
                .await
                .map_err(|err| Error::Backend(err.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|err| Error::Backend(err.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> tower_sessions::session_store::Result<Option<Record>> {
        let db = &GLOBAL.db;
        let session = query!(db, Session)
            .condition(Session::F.session_id.equals(session_id.to_string()))
            .optional()
            .await
            .map_err(|x| Error::Backend(x.to_string()))?;

        if let Some(session) = session {
            Ok(Some(Record {
                id: Id::from_str(&session.session_id)
                    .map_err(|err| Error::Decode(err.to_string()))?,
                data: session.data.into_inner(),
                expiry_date: session.expiration_time,
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> tower_sessions::session_store::Result<()> {
        let db = &GLOBAL.db;
        delete!(db, Session)
            .condition(Session::F.session_id.equals(session_id.to_string()))
            .await
            .map_err(|x| Error::Backend(x.to_string()))?;

        Ok(())
    }
}
