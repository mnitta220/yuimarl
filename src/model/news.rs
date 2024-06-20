use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const COLLECTION_NAME: &'static str = "news";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct News {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub uid: String,
    pub event: i32,
    pub project_id: String,
    pub project_name: String,
    pub ticket: Option<NewsTicket>,
    pub member_name: Option<String>,
    pub notice_id: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewsTicket {
    pub id: String,
    pub id_disp: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum NewsEvent {
    ProjectMemberAdd = 1,      // プロジェクトメンバーに追加された
    ProjectRoleUpdate = 2,     // プロジェクトメンバーのロールを更新した
    ProjectMemberDelete = 3,   // プロジェクトメンバーから削除された
    TicketMemberAdd = 4,       // チケットメンバーに追加された
    TicketMemberDelete = 5,    // チケットメンバーから削除された
    TicketUpdate = 6,          // チケットが更新された
    ProjectDelete = 7,         // プロジェクトが削除された
    TicketCommentAdd = 8,      // チケットのコメントが追加された
    ProjectMemberWithdraw = 9, // プロジェクトメンバーが離脱した
    OperationNotice = 10,      // 運用からのお知らせ
    TicketNoteUpdate = 11,     // チケットのノートが更新された
    None = 0,
}

impl News {
    pub fn num_to_event(&self) -> NewsEvent {
        match self.event {
            1 => NewsEvent::ProjectMemberAdd,
            2 => NewsEvent::ProjectRoleUpdate,
            3 => NewsEvent::ProjectMemberDelete,
            4 => NewsEvent::TicketMemberAdd,
            5 => NewsEvent::TicketMemberDelete,
            6 => NewsEvent::TicketUpdate,
            7 => NewsEvent::ProjectDelete,
            8 => NewsEvent::TicketCommentAdd,
            9 => NewsEvent::ProjectMemberWithdraw,
            10 => NewsEvent::OperationNotice,
            11 => NewsEvent::TicketNoteUpdate,
            _ => NewsEvent::None,
        }
    }

    pub async fn upsert(
        uid: &str,
        event: NewsEvent,
        project_id: &str,
        project_name: &str,
        ticket: Option<NewsTicket>,
        member_name: Option<String>,
        notice_id: Option<String>,
        message: Option<String>,
        db: &FirestoreDb,
    ) -> Result<()> {
        let ev = event as i32;
        let now = Utc::now();

        if let Some(ref t) = ticket {
            // 同じユーザーに１つのチケットのイベントは１つのみにする
            let object_stream: BoxStream<FirestoreResult<News>> = match db
                .fluent()
                .select()
                .from(COLLECTION_NAME)
                .filter(|q| q.for_all([q.field(path!(News::uid)).eq(uid)]))
                .order_by([(path!(News::timestamp), FirestoreQueryDirection::Ascending)])
                .obj()
                .stream_query_with_errors()
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            let news: Vec<News> = match object_stream.try_collect().await {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            for mut n in news {
                if n.event == ev {
                    if let Some(tkt) = &n.ticket {
                        if tkt.id == t.id {
                            n.timestamp = now;

                            if let Err(e) = db
                                .fluent()
                                .update()
                                .in_col(&COLLECTION_NAME)
                                .document_id(&n.id)
                                .object(&n)
                                .execute::<News>()
                                .await
                            {
                                return Err(anyhow::anyhow!(e.to_string()));
                            };

                            return Ok(());
                        }
                    }
                }
            }
        }

        let news = News {
            id: Uuid::now_v7().to_string(),
            timestamp: now,
            uid: uid.to_string(),
            event: ev,
            project_id: project_id.to_string(),
            project_name: project_name.to_string(),
            ticket,
            member_name,
            notice_id,
            message,
        };

        if let Err(e) = db
            .fluent()
            .update()
            .in_col(&COLLECTION_NAME)
            .document_id(&news.id)
            .object(&news)
            .execute::<News>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        };

        Ok(())
    }

    pub async fn get_news_list(uid: &String, db: &FirestoreDb) -> Result<Vec<Self>> {
        let object_stream: BoxStream<FirestoreResult<News>> = match db
            .fluent()
            .select()
            .fields(paths!(News::{id, timestamp, uid, event, project_id, project_name, ticket, member_name, notice_id, message}))
            .from(COLLECTION_NAME)
            .filter(|q| q.for_all([q.field(path!(News::uid)).eq(uid)]))
            .order_by([(path!(News::timestamp), FirestoreQueryDirection::Ascending)])
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let news: Vec<News> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(news)
    }

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

    pub async fn add_operation_notice(message: &str, db: &FirestoreDb) -> Result<String> {
        let notice_id = Uuid::now_v7().to_string();
        let now = Utc::now();

        let object_stream: BoxStream<FirestoreResult<super::user::User>> = match db
            .fluent()
            .select()
            .fields(paths!(super::user::User::{uid, email, name, status, created_at, last_login}))
            .from(super::user::COLLECTION_NAME)
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut news = News {
            id: Uuid::now_v7().to_string(),
            timestamp: now,
            uid: "".to_string(),
            event: NewsEvent::OperationNotice as i32,
            project_id: "".to_string(),
            project_name: "".to_string(),
            ticket: None,
            member_name: None,
            notice_id: Some(notice_id.clone()),
            message: Some(message.to_string()),
        };

        let users: Vec<super::user::User> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for user in users {
            if user.status != super::user::UserStatus::Approved as i32 {
                continue;
            }
            news.uid = user.uid;

            if let Err(e) = db
                .fluent()
                .insert()
                .into(&COLLECTION_NAME)
                .document_id(&news.id)
                .object(&news)
                .execute::<News>()
                .await
            {
                return Err(anyhow::anyhow!(e.to_string()));
            };

            news.id = Uuid::now_v7().to_string();
        }

        Ok(notice_id)
    }

    pub async fn del_operation_notice(notice_id: &str, db: &FirestoreDb) -> Result<()> {
        let object_stream: BoxStream<FirestoreResult<News>> = match db
            .fluent()
            .select()
            .fields(paths!(News::{id, timestamp, uid, event, project_id, project_name}))
            .from(COLLECTION_NAME)
            .filter(|q| q.for_all([q.field(path!(News::notice_id)).eq(notice_id)]))
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let news_list: Vec<News> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for news in news_list {
            if let Err(e) = db
                .fluent()
                .delete()
                .from(&COLLECTION_NAME)
                .document_id(news.id)
                .execute()
                .await
            {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        }

        Ok(())
    }
}
