use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::{stream::BoxStream, TryStreamExt};
use serde::{Deserialize, Serialize};

use super::user::User;

const COLLECTION_NAME: &'static str = "session";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: String,
    pub uid: String,
    pub name: String,
    pub email: String,
    pub photo_url: String,
    pub e2e_test: Option<bool>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub async fn find(id: &str, db: &FirestoreDb) -> Result<Option<Self>> {
        let obj_by_id: Option<Session> = match db
            .fluent()
            .select()
            .by_id_in(COLLECTION_NAME)
            .obj()
            .one(id)
            .await
        {
            Ok(ret) => ret,
            Err(e) => {
                tracing::error!("failed to connect firestore: {:?}", e);
                std::process::exit(0x0100);
            }
        };

        tracing::debug!("Get by id {:?}", obj_by_id);

        Ok(obj_by_id)
    }

    pub async fn upsert(session: &Session, db: &FirestoreDb) -> Result<()> {
        let obj: Option<Session> = match db
            .fluent()
            .update()
            .in_col(&COLLECTION_NAME)
            .document_id(&session.id)
            .object(session)
            .execute()
            .await
        {
            Ok(ret) => ret,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        tracing::debug!("Session upserted {:?}", obj);

        Ok(())
    }

    pub async fn update_name(session: &mut Session, name: &str, db: &FirestoreDb) -> Result<()> {
        session.name = name.to_string();

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Session::name))
            .in_col(&COLLECTION_NAME)
            .document_id(&session.id)
            .object(session)
            .execute::<Session>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }

    pub async fn e2e_test_session(db: &FirestoreDb, session_id: &str, user: &User) -> Result<Self> {
        let object_stream: BoxStream<FirestoreResult<Session>> = match db
            .fluent()
            .select()
            .from(COLLECTION_NAME)
            .filter(|q| q.for_all([q.field(path!(Session::e2e_test)).eq(true)]))
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let sessions: Vec<Session> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        if sessions.len() > 0 {
            return Ok(sessions[0].clone());
        }

        let session = Session {
            id: session_id.to_string(),
            uid: user.uid.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            photo_url: "".to_string(),
            e2e_test: Some(true),
            created_at: Utc::now(),
        };

        if let Err(e) = Session::upsert(&session, db).await {
            return Err(anyhow::anyhow!(e));
        }

        Ok(session)
    }
}
