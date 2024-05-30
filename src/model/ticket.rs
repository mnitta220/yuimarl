use super::history::{History, HistoryEvent, MAX_HISTORY};
use super::project::Project;
use super::session::Session;
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;

pub const COLLECTION_NAME: &'static str = "ticket";
pub const COLLECTION_MEMBER: &'static str = "ticket_member";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub id: Option<String>,                // ID(uuid)
    pub project_id: Option<String>,        // プロジェクトID
    pub id_disp: Option<String>,           // 表示用チケットID（接頭辞＋連番）
    pub name: Option<String>,              // チケット名
    pub description: Option<String>,       // 内容
    pub start_date: Option<String>,        // 開始日
    pub end_date: Option<String>,          // 終了日
    pub progress: i32,                     // 進捗率
    pub priority: i32,                     // 優先度
    pub parent_id: Option<String>,         // 親チケットID
    pub deliverables: Option<String>,      // 成果物(JSON)
    pub owner: Option<String>,             // 登録ユーザー
    pub note: Option<String>,              // ノート（マークダウン）
    pub history: Option<String>,           // 更新履歴 (JSON)
    pub created_at: Option<DateTime<Utc>>, // 作成日時
    pub updated_at: Option<DateTime<Utc>>, // 更新日時
    pub deleted: bool,                     // 削除フラグ
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketMember {
    pub id: Option<String>,         // ID(uuid)
    pub ticket_id: Option<String>,  // チケットID
    pub project_id: Option<String>, // プロジェクトID
    pub uid: String,                // メンバーのユーザーID
    pub seq: i32,                   // 表示順
    pub name: Option<String>,       // メンバーの名前
    pub email: Option<String>,      // メンバーのメールアドレス
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            id: None,
            project_id: None,
            id_disp: None,
            name: None,
            description: None,
            start_date: None,
            end_date: None,
            progress: 0,
            priority: 0,
            parent_id: None,
            deliverables: None,
            owner: None,
            note: None,
            history: None,
            created_at: None,
            updated_at: None,
            deleted: false,
        }
    }

    pub async fn insert(
        input: &crate::handlers::ticket::TicketInput,
        session: &Session,
        project: &super::project::Project,
        members: &Vec<TicketMember>,
        db: &FirestoreDb,
    ) -> Result<Self> {
        let ticket_number = project.ticket_number.unwrap_or_default() + 1;
        let id_disp = format!(
            "{}{}",
            project.prefix.clone().unwrap_or_default(),
            ticket_number
        );
        let progress = match input.progress.parse::<i32>() {
            Ok(p) => p,
            Err(_) => 0,
        };
        let priority = match input.priority.parse::<i32>() {
            Ok(p) => p,
            Err(_) => 0,
        };

        let mut ticket = Ticket::new();
        let now = Utc::now();
        ticket.project_id = project.id.clone();
        ticket.id_disp = Some(id_disp);
        ticket.name = Some(input.name.clone());
        ticket.description = Some(input.description.clone());
        ticket.start_date = Some(input.start_date.clone());
        ticket.end_date = Some(input.end_date.clone());
        ticket.progress = progress;
        ticket.priority = priority;
        ticket.parent_id = Some(input.parent.clone());
        ticket.owner = Some(session.uid.clone());
        ticket.created_at = Some(now);
        ticket.updated_at = Some(now);

        let history = History {
            timestamp: now,
            uid: session.uid.clone(),
            user_name: session.name.clone(),
            event: HistoryEvent::TicketCreate as i32,
        };
        let history = vec![history];
        if let Ok(h) = serde_json::to_string(&history) {
            ticket.history = Some(h);
        }

        let mut count = 0u32;
        let mut id = Uuid::now_v7().to_string();

        loop {
            count += 1;
            if count > 9 {
                return Err(anyhow::anyhow!("Failed to create ticket".to_string()));
            }
            ticket.id = Some(id.clone());

            match db
                .fluent()
                .insert()
                .into(&COLLECTION_NAME)
                .document_id(id.clone())
                .object(&ticket)
                .execute::<Ticket>()
                .await
            {
                Ok(_) => {
                    break;
                }
                Err(e) => match &e {
                    firestore::errors::FirestoreError::DataConflictError(e) => {
                        tracing::error!("DataConflictError: {:?}", e);
                        id = Uuid::now_v7().to_string();
                        continue;
                    }
                    _ => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                },
            };
        }

        let mut seq = 0;
        for member in members {
            let mid = Uuid::now_v7().to_string();
            let mut member_new = TicketMember::new(member.uid.clone());
            member_new.id = Some(mid.clone());
            member_new.ticket_id = Some(id.clone());
            member_new.project_id = Some(project.id.clone().unwrap());
            member_new.seq = seq;

            match db
                .fluent()
                .insert()
                .into(&COLLECTION_MEMBER)
                .document_id(mid)
                .object(&member_new)
                .execute::<TicketMember>()
                .await
            {
                Ok(_) => {}
                Err(e) => return Err(anyhow::anyhow!(e.to_string())),
            };
            seq += 1;
        }

        let mut project = project.clone();
        project.ticket_number = Some(ticket_number);

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(super::project::Project::ticket_number))
            .in_col(&super::project::COLLECTION_NAME)
            .document_id(&project.id.clone().unwrap())
            .object(&project)
            .execute::<super::project::Project>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        tracing::debug!("Ticket inserted {:?}", ticket);

        Ok(ticket)
    }

    pub async fn update(
        input: &crate::handlers::ticket::TicketInput,
        session: &Session,
        members: &Vec<TicketMember>,
        db: &FirestoreDb,
    ) -> Result<()> {
        let progress = match input.progress.parse::<i32>() {
            Ok(p) => p,
            Err(_) => 0,
        };
        let priority = match input.priority.parse::<i32>() {
            Ok(p) => p,
            Err(_) => 0,
        };

        let mut ticket = Ticket::new();
        let now = Utc::now();
        ticket.name = Some(input.name.clone());
        if input.description.len() > 0 {
            ticket.description = Some(input.description.clone());
        }
        if input.start_date.len() > 0 {
            ticket.start_date = Some(input.start_date.clone());
        }
        if input.end_date.len() > 0 {
            ticket.end_date = Some(input.end_date.clone());
        }
        ticket.progress = progress;
        ticket.priority = priority;
        ticket.updated_at = Some(now);

        let history = History {
            timestamp: now,
            uid: session.uid.clone(),
            user_name: session.name.clone(),
            event: HistoryEvent::UpdateInfo as i32,
        };

        let mut histories = Vec::new();
        if let Some(h) = &ticket.history {
            let h: Vec<History> = match serde_json::from_str(&h) {
                Ok(h) => h,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };
            histories = h;
        }
        histories.push(history);

        loop {
            if histories.len() <= MAX_HISTORY {
                break;
            }
            histories.remove(0);
        }

        if let Ok(h) = serde_json::to_string(&histories) {
            ticket.history = Some(h);
        }

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Ticket::{name, description, start_date, end_date, progress, priority, updated_at, history}))
            .in_col(&COLLECTION_NAME)
            .document_id(&input.ticket_id)
            .object(&ticket)
            .execute::<Ticket>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        let current_members = match TicketMember::members_of_ticket(&input.ticket_id, db).await {
            Ok(m) => m,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        /*
         * チケットメンバーの更新処理
         * 現在のメンバーと更新後のメンバーをマッチングして、更新処理を行う。
         */
        let mut c = 0;
        let mut current = current_members.get(c);
        let mut u = 0;
        let mut upd = members.get(u);

        loop {
            if let Some(up) = upd {
                if let Some(cur) = current {
                    match up.seq.cmp(&cur.seq) {
                        Ordering::Less => {
                            if let Some(up) = upd {
                                let mut up = up.clone();
                                let id = Uuid::now_v7().to_string();
                                up.id = Some(id.clone());
                                up.project_id = Some(input.project_id.clone());
                                up.ticket_id = Some(input.ticket_id.clone());

                                if let Err(e) = db
                                    .fluent()
                                    .insert()
                                    .into(&COLLECTION_MEMBER)
                                    .document_id(id)
                                    .object(&up)
                                    .execute::<TicketMember>()
                                    .await
                                {
                                    return Err(anyhow::anyhow!(e.to_string()));
                                }
                            }
                            u += 1;
                            upd = members.get(u);
                        }
                        Ordering::Greater => {
                            if let Some(id) = &cur.id {
                                if let Err(e) = db
                                    .fluent()
                                    .delete()
                                    .from(&COLLECTION_MEMBER)
                                    .document_id(id)
                                    .execute()
                                    .await
                                {
                                    return Err(anyhow::anyhow!(e.to_string()));
                                }
                            }
                            c += 1;
                            current = current_members.get(c);
                        }
                        Ordering::Equal => {
                            if cur.uid != up.uid {
                                let mut cur = cur.clone();
                                cur.uid = up.uid.clone();
                                if let Err(e) = db
                                    .fluent()
                                    .update()
                                    .fields(paths!(TicketMember::uid))
                                    .in_col(&COLLECTION_MEMBER)
                                    .document_id(&cur.id.as_ref().unwrap())
                                    .object(&cur)
                                    .execute::<TicketMember>()
                                    .await
                                {
                                    return Err(anyhow::anyhow!(e.to_string()));
                                }
                            }

                            c += 1;
                            current = current_members.get(c);
                            u += 1;
                            upd = members.get(u);
                        }
                    }
                } else {
                    let mut up = up.clone();
                    let id = Uuid::now_v7().to_string();
                    up.id = Some(id.clone());
                    up.project_id = Some(input.project_id.clone());
                    up.ticket_id = Some(input.ticket_id.clone());

                    if let Err(e) = db
                        .fluent()
                        .insert()
                        .into(&COLLECTION_MEMBER)
                        .document_id(id)
                        .object(&up)
                        .execute::<TicketMember>()
                        .await
                    {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }

                    u += 1;
                    upd = members.get(u);
                }
            } else {
                if let Some(cur) = current {
                    if let Some(id) = &cur.id {
                        if let Err(e) = db
                            .fluent()
                            .delete()
                            .from(&COLLECTION_MEMBER)
                            .document_id(id)
                            .execute()
                            .await
                        {
                            return Err(anyhow::anyhow!(e.to_string()));
                        }
                    }
                } else {
                    break;
                }
                c += 1;
                current = current_members.get(c);
            }
        }

        tracing::debug!("Ticket updated {:?}", ticket);

        Ok(())
    }

    pub async fn find_current_tickets(
        project_id: &str,
        uid: &str,
        db: &FirestoreDb,
    ) -> Result<Vec<Ticket>> {
        let object_stream: BoxStream<FirestoreResult<TicketMember>> = match db
            .fluent()
            .select()
            .fields(paths!(TicketMember::{id, project_id, uid, ticket_id, seq}))
            .from(COLLECTION_MEMBER)
            .filter(|q| {
                q.for_all([
                    q.field(path!(TicketMember::project_id)).eq(&project_id),
                    q.field(path!(TicketMember::uid)).eq(&uid),
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

        let ticket_members: Vec<TicketMember> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut tickets = vec![];

        for member in ticket_members {
            match db
                .fluent()
                .select()
                .by_id_in(&COLLECTION_NAME)
                .obj::<Ticket>()
                .one(&member.ticket_id.clone().unwrap_or_default())
                .await
            {
                Ok(t) => match t {
                    Some(t) => {
                        tickets.push(t);
                    }
                    None => {
                        continue;
                    }
                },
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };
        }

        Ok(tickets)
    }

    pub async fn find_ticket_and_project(
        id: &str,
        db: &FirestoreDb,
    ) -> Result<(
        Option<Ticket>,
        Option<Project>,
        Vec<TicketMember>,
        Option<Ticket>,
    )> {
        let ticket: Option<Ticket> = match db
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

        let mut project: Option<Project> = None;
        if let Some(t) = &ticket {
            if let Some(project_id) = &t.project_id {
                project = super::project::Project::find(&project_id, db).await?;
            }
        }

        let mut ticket_members: Vec<TicketMember> = Vec::new();
        let mut parent: Option<Ticket> = None;
        if let Some(t) = &ticket {
            let object_stream: BoxStream<FirestoreResult<TicketMember>> = match db
                .fluent()
                .select()
                .fields(paths!(TicketMember::{id, project_id, uid, ticket_id, seq}))
                .from(COLLECTION_MEMBER)
                .filter(|q| q.for_all([q.field(path!(TicketMember::ticket_id)).eq(&t.id)]))
                .order_by([(path!(TicketMember::seq), FirestoreQueryDirection::Ascending)])
                .obj()
                .stream_query_with_errors()
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            ticket_members = match object_stream.try_collect().await {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            if let Some(p) = &t.parent_id {
                parent = match db
                    .fluent()
                    .select()
                    .by_id_in(COLLECTION_NAME)
                    .obj()
                    .one(&p)
                    .await
                {
                    Ok(ret) => ret,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };
            }
        }
        for member in ticket_members.iter_mut() {
            let user = match super::user::User::find(&member.uid, db).await {
                Ok(u) => u,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };
            if let Some(u) = user {
                member.name = Some(u.name);
                member.email = Some(u.email);
            }
        }

        tracing::debug!("Get by id {:?}", ticket);

        Ok((ticket, project, ticket_members, parent))
    }

    pub async fn search_by_id_disp(
        project_id: &str,
        id_disp: &str,
        db: &FirestoreDb,
    ) -> Result<Option<Ticket>> {
        let object_stream: BoxStream<FirestoreResult<Ticket>> = match db
            .fluent()
            .select()
            .fields(paths!(Ticket::{id, project_id, id_disp, name, progress, priority, deleted}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Ticket::project_id)).eq(&project_id),
                    q.field(path!(Ticket::id_disp)).eq(&id_disp.trim()),
                    q.field(path!(Ticket::deleted)).eq(false),
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

        let tickets: Vec<Ticket> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        if tickets.len() == 0 {
            return Ok(None);
        }

        Ok(tickets.get(0).cloned())
    }
}

impl TicketMember {
    pub fn new(uid: String) -> Self {
        Self {
            id: None,
            ticket_id: None,
            project_id: None,
            uid,
            seq: 0,
            name: None,
            email: None,
        }
    }

    pub async fn members_of_ticket(ticket_id: &str, db: &FirestoreDb) -> Result<Vec<TicketMember>> {
        let object_stream: BoxStream<FirestoreResult<TicketMember>> = match db
            .fluent()
            .select()
            .fields(paths!(TicketMember::{id, project_id, uid, ticket_id, seq}))
            .from(COLLECTION_MEMBER)
            .filter(|q| q.for_all([q.field(path!(TicketMember::ticket_id)).eq(&ticket_id)]))
            .order_by([(path!(TicketMember::seq), FirestoreQueryDirection::Ascending)])
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let ticket_members: Vec<TicketMember> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(ticket_members)
    }
}
