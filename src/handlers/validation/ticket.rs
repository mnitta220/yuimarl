use crate::{handlers, model};
use anyhow::Result;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketValidation {
    pub info: Option<String>,
    pub name: Option<String>,
}

impl TicketValidation {
    pub fn new() -> Self {
        Self {
            info: None,
            name: None,
        }
    }

    pub async fn validate_post(
        input: &handlers::ticket::TicketInput,
        session: &model::session::Session,
        action: crate::Action,
        db: &FirestoreDb,
    ) -> Result<Option<Self>> {
        match action {
            // チケット作成
            crate::Action::Create => {
                // TODO チケット作成権限がなければ作成できない。
                // TODO チケット作成件数の制限を超えていたら作成できない。
                // TODO チケット名が入力されていれば作成できない。
                // TODO チケット名が重複していたら作成できない。
            }
            _ => {}
        }

        Ok(None)
    }
}
