use super::ticket::Ticket;
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const COLLECTION_NAME: &'static str = "comment";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub id: String,
    pub ticket_id: String,
    pub timestamp: DateTime<Utc>,
    pub uid: String,
    pub user_name: String,
    pub comment: String,
    pub updated: bool,
}

impl Comment {
    pub fn new(ticket_id: &str, uid: &str, user_name: &str, comment: &str) -> Comment {
        Comment {
            id: Uuid::now_v7().to_string(),
            ticket_id: ticket_id.to_string(),
            timestamp: Utc::now(),
            uid: uid.to_string(),
            user_name: user_name.to_string(),
            comment: comment.to_string(),
            updated: false,
        }
    }

    pub async fn find(id: &str, db: &FirestoreDb) -> Result<Option<Self>> {
        let obj_by_id: Option<Comment> = match db
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

    pub async fn insert(comment: &Comment, ticket: &Ticket, db: &FirestoreDb) -> Result<()> {
        match db
            .fluent()
            .insert()
            .into(&COLLECTION_NAME)
            .document_id(comment.id.clone())
            .object(comment)
            .execute::<Comment>()
            .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let members =
            match super::ticket::TicketMember::members_of_ticket(&comment.ticket_id, &db).await {
                Ok(m) => m,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };
        for member in members {
            if member.uid == comment.uid {
                continue;
            }

            let news_ticket = super::news::NewsTicket {
                id: comment.ticket_id.clone(),
                id_disp: ticket.id_disp.clone().unwrap_or_default(),
                name: ticket.name.clone().unwrap_or_default(),
            };

            if let Err(e) = super::news::News::upsert(
                &member.uid,
                super::news::NewsEvent::TicketCommentAdd,
                &ticket.project_id,
                "",
                Some(news_ticket),
                None,
                None,
                None,
                &db,
            )
            .await
            {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        }

        Ok(())
    }

    pub async fn get_comment_list(ticket_id: &str, db: &FirestoreDb) -> Result<Vec<Self>> {
        let comments_stream: BoxStream<FirestoreResult<Comment>> = match db
            .fluent()
            .select()
            .from(COLLECTION_NAME)
            .filter(|q| q.for_all([q.field(path!(Comment::ticket_id)).eq(ticket_id)]))
            .order_by([(
                path!(Comment::timestamp),
                FirestoreQueryDirection::Ascending,
            )])
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let comments: Vec<Comment> = match comments_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(comments)
    }

    pub async fn update(comment: &Comment, db: &FirestoreDb) -> Result<()> {
        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Comment::{comment, updated}))
            .in_col(&COLLECTION_NAME)
            .document_id(&comment.id)
            .object(comment)
            .execute::<Comment>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }

    pub async fn delete(comment_id: &str, db: &FirestoreDb) -> Result<()> {
        if let Err(e) = db
            .fluent()
            .delete()
            .from(&COLLECTION_NAME)
            .document_id(comment_id)
            .execute()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }
}
