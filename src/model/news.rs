use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct News {
    pub timestamp: DateTime<Utc>,
    pub uid: String,
    pub event: i32,
    pub project_id: String,
    pub ticket_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NewsEvent {
    ProjectMemberAdd = 1,    // プロジェクトメンバーに追加された
    ProjectRoleUpdate = 2,   // プロジェクトメンバーのロールを更新した
    ProjectMemberDelete = 3, // プロジェクトメンバーから削除された
    TicketMemberAdd = 4,     // チケットメンバーに追加された
    TicketMemberDelete = 5,  // チケットメンバーから削除された
    TicketUpdate = 6,        // チケットが更新された
}
