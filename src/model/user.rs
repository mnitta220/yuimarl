use super::session::Session;
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

pub const COLLECTION_NAME: &'static str = "user";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub name: String,
    pub photo_url: Option<String>,
    pub status: i32, // 0:未承認, 1:承認済, 2:禁止
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSub {
    pub uid: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserStatus {
    Unapproved = 0,
    Approved = 1,
    Ban = 2,
}

impl User {
    pub fn new() -> Self {
        Self {
            uid: String::new(),
            email: String::new(),
            name: String::new(),
            photo_url: None,
            status: UserStatus::Unapproved as i32,
            created_at: Utc::now(),
            last_login: Utc::now(),
        }
    }

    pub async fn find(uid: &str, db: &FirestoreDb) -> Result<Option<Self>> {
        let obj_by_id: Option<User> = match db
            .fluent()
            .select()
            .by_id_in(COLLECTION_NAME)
            .obj()
            .one(uid)
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

    pub async fn insert(session: &Session, db: &FirestoreDb) -> Result<()> {
        let user = User {
            uid: session.uid.clone(),
            email: session.email.clone(),
            name: session.name.clone(),
            photo_url: Some(session.photo_url.clone()),
            status: UserStatus::Approved as i32,
            created_at: Utc::now(),
            last_login: Utc::now(),
        };

        if let Err(e) = db
            .fluent()
            .insert()
            .into(&COLLECTION_NAME)
            .document_id(session.uid.clone())
            .object(&user)
            .execute::<User>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }

    pub fn to_sub(&self) -> UserSub {
        UserSub {
            uid: self.uid.clone(),
            email: self.email.clone(),
            name: self.name.clone(),
        }
    }

    pub async fn search_by_email(email: &String, db: &FirestoreDb) -> Result<Vec<UserSub>> {
        let object_stream: BoxStream<FirestoreResult<User>> = match db
            .fluent()
            .select()
            .fields(paths!(User::{uid, name, email, status, created_at, last_login}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(User::email)).eq(email),
                    q.field(path!(User::status)).eq(UserStatus::Approved as i32),
                ])
            })
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let users: Vec<User> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut subs = Vec::new();

        for user in users {
            subs.push(user.to_sub());
        }

        Ok(subs)
    }

    pub async fn search_by_name(name: &String, db: &FirestoreDb) -> Result<Vec<UserSub>> {
        let object_stream: BoxStream<FirestoreResult<User>> = match db
            .fluent()
            .select()
            .fields(paths!(User::{uid, name, email, status, created_at, last_login}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(User::name)).eq(name),
                    q.field(path!(User::status)).eq(UserStatus::Approved as i32),
                ])
            })
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let users: Vec<User> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut subs = Vec::new();

        for user in users {
            subs.push(user.to_sub());
        }

        Ok(subs)
    }

    pub async fn update_name(uid: &str, name: &str, db: &FirestoreDb) -> Result<()> {
        let mut user = User::new();
        user.name = name.to_string();

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(User::name))
            .in_col(&COLLECTION_NAME)
            .document_id(uid)
            .object(&user)
            .execute::<User>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }
}
