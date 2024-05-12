use super::session::Session;
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const COLLECTION_NAME: &'static str = "project";
const COLLECTION_MEMBER: &'static str = "project_member";
pub const MEMBER_LIMIT_DEFAULT: i32 = 20;
pub const TICKET_LIMIT_DEFAULT: i32 = 1000;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: String,                // ID(uuid)
    pub project_name: String,      // プロジェクト名
    pub owner: String,             // プロジェクトオーナー
    pub prefix: String,            // チケット接頭辞
    pub language: String,          // 言語
    pub member_limit: i32,         // 最大メンバー数
    pub ticket_limit: i32,         // 最大チケット番号
    pub ticket_number: i32,        // チケット番号
    pub note: Option<String>,      // ノート（マークダウン）
    pub created_at: DateTime<Utc>, // 作成日時
    pub deleted: bool,             // 削除フラグ
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectValidation {
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectMember {
    pub id: String,                       // ID(uuid)
    pub project_id: String,               // プロジェクトID
    pub member: String,                   // メンバーのユーザーID
    pub role: i32,                        // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
    pub last_used: Option<DateTime<Utc>>, // 最終使用日時
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProjectRole {
    Owner = 1,
    Administrator = 2,
    Member = 3,
    Viewer = 4,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectMemberSub {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub role: i32,
}

impl Project {
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
            .fields(paths!(ProjectMember::{id, project_id, member, role, last_used})) // Optionally select the fields needed
            .from(COLLECTION_MEMBER)
            .filter(|q| q.for_all([q.field(path!(ProjectMember::member)).eq(&session.uid)]))
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
                .one(&member.project_id)
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

    /*
    pub async fn my_projects(session: &Session, db: &FirestoreDb) -> Result<Vec<Self>> {
        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, member, role, last_used})) // Optionally select the fields needed
            .from(COLLECTION_MEMBER)
            .filter(|q| q.for_all([q.field(path!(ProjectMember::member)).eq(&session.uid)]))
            .order_by([(
                path!(ProjectMember::last_used),
                FirestoreQueryDirection::Descending,
            )])
            .obj() // Reading documents as structures using Serde gRPC deserializer
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

        let mut projects: Vec<Project> = Vec::new();
        for member in project_members {
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
                projects.push(p);
            }
        }

        Ok(projects)
    }
    */

    pub async fn find_by_owner_and_name(
        owner: &String,
        project_name: &String,
        db: &FirestoreDb,
    ) -> Result<Vec<Self>> {
        let object_stream: BoxStream<FirestoreResult<Project>> = match db
            .fluent()
            .select()
            .fields(paths!(Project::{id, project_name, language, owner, created_at})) // Optionally select the fields needed
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Project::owner)).eq(owner),
                    q.field(path!(Project::project_name)).eq(project_name),
                ])
            })
            .obj() // Reading documents as structures using Serde gRPC deserializer
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
        members: serde_json::Value,
        db: &FirestoreDb,
    ) -> Result<Self> {
        let mut prj = Project {
            id: "".to_string(),
            project_name: input.project_name.trim().to_string(),
            owner: session.uid.clone(),
            prefix: input.prefix.trim().to_string(),
            language: "ja".to_string(),
            member_limit: MEMBER_LIMIT_DEFAULT,
            ticket_limit: TICKET_LIMIT_DEFAULT,
            ticket_number: 0,
            note: None,
            created_at: Utc::now(),
            deleted: false,
        };
        let mut count = 0u32;

        loop {
            count += 1;
            if count > 9 {
                return Err(anyhow::anyhow!("Failed to create project".to_string()));
            }
            let id = Uuid::now_v7().to_string();
            prj.id = id.clone();

            match db
                .fluent()
                .insert()
                .into(&COLLECTION_NAME)
                .document_id(id)
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
                        continue;
                    }
                    _ => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                },
            };
        }

        let empty_vec: Vec<serde_json::Value> = Vec::new();
        let mem = members["members"].as_array().unwrap_or_else(|| &empty_vec);
        let mut i = 0;
        for m in mem {
            let mut member = ProjectMember {
                id: "".to_string(),
                project_id: prj.id.clone(),
                member: String::from(m["uid"].as_str().unwrap()),
                role: m["role"].as_i64().unwrap() as i32,
                last_used: None,
            };
            if i == 0 {
                member.last_used = Some(Utc::now());
            }
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
            i += 1;
        }

        tracing::debug!("Project inserted {:?}", prj);

        Ok(prj)
    }
}

impl ProjectMember {
    pub async fn find(project_id: &str, member: &str, db: &FirestoreDb) -> Result<Vec<Self>> {
        let object_stream: BoxStream<FirestoreResult<ProjectMember>> = match db
            .fluent()
            .select()
            .fields(paths!(ProjectMember::{id, project_id, member, role, last_used})) // Optionally select the fields needed
            .from(COLLECTION_MEMBER)
            .filter(|q| {
                q.for_all([
                    q.field(path!(ProjectMember::project_id)).eq(&project_id),
                    q.field(path!(ProjectMember::member)).eq(&member),
                ])
            })
            .order_by([(
                path!(ProjectMember::last_used),
                FirestoreQueryDirection::Descending,
            )])
            .obj() // Reading documents as structures using Serde gRPC deserializer
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
        match ProjectMember::find(project_id, member, db).await {
            Ok(mut members) => {
                for m in members.iter_mut() {
                    m.last_used = Some(Utc::now());
                    tracing::debug!("update_last_used member={}", m.id);
                    if let Err(e) = db
                        .fluent()
                        .update()
                        .fields(paths!(ProjectMember::last_used))
                        .in_col(&COLLECTION_MEMBER)
                        .document_id(&m.id)
                        .object(m)
                        .execute::<ProjectMember>()
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
                .fields(paths!(ProjectMember::{id, project_id, member, role, last_used})) // Optionally select the fields needed
                .from(COLLECTION_MEMBER)
                .filter(|q| {
                    q.for_all([
                        q.field(path!(ProjectMember::project_id)).eq(&project_id),
                        q.field(path!(ProjectMember::member)).eq(&uid),
                    ])
                })
                .order_by([(
                    path!(ProjectMember::last_used),
                    FirestoreQueryDirection::Descending,
                )])
                .obj() // Reading documents as structures using Serde gRPC deserializer
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
            let mut member = ProjectMember {
                id: "".to_string(),
                project_id: project_id.to_string(),
                member: uid.to_string(),
                role: role.parse::<i32>().unwrap(),
                last_used: None,
            };
            let mut count = 0u32;

            loop {
                count += 1;
                if count > 9 {
                    return Err(anyhow::anyhow!("Failed to create project".to_string()));
                }
                let id = Uuid::now_v7().to_string();
                member.id = id.clone();

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

    pub fn role_to_string(&self) -> String {
        match self.role {
            1 => "オーナー".to_string(),
            2 => "管理者".to_string(),
            3 => "メンバー".to_string(),
            4 => "閲覧者".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl ProjectMemberSub {
    pub fn role_to_string(&self) -> String {
        match self.role {
            1 => "オーナー".to_string(),
            2 => "管理者".to_string(),
            3 => "メンバー".to_string(),
            4 => "閲覧者".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
