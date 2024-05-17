use super::session::Session;
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
    pub id: Option<String>,                // ID(uuid)
    pub project_name: Option<String>,      // プロジェクト名
    pub owner: Option<String>,             // プロジェクトオーナー
    pub prefix: Option<String>,            // チケット接頭辞
    pub language: Option<String>,          // 言語
    pub member_limit: Option<i32>,         // 最大メンバー数
    pub ticket_limit: Option<i32>,         // 最大チケット番号
    pub ticket_number: Option<i32>,        // チケット番号
    pub note: Option<String>,              // ノート（マークダウン）
    pub created_at: Option<DateTime<Utc>>, // 作成日時
    pub deleted: bool,                     // 削除フラグ
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectValidation {
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectMember {
    pub id: Option<String>,               // ID(uuid)
    pub project_id: Option<String>,       // プロジェクトID
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
    pub fn new() -> Self {
        Project {
            id: None,
            project_name: None,
            owner: None,
            prefix: None,
            language: None,
            member_limit: None,
            ticket_limit: None,
            ticket_number: None,
            note: None,
            created_at: None,
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

    pub async fn last_project(
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

        for member in project_members {
            match db
                .fluent()
                .select()
                .by_id_in(&COLLECTION_NAME)
                .obj::<Project>()
                .one(&member.project_id.clone().unwrap_or_default())
                .await
            {
                Ok(p) => match p {
                    Some(p) => {
                        if p.deleted {
                            continue;
                        }
                        return Ok((Some(p), Some(member)));
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

        Ok((None, None))
    }

    pub async fn find_by_owner_and_name(
        owner: &String,
        project_name: &String,
        db: &FirestoreDb,
    ) -> Result<Vec<Self>> {
        let object_stream: BoxStream<FirestoreResult<Project>> = match db
            .fluent()
            .select()
            .fields(paths!(Project::{id, project_name, owner, prefix, deleted}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Project::owner)).eq(owner),
                    q.field(path!(Project::project_name)).eq(project_name),
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

    pub async fn insert(
        input: &crate::handlers::project::ProjectInput,
        session: &Session,
        project_members: &mut Vec<ProjectMember>,
        db: &FirestoreDb,
    ) -> Result<Project> {
        let mut prj = Project::new();
        prj.project_name = Some(input.project_name.trim().to_string());
        prj.owner = Some(session.uid.clone());
        prj.prefix = Some(input.prefix.trim().to_string());
        prj.member_limit = Some(MEMBER_LIMIT_DEFAULT);
        prj.ticket_limit = Some(TICKET_LIMIT_DEFAULT);
        prj.ticket_number = Some(0);
        prj.created_at = Some(Utc::now());
        let mut count = 0u32;
        let mut id = Uuid::now_v7().to_string();

        loop {
            count += 1;
            if count > 9 {
                return Err(anyhow::anyhow!("Failed to create project".to_string()));
            }
            prj.id = Some(id.clone());

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
                        continue;
                    }
                    _ => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                },
            };
        }

        for member in project_members {
            member.project_id = Some(id.clone());
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
                member.id = Some(id.clone());

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
        }

        tracing::debug!("Project inserted {:?}", prj);

        Ok(prj)
    }

    pub async fn update(
        input: &crate::handlers::project::ProjectInput,
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
        prj.project_name = Some(input.project_name.trim().to_string());
        prj.prefix = Some(input.prefix.trim().to_string());

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Project::{project_name, prefix}))
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
                                up.project_id = Some(String::from(&input.project_id));
                                let id = Uuid::now_v7().to_string();
                                up.id = Some(id.clone());
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
                            }
                            u += 1;
                            upd = project_members.get(u);
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
                            if cur.email != up.email || cur.name != up.name || cur.role != up.role {
                                let mut cur = cur.clone();
                                cur.email = up.email.clone();
                                cur.name = up.name.clone();
                                cur.role = up.role;
                                if let Err(e) = db
                                    .fluent()
                                    .update()
                                    .fields(paths!(ProjectMember::last_used))
                                    .in_col(&COLLECTION_MEMBER)
                                    .document_id(&cur.id.as_ref().unwrap())
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
                    up.project_id = Some(String::from(&input.project_id));
                    let id = Uuid::now_v7().to_string();
                    up.id = Some(id.clone());
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

                    u += 1;
                    upd = project_members.get(u);
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

        tracing::debug!("Project updated {:?}", prj);

        Ok(prj)
    }

    pub async fn update_note(
        input: &crate::handlers::project::UpdNoteInput,
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
        prj.note = Some(input.markdown.trim().to_string());

        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(Project::{note}))
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
}

impl ProjectMember {
    pub fn new(uid: String) -> Self {
        Self {
            id: None,
            project_id: None,
            uid: uid,
            role: None,
            last_used: None,
            name: None,
            email: None,
            project_name: None,
        }
    }

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
            .fields(paths!(ProjectMember::{id, uid, role, email, name, last_used}))
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

    pub async fn update_last_used(project_id: &str, member: &str, db: &FirestoreDb) -> Result<()> {
        tracing::debug!(
            "update_last_used project={:?}, member={}",
            project_id,
            member
        );

        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, uid, role, last_used}))
            .from(COLLECTION_MEMBER)
            .filter(|q| {
                q.for_all([
                    q.field(path!(ProjectMember::project_id)).eq(&project_id),
                    q.field(path!(ProjectMember::uid)).eq(&member),
                ])
            })
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

        for m in project_members.iter_mut() {
            if let Some(id) = &m.id {
                m.last_used = Some(Utc::now());
                if let Err(e) = db
                    .fluent()
                    .update()
                    .fields(paths!(ProjectMember::last_used))
                    .in_col(&COLLECTION_MEMBER)
                    .document_id(id)
                    .object(m)
                    .execute::<ProjectMember>()
                    .await
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }

        Ok(())
    }

    pub async fn add_project_member(
        project_id: &str,
        add_members: &String,
        db: &FirestoreDb,
    ) -> Result<String> {
        let v: Vec<&str> = add_members.split(',').collect();
        let mut i = 0;
        let mut uid: &str;
        let mut role: &str;

        loop {
            if i >= v.len() {
                break;
            }
            uid = v[i];
            i += 1;
            if i >= v.len() {
                break;
            }
            role = v[i];
            i += 1;
            tracing::info!("uid={} role={}", uid, role);
            let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
                .fluent()
                .select()
                .fields(paths!(ProjectMember::{id, project_id, uid, role, last_used}))
                .from(COLLECTION_MEMBER)
                .filter(|q| {
                    q.for_all([
                        q.field(path!(ProjectMember::project_id)).eq(&project_id),
                        q.field(path!(ProjectMember::uid)).eq(&uid),
                    ])
                })
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
            if project_members.len() > 0 {
                return Ok(format!("{}はすでに存在します。", uid));
            }

            let mut member = ProjectMember::new(uid.to_string());
            member.project_id = Some(project_id.to_string());
            //member.uid = Some(uid.to_string());
            member.role = Some(role.parse::<i32>().unwrap());
            let mut count = 0u32;

            loop {
                count += 1;
                if count > 9 {
                    return Err(anyhow::anyhow!("Failed to create project".to_string()));
                }
                let id = Uuid::now_v7().to_string();
                member.id = Some(id.clone());
                member.last_used = Some(Utc::now());

                match db
                    .fluent()
                    .insert()
                    .into(&COLLECTION_MEMBER)
                    .document_id(id)
                    .object(&member)
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
        }

        Ok("".to_string())
    }

    pub async fn my_projects(session: &Session, db: &FirestoreDb) -> Result<Vec<Self>> {
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

        for member in &mut project_members {
            if let Some(id) = &member.project_id {
                let prj: Option<Project> = match db
                    .fluent()
                    .select()
                    .by_id_in(&COLLECTION_NAME)
                    .obj()
                    .one(id)
                    .await
                {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };
                if let Some(p) = prj {
                    member.project_name = p.project_name;
                }
            }
        }

        Ok(project_members)
    }

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
