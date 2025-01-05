use anyhow::Result;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct E2eTestValidation {
    pub info: Option<String>,
    pub e2e_test_password: Option<String>,
    pub result: bool,
}

impl E2eTestValidation {
    pub fn new() -> Self {
        Self {
            info: None,
            e2e_test_password: None,
            result: false,
        }
    }

    pub async fn validate(password: &str, _db: &FirestoreDb) -> Result<Option<Self>> {
        let mut validation = Self::new();

        let e2e_test_password = match crate::E2E_TEST_PASSWORD.get() {
            Some(password) => password.to_string(),
            None => "".to_string(),
        };

        if e2e_test_password.len() == 0 {
            return Err(anyhow::anyhow!(
                "E2E_TEST_PASSWORD 環境変数が設定されていません。"
            ));
        }

        if password != e2e_test_password {
            return Err(anyhow::anyhow!("E2E_TEST_PASSWORD が正しくありません。"));
        }

        validation.result = true;
        Ok(Some(validation))
    }
}
