use crate::{handlers, model};
use anyhow::Result;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoticeValidation {
    pub info: Option<String>,
    pub message: Option<String>,
    pub notice_password: Option<String>,
    pub notice_id: Option<String>,
    pub result: bool,
}

impl NoticeValidation {
    pub fn new() -> Self {
        Self {
            info: None,
            message: None,
            notice_password: None,
            notice_id: None,
            result: false,
        }
    }

    pub async fn validate_post_add(
        input: &handlers::notice::NoticeAddInput,
        _session: &model::session::Session,
        _db: &FirestoreDb,
    ) -> Result<Option<Self>> {
        let mut validation = Self::new();

        let notice_password = match crate::NOTICE_PASSWORD.get() {
            Some(password) => password.to_string(),
            None => "".to_string(),
        };

        if notice_password.len() == 0 {
            validation.info = Some("NOTICE_PASSWORD 環境変数が設定されていません。".to_string());
            return Ok(Some(validation));
        }

        if input.password != notice_password {
            validation.notice_password = Some("NOTICE_PASSWORD が正しくありません。".to_string());
            return Ok(Some(validation));
        }

        if input.message.trim().len() == 0 {
            validation.message = Some("入力してください。".to_string());
            return Ok(Some(validation));
        }

        validation.result = true;
        Ok(Some(validation))
    }

    pub async fn validate_post_del(
        input: &handlers::notice::NoticeDelInput,
        _session: &model::session::Session,
        _db: &FirestoreDb,
    ) -> Result<Option<Self>> {
        let mut validation = Self::new();

        let notice_password = match crate::NOTICE_PASSWORD.get() {
            Some(password) => password.to_string(),
            None => "".to_string(),
        };

        if notice_password.len() == 0 {
            validation.info = Some("NOTICE_PASSWORD 環境変数が設定されていません。".to_string());
            return Ok(Some(validation));
        }

        if input.password != notice_password {
            validation.notice_password = Some("NOTICE_PASSWORD が正しくありません。".to_string());
            return Ok(Some(validation));
        }

        if input.notice_id.trim().len() == 0 {
            validation.notice_id = Some("入力してください。".to_string());
            return Ok(Some(validation));
        }

        validation.result = true;
        Ok(Some(validation))
    }
}
