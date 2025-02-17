use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use serde::{Deserialize, Serialize};

pub const COLLECTION_NAME: &'static str = "session";

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
}
