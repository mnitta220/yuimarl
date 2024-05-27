use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const MAX_HISTORY: usize = 50;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct History {
    pub timestamp: DateTime<Utc>,
    pub uid: String,
    pub user_name: String,
    pub event: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum HistoryEvent {
    ProjectCreate = 1,
    UpdateInfo = 2,
    UpdateNote = 3,
    TicketCreate = 4,
}

impl History {
    pub fn history_to_string(&self) -> String {
        match self.event {
            1 => "プロジェクト作成".to_string(),
            2 => "基本情報更新".to_string(),
            3 => "ノート更新".to_string(),
            4 => "チケット作成".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
