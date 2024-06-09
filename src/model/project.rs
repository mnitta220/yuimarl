use super::history::{History, HistoryEvent, MAX_HISTORY};
use super::session::Session;
use super::ticket::{Ticket, TicketMember};
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;

pub const COLLECTION_NAME: &'static str = "project";
const COLLECTION_MEMBER: &'static str = "project_member";
pub const MEMBER_LIMIT_DEFAULT: i32 = 20;
pub const TICKET_LIMIT_DEFAULT: i32 = 1000;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: String,                        // ID(uuid)
    pub project_name: Option<String>,      // プロジェクト名
    pub owner: Option<String>,             // プロジェクトオーナー
    pub prefix: Option<String>,            // チケット接頭辞
    pub member_limit: Option<i32>,         // 最大メンバー数
    pub ticket_limit: Option<i32>,         // 最大チケット番号
    pub ticket_number: Option<i32>,        // チケット番号
    pub note: Option<String>,              // ノート（マークダウン）
    pub created_at: Option<DateTime<Utc>>, // 作成日時
    pub updated_at: Option<DateTime<Utc>>, // 更新日時
    pub history: Option<String>,           // 更新履歴 (JSON)
    pub db_check: bool,                    // データベースチェック用
    pub deleted: bool,                     // 削除フラグ
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectMember {
    pub id: String,                       // ID(uuid)
    pub project_id: String,               // プロジェクトID
    pub uid: String,                      // メンバーのユーザーID
    pub role: Option<i32>,                // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
    pub name: Option<String>,             // メンバーの名前
    pub email: Option<String>,            // メンバーのメールアドレス
    pub last_used: Option<DateTime<Utc>>, // 最終使用日時
    pub project_name: Option<String>,     // プロジェクト名
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProjectRole {
    Owner = 1,
    Administrator = 2,
    Member = 3,
    Viewer = 4,
}

impl Project {
    pub fn new(id: &str) -> Self {
        Project {
            id: id.to_string(),
            project_name: None,
            owner: None,
            prefix: None,
            member_limit: None,
            ticket_limit: None,
            ticket_number: None,
            note: None,
            created_at: None,
            updated_at: None,
            history: None,
            db_check: false,
            deleted: false,
        }
    }

    pub async fn find(id: &str, db: &FirestoreDb) -> Result<Option<Self>> {
        let obj_by_id: Option<Project> = match db
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

    pub async fn current_project(
        session: &Session,
        db: &FirestoreDb,
    ) -> Result<(Option<Self>, Option<ProjectMember>)> {
        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, uid, role, last_used}))
            .from(COLLECTION_MEMBER)
            .filter(|q| q.for_all([q.field(path!(ProjectMember::uid)).eq(&session.uid)]))
            .order_by([(
                path!(ProjectMember::last_used),
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

        let project_members: Vec<ProjectMember> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut prj: Option<Project> = None;
        let mut project_member: Option<ProjectMember> = None;

        for member in project_members {
            //if let Some(ref project_id) = member.project_id {
            match db
                .fluent()
                .select()
                .by_id_in(&COLLECTION_NAME)
                .obj::<Project>()
                .one(&member.project_id)
                .await
            {
                Ok(p) => match p {
                    Some(p) => {
                        if p.deleted {
                            continue;
                        }
                        prj = Some(p);
                        project_member = Some(member);
                        break;
                    }
                    None => {
                        continue;
                    }
                },
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };
            //}
        }

        Ok((prj, project_member))
    }

    pub async fn current_project_and_tickets(
        session: &Session,
        db: &FirestoreDb,
    ) -> Result<(
        Option<Self>,
        Option<ProjectMember>,
        Vec<super::ticket::Ticket>,
    )> {
        let (project, member) = Self::current_project(&session, &db).await?;

        let mut tickets: Vec<super::ticket::Ticket> = Vec::new();

        if let Some(p) = &project {
            match super::ticket::Ticket::find_current_tickets(&p.id, &session.uid, &db).await {
                Ok(t) => tickets = t,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }

        Ok((project, member, tickets))
    }

    pub async fn find_by_owner_and_name(
        owner: &String,
        project_name: &String,
        db: &FirestoreDb,
    ) -> Result<Vec<Self>> {
        let object_stream: BoxStream<FirestoreResult<Project>> = match db
            .fluent()
            .select()
            .fields(paths!(Project::{id, project_name, owner, prefix, db_check, deleted}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Project::owner)).eq(owner),
                    q.field(path!(Project::project_name)).eq(project_name),
                    q.field(path!(Project::deleted)).eq(false),
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

        let projects: Vec<Project> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(projects)
    }

    // ログインユーザーがオーナーになっているプロジェクトの件数を取得する。
    pub async fn count_owner_projects(owner: &String, db: &FirestoreDb) -> Result<usize> {
        let object_stream: BoxStream<FirestoreResult<Project>> = match db
            .fluent()
            .select()
            .fields(paths!(Project::{id, db_check, deleted}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Project::owner)).eq(owner),
                    q.field(path!(Project::deleted)).eq(false),
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

        let projects: Vec<Project> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(projects.len())
    }

    pub async fn insert(
        input: &crate::handlers::project::ProjectInput,
        session: &Session,
        project_members: &mut Vec<ProjectMember>,
        db_check: bool,
        db: &FirestoreDb,
    ) -> Result<Project> {
        let mut id = Uuid::now_v7().to_string();
        let mut prj = Project::new(&id);
        let now = Utc::now();
        prj.project_name = Some(input.project_name.trim().to_string());
        prj.owner = Some(session.uid.clone());
        prj.prefix = Some(input.prefix.trim().to_string());
        prj.member_limit = Some(MEMBER_LIMIT_DEFAULT);
        prj.ticket_limit = Some(TICKET_LIMIT_DEFAULT);
        prj.ticket_number = Some(0);
        prj.created_at = Some(now);
        prj.updated_at = Some(now);
        prj.db_check = db_check;

        let history = History {
            timestamp: now,
            uid: session.uid.clone(),
            user_name: session.name.clone(),
            event: HistoryEvent::ProjectCreate as i32,
        };
        let history = vec![history];
        if let Ok(h) = serde_json::to_string(&history) {
            prj.history = Some(h);
        }

        let mut count = 0u32;

        loop {
            count += 1;
            if count > 9 {
                return Err(anyhow::anyhow!("Failed to create project".to_string()));
            }
            //prj.id = Some(id.clone());

            match db
                .fluent()
                .insert()
                .into(&COLLECTION_NAME)
                .document_id(id.clone())
                .object(&prj)
                .execute::<Project>()
                .await
            {
                Ok(_) => {
                    break;
                }
                Err(e) => match &e {
                    firestore::errors::FirestoreError::DataConflictError(e) => {
                        tracing::error!("DataConflictError: {:?}", e);
                        id = Uuid::now_v7().to_string();
                        prj.id = id.clone();
                        continue;
                    }
                    _ => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                },
            };
        }

        for member in project_members {
            member.project_id = id.clone();
            member.last_used = Some(Utc::now());
            let mut count = 0u32;

            loop {
                count += 1;
                if count > 9 {
                    return Err(anyhow::anyhow!(
                        "Failed to create project_member".to_string()
                    ));
                }
                let id = Uuid::now_v7().to_string();
                member.id = id.clone();

                match db
                    .fluent()
                    .insert()
                    .into(&COLLECTION_MEMBER)
                    .document_id(id)
                    .object(&member.clone())
                    .execute::<ProjectMember>()
                    .await
                {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => match &e {
                        firestore::errors::FirestoreError::DataConflictError(e) => {
                            tracing::error!("DataConflictError: {:?}", e);
                            continue;
                        }
                        _ => {
                            return Err(anyhow::anyhow!(e.to_string()));
                        }
                    },
                };
            }

            if member.uid != session.uid {
                if let Err(e) = super::news::News::upsert(
                    &member.uid,
                    super::news::NewsEvent::ProjectMemberAdd,
                    &id,
                    &input.project_name,
                    None,
                    &db,
                )
                .await
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }

        tracing::debug!("Project inserted {:?}", prj);

        Ok(prj)
    }

    pub async fn update(
        input: &crate::handlers::project::ProjectInput,
        session: &Session,
        project_members: &mut Vec<ProjectMember>,
        db: &FirestoreDb,
    ) -> Result<Project> {
        let prj = match Project::find(&input.project_id, db).await {
            Ok(p) => p,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut prj = match prj {
            Some(p) => p,
            None => {
                return Err(anyhow::anyhow!("Project not found".to_string()));
            }
        };

        let now = Utc::now();
        prj.project_name = Some(input.project_name.trim().to_string());
        prj.prefix = Some(input.prefix.trim().to_string());
        prj.updated_at = Some(now);

        let history = History {
            timestamp: now,
            uid: session.uid.clone(),
            user_name: session.name.clone(),
            event: HistoryEvent::UpdateInfo as i32,
        };

        let mut histories = Vec::new();
        if let Some(h) = &prj.history {
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
            prj.history = Some(h);
        }

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Project::{project_name, prefix, updated_at, history}))
            .in_col(&COLLECTION_NAME)
            .document_id(&input.project_id)
            .object(&prj)
            .execute::<Project>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        let current_members =
            match ProjectMember::members_of_project(&input.project_id, true, db).await {
                Ok(m) => m,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

        project_members.sort_by(|a, b| a.uid.cmp(&b.uid));

        /*
         * プロジェクトメンバーの更新処理
         * 現在のメンバーと更新後のメンバーをマッチングして、更新処理を行う。
         */
        let mut c = 0;
        let mut current = current_members.get(c);
        let mut u = 0;
        let mut upd = project_members.get(u);

        loop {
            if let Some(up) = upd {
                if let Some(cur) = current {
                    match up.uid.cmp(&cur.uid) {
                        Ordering::Less => {
                            if let Some(up) = upd {
                                let mut up = up.clone();
                                up.project_id = String::from(&input.project_id);
                                let id = Uuid::now_v7().to_string();
                                up.id = id.clone();
                                up.last_used = Some(Utc::now());

                                if let Err(e) = db
                                    .fluent()
                                    .insert()
                                    .into(&COLLECTION_MEMBER)
                                    .document_id(id)
                                    .object(&up)
                                    .execute::<ProjectMember>()
                                    .await
                                {
                                    return Err(anyhow::anyhow!(e.to_string()));
                                }

                                if up.uid != session.uid {
                                    if let Err(e) = super::news::News::upsert(
                                        &up.uid,
                                        super::news::NewsEvent::ProjectMemberAdd,
                                        &input.project_id,
                                        &input.project_name,
                                        None,
                                        &db,
                                    )
                                    .await
                                    {
                                        return Err(anyhow::anyhow!(e.to_string()));
                                    }
                                }
                            }

                            u += 1;
                            upd = project_members.get(u);
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

                            if cur.uid != session.uid {
                                if let Err(e) = super::news::News::upsert(
                                    &cur.uid,
                                    super::news::NewsEvent::ProjectMemberDelete,
                                    &input.project_id,
                                    &input.project_name,
                                    None,
                                    &db,
                                )
                                .await
                                {
                                    return Err(anyhow::anyhow!(e.to_string()));
                                }
                            }

                            c += 1;
                            current = current_members.get(c);
                        }

                        Ordering::Equal => {
                            if cur.email != up.email || cur.name != up.name || cur.role != up.role {
                                let mut cur = cur.clone();
                                cur.email = up.email.clone();
                                cur.name = up.name.clone();
                                cur.role = up.role;

                                if let Err(e) = db
                                    .fluent()
                                    .update()
                                    .fields(paths!(
                                        ProjectMember::email,
                                        ProjectMember::name,
                                        ProjectMember::role
                                    ))
                                    .in_col(&COLLECTION_MEMBER)
                                    .document_id(&cur.id)
                                    .object(&cur)
                                    .execute::<ProjectMember>()
                                    .await
                                {
                                    return Err(anyhow::anyhow!(e.to_string()));
                                }
                            }

                            c += 1;
                            current = current_members.get(c);
                            u += 1;
                            upd = project_members.get(u);
                        }
                    }
                } else {
                    let mut up = up.clone();
                    up.project_id = String::from(&input.project_id);
                    let id = Uuid::now_v7().to_string();
                    up.id = id.clone();
                    up.last_used = Some(Utc::now());

                    if let Err(e) = db
                        .fluent()
                        .insert()
                        .into(&COLLECTION_MEMBER)
                        .document_id(id)
                        .object(&up)
                        .execute::<ProjectMember>()
                        .await
                    {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }

                    if up.uid != session.uid {
                        if let Err(e) = super::news::News::upsert(
                            &up.uid,
                            super::news::NewsEvent::ProjectMemberAdd,
                            &input.project_id,
                            &input.project_name,
                            None,
                            &db,
                        )
                        .await
                        {
                            return Err(anyhow::anyhow!(e.to_string()));
                        }
                    }

                    u += 1;
                    upd = project_members.get(u);
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

                    if cur.uid != session.uid {
                        if let Err(e) = super::news::News::upsert(
                            &cur.uid,
                            super::news::NewsEvent::ProjectMemberDelete,
                            &input.project_id,
                            &input.project_name,
                            None,
                            &db,
                        )
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

        tracing::debug!("Project updated {:?}", prj);

        Ok(prj)
    }

    pub async fn update_note(
        input: &crate::handlers::project::NoteInput,
        session: &Session,
        db: &FirestoreDb,
    ) -> Result<Project> {
        let prj = match Project::find(&input.project_id, db).await {
            Ok(p) => p,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };
        let mut prj = match prj {
            Some(p) => p,
            None => {
                return Err(anyhow::anyhow!("Project not found".to_string()));
            }
        };
        let now = Utc::now();
        prj.note = Some(input.markdown.trim().to_string());
        prj.updated_at = Some(now);

        let history = History {
            timestamp: now,
            uid: session.uid.clone(),
            user_name: session.name.clone(),
            event: HistoryEvent::UpdateNote as i32,
        };

        let mut histories = Vec::new();
        if let Some(h) = &prj.history {
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
            prj.history = Some(h);
        }

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Project::{note, updated_at, history}))
            .in_col(&COLLECTION_NAME)
            .document_id(&input.project_id)
            .object(&prj)
            .execute::<Project>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(prj)
    }

    pub async fn delete(
        input: &crate::handlers::project::ProjectInput,
        session: &Session,
        db: &FirestoreDb,
    ) -> Result<()> {
        let prj = match Project::find(&input.project_id, db).await {
            Ok(p) => p,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };
        let mut prj = match prj {
            Some(p) => p,
            None => {
                return Ok(());
            }
        };
        if prj.deleted {
            return Ok(());
        }

        let now = Utc::now();
        prj.updated_at = Some(now);
        prj.deleted = true;

        let history = History {
            timestamp: now,
            uid: session.uid.clone(),
            user_name: session.name.clone(),
            event: HistoryEvent::UpdateInfo as i32,
        };

        let mut histories = Vec::new();
        if let Some(h) = &prj.history {
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
            prj.history = Some(h);
        }

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Project::{updated_at, history, deleted}))
            .in_col(&COLLECTION_NAME)
            .document_id(&input.project_id)
            .object(&prj)
            .execute::<Project>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        let members = match ProjectMember::members_of_project(&input.project_id, true, &db).await {
            Ok(m) => m,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for member in members {
            if member.uid != session.uid {
                if let Err(e) = super::news::News::upsert(
                    &member.uid,
                    super::news::NewsEvent::ProjectDelete,
                    &input.project_id,
                    &input.project_name,
                    None,
                    &db,
                )
                .await
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }

        tracing::debug!("Project deleted {:?}", prj);

        Ok(())
    }

    /// データベースチェックで作成されたドキュメントを削除する
    pub async fn delete_db_check(db: &FirestoreDb) -> Result<()> {
        let mut transaction = match db.begin_transaction().await {
            Ok(t) => t,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let object_stream: BoxStream<FirestoreResult<Project>> = match db
            .fluent()
            .select()
            .fields(paths!(Project::{id, db_check, deleted}))
            .from(COLLECTION_NAME)
            .filter(|q| q.for_all([q.field(path!(Project::db_check)).eq(true)]))
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let projects: Vec<Project> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for prj in projects {
            let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
                .fluent()
                .select()
                .fields(paths!(ProjectMember::{id, project_id, uid}))
                .from(COLLECTION_MEMBER)
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

            let project_members: Vec<ProjectMember> = match object_stream.try_collect().await {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            for member in project_members {
                if let Err(e) = db
                    .fluent()
                    .delete()
                    .from(COLLECTION_MEMBER)
                    .document_id(&member.id)
                    .add_to_transaction(&mut transaction)
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }

            let object_stream: BoxStream<FirestoreResult<Ticket>> = match db
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

            let tickets: Vec<Ticket> = match object_stream.try_collect().await {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            for ticket in tickets {
                let object_stream: BoxStream<FirestoreResult<TicketMember>> = match db
                    .fluent()
                    .select()
                    .fields(paths!(TicketMember::{id, ticket_id, project_id, uid, seq}))
                    .from(super::ticket::COLLECTION_MEMBER)
                    .filter(|q| q.for_all([q.field(path!(TicketMember::ticket_id)).eq(&ticket.id)]))
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
                .from(COLLECTION_NAME)
                .document_id(&prj.id)
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

impl ProjectMember {
    pub fn new(id: &str, project_id: &str, uid: &str) -> Self {
        Self {
            id: id.to_string(),
            project_id: project_id.to_string(),
            uid: uid.to_string(),
            role: None,
            last_used: None,
            name: None,
            email: None,
            project_name: None,
        }
    }

    /// プロジェクトのメンバーを取得する
    pub async fn members_of_project(
        project_id: &str,
        order_by_uid: bool,
        db: &FirestoreDb,
    ) -> Result<Vec<Self>> {
        let order = match order_by_uid {
            true => vec![(
                path!(ProjectMember::uid),
                FirestoreQueryDirection::Ascending,
            )],
            false => vec![
                (
                    path!(ProjectMember::role),
                    FirestoreQueryDirection::Ascending,
                ),
                (
                    path!(ProjectMember::email),
                    FirestoreQueryDirection::Ascending,
                ),
            ],
        };

        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, uid, role, email, name, last_used}))
            .from(COLLECTION_MEMBER)
            .filter(|q| q.for_all([q.field(path!(ProjectMember::project_id)).eq(&project_id)]))
            .order_by(order)
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let project_members: Vec<ProjectMember> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(project_members)
    }

    /// プロジェクトメンバー情報を取得する
    pub async fn find(project_id: &str, uid: &str, db: &FirestoreDb) -> Result<Option<Self>> {
        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, uid, role, email, name, last_used}))
            .from(COLLECTION_MEMBER)
            .filter(|q| {
                q.for_all([
                    q.field(path!(ProjectMember::project_id)).eq(&project_id),
                    q.field(path!(ProjectMember::uid)).eq(&uid),
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

        let project_members: Vec<ProjectMember> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        if let Some(m) = project_members.get(0) {
            Ok(Some(m.clone()))
        } else {
            Ok(None)
        }
    }

    /// 自分のプロジェクトを検索する
    pub async fn my_projects(session: &Session, db: &FirestoreDb) -> Result<(Vec<Self>, i32)> {
        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, uid, role, last_used}))
            .from(COLLECTION_MEMBER)
            .filter(|q| q.for_all([q.field(path!(ProjectMember::uid)).eq(&session.uid)]))
            .order_by([(
                path!(ProjectMember::last_used),
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

        let mut project_members: Vec<ProjectMember> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut members = Vec::new();
        let mut owner_cnt = 0;
        for member in &mut project_members {
            let prj: Option<Project> = match db
                .fluent()
                .select()
                .by_id_in(&COLLECTION_NAME)
                .obj()
                .one(&member.project_id)
                .await
            {
                Ok(p) => p,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };
            if let Some(p) = prj {
                if p.deleted {
                    continue;
                }
                if let Some(r) = member.role {
                    if r == 1 {
                        owner_cnt += 1;
                    }
                    let mut m = member.clone();
                    m.project_name = p.project_name;
                    members.push(m);
                }
            }
        }

        Ok((members, owner_cnt))
    }

    /// ユーザーの選択中のプロジェクトを設定する
    pub async fn update_current(
        session: &Session,
        project_id: &str,
        db: &FirestoreDb,
    ) -> Result<()> {
        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, uid, role, last_used}))
            .from(COLLECTION_MEMBER)
            .filter(|q| {
                q.for_all([
                    q.field(path!(ProjectMember::project_id)).eq(&project_id),
                    q.field(path!(ProjectMember::uid)).eq(&session.uid),
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

        let project_members: Vec<ProjectMember> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        if let Some(member) = project_members.get(0) {
            let mut member = member.clone();
            member.last_used = Some(Utc::now());
            if let Err(e) = db
                .fluent()
                .update()
                .fields(paths!(ProjectMember::{last_used}))
                .in_col(&COLLECTION_MEMBER)
                .document_id(&member.id)
                .object(&member)
                .execute::<ProjectMember>()
                .await
            {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        }

        Ok(())
    }

    /// ロールを文字列に変換する
    pub fn role_to_string(&self) -> String {
        match self.role {
            Some(r) => match r {
                1 => "オーナー".to_string(),
                2 => "管理者".to_string(),
                3 => "メンバー".to_string(),
                4 => "閲覧者".to_string(),
                _ => "Unknown".to_string(),
            },
            None => "Unknown".to_string(),
        }
    }
}
