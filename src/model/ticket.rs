use super::history::{History, HistoryEvent, MAX_HISTORY};
use super::project::{Project, ProjectMember};
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
    pub id: String,                        // ID(uuid)
    pub project_id: String,                // プロジェクトID
    pub id_disp: Option<String>,           // 表示用チケットID（接頭辞＋連番）
    pub name: Option<String>,              // チケット名
    pub description: Option<String>,       // 内容
    pub start_date: Option<String>,        // 開始日
    pub end_date: Option<String>,          // 終了日
    pub progress: i32,                     // 進捗率
    pub priority: i32,                     // 優先度
    pub parent_id: Option<String>,         // 親チケットID
    pub parent_id_disp: Option<String>,    // 親チケット表示用チケットID
    pub parent_name: Option<String>,       // 親チケット名
    pub deliverables: Option<String>,      // 成果物(JSON)
    pub owner: Option<String>,             // 登録ユーザー
    pub note: Option<String>,              // ノート（マークダウン）
    pub history: Option<String>,           // 更新履歴 (JSON)
    pub created_at: Option<DateTime<Utc>>, // 作成日時
    pub updated_at: Option<DateTime<Utc>>, // 更新日時
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketMember {
    pub id: String,                       // ID(uuid)
    pub ticket_id: String,                // チケットID
    pub project_id: String,               // プロジェクトID
    pub uid: String,                      // メンバーのユーザーID
    pub seq: i32,                         // 表示順
    pub name: Option<String>,             // メンバーの名前
    pub email: Option<String>,            // メンバーのメールアドレス
    pub last_used: Option<DateTime<Utc>>, // 最終使用日時
}

impl Ticket {
    pub fn new(ticket_id: &str, project_id: &str) -> Self {
        Self {
            id: ticket_id.to_string(),
            project_id: project_id.to_string(),
            id_disp: None,
            name: None,
            description: None,
            start_date: None,
            end_date: None,
            progress: 0,
            priority: 0,
            parent_id: None,
            parent_id_disp: None,
            parent_name: None,
            deliverables: None,
            owner: None,
            note: None,
            history: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn priority_to_string(&self) -> String {
        match self.priority {
            4 => "最優先".to_string(),
            3 => "高".to_string(),
            2 => "中".to_string(),
            1 => "低".to_string(),
            _ => "".to_string(),
        }
    }

    pub async fn find(id: &str, db: &FirestoreDb) -> Result<Option<Self>> {
        let obj_by_id: Option<Ticket> = match db
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

        let mut id = Uuid::now_v7().to_string();
        let mut ticket = Ticket::new(&id, &project.id);
        let now = Utc::now();
        ticket.project_id = project.id.clone();
        ticket.id_disp = Some(id_disp.clone());
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

        if input.parent.len() > 0 {
            ticket.parent_id = Some(input.parent.clone());
        }

        if input.deliverables.len() > 0 {
            ticket.deliverables = Some(input.deliverables.clone());
        }

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

        loop {
            count += 1;
            if count > 9 {
                return Err(anyhow::anyhow!("Failed to create ticket".to_string()));
            }

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
                        ticket.id = id.clone();
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
            let mut member_new = TicketMember::new(&mid, &id, &project.id, &member.uid);
            member_new.seq = seq;
            member_new.last_used = Some(now);

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

            if member.uid != session.uid {
                let news_ticket = super::news::NewsTicket {
                    id: id.clone(),
                    id_disp: id_disp.clone(),
                    name: input.name.clone(),
                };

                if let Err(e) = super::news::News::upsert(
                    &member.uid,
                    super::news::NewsEvent::TicketMemberAdd,
                    &id,
                    &project.project_name.clone().unwrap_or_default(),
                    Some(news_ticket),
                    None,
                    &db,
                )
                .await
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }

            seq += 1;
        }

        let mut project = project.clone();
        project.ticket_number = Some(ticket_number);

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(super::project::Project::ticket_number))
            .in_col(&super::project::COLLECTION_NAME)
            .document_id(&project.id)
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
        let ticket = match Self::find(&input.ticket_id, db).await {
            Ok(t) => t,
            Err(e) => return Err(anyhow::anyhow!(e.to_string())),
        };

        let mut ticket = match ticket {
            Some(t) => t,
            None => {
                return Err(anyhow::anyhow!("Ticket not found".to_string()));
            }
        };

        let progress = match input.progress.parse::<i32>() {
            Ok(p) => p,
            Err(_) => 0,
        };
        let priority = match input.priority.parse::<i32>() {
            Ok(p) => p,
            Err(_) => 0,
        };

        let now = Utc::now();
        ticket.name = Some(input.name.clone());

        if input.description.len() > 0 {
            ticket.description = Some(input.description.clone());
        } else {
            ticket.description = None;
        }

        if input.start_date.len() > 0 {
            ticket.start_date = Some(input.start_date.clone());
        } else {
            ticket.start_date = None;
        }

        if input.end_date.len() > 0 {
            ticket.end_date = Some(input.end_date.clone());
        } else {
            ticket.end_date = None;
        }

        ticket.progress = progress;
        ticket.priority = priority;

        if input.deliverables.len() > 0 {
            ticket.deliverables = Some(input.deliverables.clone());
        } else {
            ticket.deliverables = None;
        }

        if input.parent.len() > 0 {
            ticket.parent_id = Some(input.parent.clone());
        } else {
            ticket.parent_id = None;
        }

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
            .fields(paths!(Ticket::{name, description, start_date, end_date, progress, priority, deliverables, parent_id, updated_at, history}))
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
                                up.id = id.clone();
                                up.project_id = input.project_id.clone();
                                up.ticket_id = input.ticket_id.clone();
                                up.last_used = Some(now);

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
                            if let Err(e) = db
                                .fluent()
                                .delete()
                                .from(&COLLECTION_MEMBER)
                                .document_id(&cur.id)
                                .execute()
                                .await
                            {
                                return Err(anyhow::anyhow!(e.to_string()));
                            }

                            c += 1;
                            current = current_members.get(c);
                        }

                        Ordering::Equal => {
                            if cur.uid == up.uid {
                                if cur.uid == session.uid {
                                    let mut cur = cur.clone();
                                    cur.last_used = Some(now);
                                    if let Err(e) = db
                                        .fluent()
                                        .update()
                                        .fields(paths!(TicketMember::last_used))
                                        .in_col(&COLLECTION_MEMBER)
                                        .document_id(&cur.id)
                                        .object(&cur)
                                        .execute::<TicketMember>()
                                        .await
                                    {
                                        return Err(anyhow::anyhow!(e.to_string()));
                                    }
                                }
                            } else {
                                let mut cur = cur.clone();
                                cur.uid = up.uid.clone();
                                if let Err(e) = db
                                    .fluent()
                                    .update()
                                    .fields(paths!(TicketMember::uid))
                                    .in_col(&COLLECTION_MEMBER)
                                    .document_id(&cur.id)
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
                    up.id = id.clone();
                    up.project_id = input.project_id.clone();
                    up.ticket_id = input.ticket_id.clone();
                    up.last_used = Some(now);

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
                    if let Err(e) = db
                        .fluent()
                        .delete()
                        .from(&COLLECTION_MEMBER)
                        .document_id(&cur.id)
                        .execute()
                        .await
                    {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                } else {
                    break;
                }

                c += 1;
                current = current_members.get(c);
            }
        }

        for upd in members {
            if upd.uid == session.uid {
                continue;
            }

            /*
            let news_ticket = super::news::NewsTicket {
                id: input.ticket_id.to_string(),
                id_disp: ticket.id_disp.clone().unwrap_or_default(),
                name: ticket.name.clone().unwrap_or_default(),
            };
            */

            let mut found = false;
            for cur in &current_members {
                if upd.uid == cur.uid {
                    found = true;
                    break;
                }
            }

            if found {
                let news_ticket = super::news::NewsTicket {
                    id: input.ticket_id.to_string(),
                    id_disp: ticket.id_disp.clone().unwrap_or_default(),
                    name: ticket.name.clone().unwrap_or_default(),
                };

                if let Err(e) = super::news::News::upsert(
                    &upd.uid,
                    super::news::NewsEvent::TicketUpdate,
                    &ticket.project_id,
                    "",
                    Some(news_ticket),
                    None,
                    &db,
                )
                .await
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            } else {
                let news_ticket = super::news::NewsTicket {
                    id: input.ticket_id.to_string(),
                    id_disp: ticket.id_disp.clone().unwrap_or_default(),
                    name: ticket.name.clone().unwrap_or_default(),
                };

                if let Err(e) = super::news::News::upsert(
                    &upd.uid,
                    super::news::NewsEvent::TicketMemberAdd,
                    &ticket.project_id,
                    "",
                    Some(news_ticket),
                    None,
                    &db,
                )
                .await
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }

        tracing::debug!("Ticket updated {:?}", ticket);

        Ok(())
    }

    pub async fn update_note(
        input: &crate::handlers::ticket::NoteInput,
        session: &Session,
        db: &FirestoreDb,
    ) -> Result<Ticket> {
        let ticket = match Ticket::find(&input.ticket_id, db).await {
            Ok(t) => t,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };
        let mut ticket = match ticket {
            Some(t) => t,
            None => {
                return Err(anyhow::anyhow!("Ticket not found".to_string()));
            }
        };
        let now = Utc::now();
        ticket.note = Some(input.markdown.trim().to_string());
        ticket.updated_at = Some(now);

        let history = History {
            timestamp: now,
            uid: session.uid.clone(),
            user_name: session.name.clone(),
            event: HistoryEvent::UpdateNote as i32,
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
            .fields(paths!(Ticket::{note, updated_at, history}))
            .in_col(&COLLECTION_NAME)
            .document_id(&input.ticket_id)
            .object(&ticket)
            .execute::<Ticket>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        match TicketMember::find_by_id_and_uid(&input.ticket_id, &session.uid, db).await {
            Ok(m) => {
                if let Some(mut m) = m {
                    m.last_used = Some(now);
                    if let Err(e) = db
                        .fluent()
                        .update()
                        .fields(paths!(TicketMember::last_used))
                        .in_col(&COLLECTION_MEMBER)
                        .document_id(&m.id)
                        .object(&m)
                        .execute::<TicketMember>()
                        .await
                    {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        }

        let members = match TicketMember::members_of_ticket(&input.ticket_id, db).await {
            Ok(m) => m,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for member in members {
            if member.uid == session.uid {
                continue;
            }

            let news_ticket = super::news::NewsTicket {
                id: input.ticket_id.to_string(),
                id_disp: ticket.id_disp.clone().unwrap_or_default(),
                name: ticket.name.clone().unwrap_or_default(),
            };

            if let Err(e) = super::news::News::upsert(
                &member.uid,
                super::news::NewsEvent::TicketUpdate,
                &ticket.project_id,
                "",
                Some(news_ticket),
                None,
                &db,
            )
            .await
            {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        }

        Ok(ticket)
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
            .order_by([(
                path!(TicketMember::last_used),
                FirestoreQueryDirection::Descending,
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
                .one(&member.ticket_id.clone())
                .await
            {
                Ok(t) => match t {
                    Some(t) => {
                        if t.progress != 100 {
                            tickets.push(t);
                        }
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

        tickets.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(tickets)
    }

    pub async fn find_ticket_and_project(
        ticket_id: &str,
        uid: &str,
        db: &FirestoreDb,
    ) -> Result<(
        Option<Ticket>,
        Option<Project>,
        Option<ProjectMember>,
        Vec<TicketMember>,
        Option<Ticket>,
        Vec<Ticket>,
    )> {
        let ticket: Option<Ticket> = match db
            .fluent()
            .select()
            .by_id_in(COLLECTION_NAME)
            .obj()
            .one(ticket_id)
            .await
        {
            Ok(ret) => ret,
            Err(e) => {
                tracing::error!("failed to connect firestore: {:?}", e);
                std::process::exit(0x0100);
            }
        };

        let mut project: Option<Project> = None;
        let mut project_member: Option<ProjectMember> = None;
        if let Some(t) = &ticket {
            match super::project::Project::find(&t.project_id, db).await {
                Ok(p) => {
                    project = p;
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
            project_member = super::project::ProjectMember::find(&t.project_id, uid, db).await?;
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

        let object_stream: BoxStream<FirestoreResult<Ticket>> = match db
            .fluent()
            .select()
            .fields(paths!(Ticket::{id, project_id, id_disp, name, progress, priority}))
            .from(COLLECTION_NAME)
            .filter(|q| q.for_all([q.field(path!(Ticket::parent_id)).eq(&ticket_id)]))
            .order_by([(path!(Ticket::id), FirestoreQueryDirection::Ascending)])
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let children: Vec<Ticket> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        tracing::debug!("Get by id {:?}", ticket);

        Ok((
            ticket,
            project,
            project_member,
            ticket_members,
            parent,
            children,
        ))
    }

    pub async fn search_by_id_disp(
        project_id: &str,
        id_disp: &str,
        db: &FirestoreDb,
    ) -> Result<Option<Ticket>> {
        let object_stream: BoxStream<FirestoreResult<Ticket>> = match db
            .fluent()
            .select()
            .fields(paths!(Ticket::{id, project_id, id_disp, name, progress, priority}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Ticket::project_id)).eq(&project_id),
                    q.field(path!(Ticket::id_disp)).eq(&id_disp.trim()),
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

    pub async fn search_list(
        project_id: &str,
        input: &super::super::handlers::ticket_list::TicketListInput,
        db: &FirestoreDb,
    ) -> Result<Vec<Ticket>> {
        let object_stream: BoxStream<FirestoreResult<Ticket>> = match db
            .fluent()
            .select()
            .fields(paths!(Ticket::{id, project_id, id_disp, name, progress, priority, start_date, end_date, parent_id}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Ticket::project_id)).eq(&project_id)
                ])
            })
            .order_by([
                (path!(Ticket::id), FirestoreQueryDirection::Ascending),
            ])
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

        let mut serched: Vec<Ticket> = Vec::new();

        for ticket in &tickets {
            if input.ticketid.len() > 0 {
                if let Some(id_disp) = &ticket.id_disp {
                    if id_disp != &input.ticketid {
                        continue;
                    }
                }
            }
            if input.ticketname.len() > 0 {
                if let Some(n) = &ticket.name {
                    if n.find(&input.ticketname).is_none() {
                        continue;
                    }
                }
            }
            if let Some(f) = &input.finished {
                if f != "on" && f != "true" {
                    if ticket.progress == 100 {
                        continue;
                    }
                }
            } else {
                if ticket.progress == 100 {
                    continue;
                }
            }

            let mut new_ticket = ticket.clone();
            if let Some(parent_id) = &ticket.parent_id {
                let parent = tickets.iter().find(|t| &t.id == parent_id);
                if let Some(p) = parent {
                    if input.parentid.len() > 0 {
                        if let Some(parent_id) = &p.id_disp {
                            if parent_id != &input.parentid {
                                continue;
                            }
                        }
                    }
                    new_ticket.parent_id_disp = p.id_disp.clone();
                    new_ticket.parent_name = p.name.clone();
                } else {
                    if input.parentid.len() > 0 {
                        continue;
                    }
                }
            } else {
                if input.parentid.len() > 0 {
                    continue;
                }
            }

            serched.push(new_ticket);
        }

        Ok(serched)
    }

    pub async fn delete(
        input: &crate::handlers::ticket::TicketInput,
        db: &FirestoreDb,
    ) -> Result<()> {
        let mut transaction = match db.begin_transaction().await {
            Ok(t) => t,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        if let Err(e) = db
            .fluent()
            .delete()
            .from(COLLECTION_NAME)
            .document_id(&input.ticket_id)
            .add_to_transaction(&mut transaction)
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        let object_stream: BoxStream<FirestoreResult<TicketMember>> = match db
            .fluent()
            .select()
            .fields(paths!(TicketMember::{id, ticket_id, uid, seq}))
            .from(COLLECTION_MEMBER)
            .filter(|q| q.for_all([q.field(path!(TicketMember::ticket_id)).eq(&input.ticket_id)]))
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let members: Vec<TicketMember> = match object_stream.try_collect().await {
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

        if let Err(e) = transaction.commit().await {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }
}

impl TicketMember {
    pub fn new(id: &str, ticket_id: &str, project_id: &str, uid: &str) -> Self {
        Self {
            id: id.to_string(),
            ticket_id: ticket_id.to_string(),
            project_id: project_id.to_string(),
            uid: uid.to_string(),
            seq: 0,
            name: None,
            email: None,
            last_used: None,
        }
    }

    pub async fn find_by_id_and_uid(
        ticket_id: &str,
        uid: &str,
        db: &FirestoreDb,
    ) -> Result<Option<TicketMember>> {
        let object_stream: BoxStream<FirestoreResult<TicketMember>> = match db
            .fluent()
            .select()
            .fields(paths!(TicketMember::{id, project_id, uid, ticket_id, seq}))
            .from(COLLECTION_MEMBER)
            .filter(|q| {
                q.for_all([
                    q.field(path!(TicketMember::ticket_id)).eq(&ticket_id),
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

        if let Some(ticket_member) = ticket_members.get(0) {
            return Ok(Some(ticket_member.clone()));
        }

        Ok(None)
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
