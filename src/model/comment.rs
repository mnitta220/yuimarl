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
    //pub ticket: Option<NewsTicket>,
}

/*
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NewsEvent {
    ProjectMemberAdd = 1,    // プロジェクトメンバーに追加された
    ProjectRoleUpdate = 2,   // プロジェクトメンバーのロールを更新した
    ProjectMemberDelete = 3, // プロジェクトメンバーから削除された
    TicketMemberAdd = 4,     // チケットメンバーに追加された
    TicketMemberDelete = 5,  // チケットメンバーから削除された
    TicketUpdate = 6,        // チケットが更新された
    ProjectDelete = 7,       // プロジェクトが削除された
    None = 0,
}
*/

impl Comment {
    pub fn new(ticket_id: &str, uid: &str, user_name: &str, comment: &str) -> Comment {
        Comment {
            id: Uuid::now_v7().to_string(),
            ticket_id: ticket_id.to_string(),
            timestamp: Utc::now(),
            uid: uid.to_string(),
            user_name: user_name.to_string(),
            comment: comment.to_string(),
        }
    }

    pub async fn insert(
        comment: Comment,
        //event: NewsEvent,
        //project_id: &str,
        //project_name: &str,
        //ticket: Option<NewsTicket>,
        db: &FirestoreDb,
    ) -> Result<()> {
        //let ev = event as i32;
        //let now = Utc::now();
        match db
            .fluent()
            .insert()
            .into(&COLLECTION_NAME)
            .document_id(comment.id.clone())
            .object(&comment)
            .execute::<Comment>()
            .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(())
    }

    pub async fn get_comment_list(ticket_id: &str, db: &FirestoreDb) -> Result<Vec<Self>> {
        let object_stream: BoxStream<FirestoreResult<Comment>> = match db
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

        let comments: Vec<Comment> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(comments)
    }

    /*
    pub async fn delete(news_id: &str, uid: &str, db: &FirestoreDb) -> Result<()> {
        let news: Option<News> = match db
            .fluent()
            .select()
            .by_id_in(COLLECTION_NAME)
            .obj()
            .one(news_id)
            .await
        {
            Ok(ret) => ret,
            Err(e) => {
                tracing::error!("failed to connect firestore: {:?}", e);
                std::process::exit(0x0100);
            }
        };

        if let Some(news) = news {
            if news.uid != uid {
                return Err(anyhow::anyhow!("not allowed"));
            }
        }

        if let Err(e) = db
            .fluent()
            .delete()
            .from(&COLLECTION_NAME)
            .document_id(news_id)
            .execute()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }
    */
}
