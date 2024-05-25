use crate::{handlers, model};
use anyhow::Result;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketValidation {
    pub name: Option<String>,
}

impl TicketValidation {
    pub fn new() -> Self {
        Self { name: None }
    }

    pub async fn validate_post(
        input: &handlers::ticket::TicketInput,
        session: &model::session::Session,
        action: crate::Action,
        db: &FirestoreDb,
    ) -> Result<Option<Self>> {
        match action {
            // プロジェクト作成
            crate::Action::Create => {
                // TODO プロジェクト作成件数の制限を超えていたら作成できない。
            }
            _ => {}
        }

        Ok(None)
    }
}
