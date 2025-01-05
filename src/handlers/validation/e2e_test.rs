use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::handlers;

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

    pub async fn validate_post(input: &handlers::e2e_test::E2eTestInput) -> Result<Option<Self>> {
        let mut validation = Self::new();

        let e2e_test_password = match crate::E2E_TEST_PASSWORD.get() {
            Some(password) => password.to_string(),
            None => "".to_string(),
        };

        if e2e_test_password.len() == 0 {
            validation.info = Some("E2E_TEST_PASSWORD 環境変数が設定されていません".to_string());
            return Ok(Some(validation));
        }

        if input.e2e_test_password != e2e_test_password {
            validation.e2e_test_password =
                Some("E2E_TEST_PASSWORD が正しくありません。".to_string());
            return Ok(Some(validation));
        }

        validation.result = true;
        Ok(Some(validation))
    }
}
