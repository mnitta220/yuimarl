//use super::session::Session;
//use anyhow::Result;
use chrono::{DateTime, Utc};
//use firestore::*;
use serde::{Deserialize, Serialize};
//use uuid::Uuid;

//const COLLECTION_NAME: &'static str = "ticket";
//const COLLECTION_MEMBER: &'static str = "ticket_member";

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
    pub priority: Option<String>,          // 優先度
    pub parent: Option<String>,            // 親チケット
    pub deliverables: Option<String>,      // 成果物(JSON)
    pub owner: Option<String>,             // 登録ユーザー
    pub note: Option<String>,              // ノート（マークダウン）
    pub history: Option<String>,           // 更新履歴 (JSON)
    pub created_at: Option<DateTime<Utc>>, // 作成日時
    pub deleted: bool,                     // 削除フラグ
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketMember {
    pub id: Option<String>,        // ID(uuid)
    pub ticket_id: Option<String>, // チケットID
    pub uid: String,               // メンバーのユーザーID
    pub seq: Option<i32>,          // 表示順
    pub name: Option<String>,      // メンバーの名前
    pub email: Option<String>,     // メンバーのメールアドレス
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketValidation {
    pub name: Option<String>,
}

impl Ticket {
    /*
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
            priority: None,
            parent: None,
            deliverables: None,
            owner: None,
            note: None,
            history: None,
            created_at: None,
            deleted: false,
        }
    }
    */

    /*
    pub async fn insert(
        input: &crate::handlers::ticket::TicketCreateInput,
        session: &Session,
        project: &super::project::Project,
        db: &FirestoreDb,
    ) -> Result<()> {
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

        let mut ticket = Ticket::new();
        ticket.project_id = project.id.clone();
        ticket.id_disp = Some(id_disp);
        ticket.name = Some(input.name.clone());
        ticket.start_date = Some(input.start_date.clone());
        ticket.end_date = Some(input.end_date.clone());
        ticket.progress = progress;
        ticket.priority = Some(input.priority.clone());
        ticket.owner = Some(session.uid.clone());
        ticket.created_at = Some(Utc::now());
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

        let project_new = super::project::Project {
            ticket_number: Some(ticket_number),
            ..project.clone()
        };
        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(super::project::Project::ticket_number))
            .in_col(&super::project::COLLECTION_NAME)
            .document_id(id)
            .object(&project_new)
            .execute::<super::project::Project>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }
        tracing::debug!("Ticket inserted {:?}", ticket);

        Ok(())
    }
    */
}
