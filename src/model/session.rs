use super::project::{Project, ProjectMember};
use super::ticket::{Ticket, TicketMember};
use super::user::User;
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

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

    /// E2Eテストデータを削除する
    pub async fn delete_e2e_test(db: &FirestoreDb) -> Result<()> {
        let mut transaction = match db.begin_transaction().await {
            Ok(t) => t,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let sessions_stream: BoxStream<FirestoreResult<Session>> = match db
            .fluent()
            .select()
            .fields(paths!(Session::{id, uid, name, email, photo_url, e2e_test, created_at}))
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

        let sessions: Vec<Session> = match sessions_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for session in sessions {
            if let Err(e) = db
                .fluent()
                .delete()
                .from(COLLECTION_NAME)
                .document_id(&session.id)
                .add_to_transaction(&mut transaction)
            {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        }

        let users_stream: BoxStream<FirestoreResult<User>> = match db
            .fluent()
            .select()
            .fields(paths!(User::{uid, email, name, status, e2e_test, created_at, last_login}))
            .from(super::user::COLLECTION_NAME)
            .filter(|q| q.for_all([q.field(path!(User::e2e_test)).eq(true)]))
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let users: Vec<User> = match users_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for user in users {
            let projects_stream: BoxStream<FirestoreResult<Project>> = match db
                .fluent()
                .select()
                .fields(paths!(Project::{id, db_check, deleted}))
                .from(super::project::COLLECTION_NAME)
                .filter(|q| q.for_all([q.field(path!(Project::owner)).eq(user.uid.clone())]))
                .obj()
                .stream_query_with_errors()
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            let projects: Vec<Project> = match projects_stream.try_collect().await {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            for prj in projects {
                let project_members_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
                    .fluent()
                    .select()
                    .fields(paths!(ProjectMember::{id, project_id, uid}))
                    .from(super::project::COLLECTION_MEMBER)
                    .filter(|q| q.for_all([q.field(path!(ProjectMember::project_id)).eq(&prj.id)]))
                    .obj()
                    .stream_query_with_errors()
                    .await
                {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };

                let project_members: Vec<ProjectMember> =
                    match project_members_stream.try_collect().await {
                        Ok(s) => s,
                        Err(e) => {
                            return Err(anyhow::anyhow!(e.to_string()));
                        }
                    };

                for member in project_members {
                    if let Err(e) = db
                        .fluent()
                        .delete()
                        .from(super::project::COLLECTION_MEMBER)
                        .document_id(&member.id)
                        .add_to_transaction(&mut transaction)
                    {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                }

                let tickets_stream: BoxStream<FirestoreResult<Ticket>> = match db
                    .fluent()
                    .select()
                    .fields(paths!(Ticket::{id, project_id, progress, priority}))
                    .from(super::ticket::COLLECTION_NAME)
                    .filter(|q| q.for_all([q.field(path!(Ticket::project_id)).eq(&prj.id)]))
                    .obj()
                    .stream_query_with_errors()
                    .await
                {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };

                let tickets: Vec<Ticket> = match tickets_stream.try_collect().await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };

                for ticket in tickets {
                    let ticket_members_stream: BoxStream<FirestoreResult<TicketMember>> = match db
                        .fluent()
                        .select()
                        .fields(paths!(TicketMember::{id, ticket_id, project_id, uid, seq}))
                        .from(super::ticket::COLLECTION_MEMBER)
                        .filter(|q| {
                            q.for_all([q.field(path!(TicketMember::ticket_id)).eq(&ticket.id)])
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

                    let members: Vec<TicketMember> = match ticket_members_stream.try_collect().await
                    {
                        Ok(s) => s,
                        Err(e) => {
                            return Err(anyhow::anyhow!(e.to_string()));
                        }
                    };

                    for member in members {
                        if let Err(e) = db
                            .fluent()
                            .delete()
                            .from(super::ticket::COLLECTION_MEMBER)
                            .document_id(&member.id)
                            .add_to_transaction(&mut transaction)
                        {
                            return Err(anyhow::anyhow!(e.to_string()));
                        }
                    }

                    if let Err(e) = db
                        .fluent()
                        .delete()
                        .from(super::ticket::COLLECTION_NAME)
                        .document_id(&ticket.id)
                        .add_to_transaction(&mut transaction)
                    {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                }

                if let Err(e) = db
                    .fluent()
                    .delete()
                    .from(super::project::COLLECTION_NAME)
                    .document_id(&prj.id)
                    .add_to_transaction(&mut transaction)
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }

        if let Err(e) = transaction.commit().await {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }
}
