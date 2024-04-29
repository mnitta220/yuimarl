use crate::model;
use axum::extract::Form;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserByEmainInput {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub result: String,
    pub users: Vec<User>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub name: String,
}

pub async fn user_by_email(Form(input): Form<UserByEmainInput>) -> String {
    tracing::debug!("GET /user_by_email");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            let result = UserResult {
                result: "NG".to_string(),
                users: Vec::new(),
                message: format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let users = match model::user::User::search_by_email(&input.email, &db).await {
        Ok(u) => u,
        Err(e) => {
            let result = UserResult {
                result: "NG".to_string(),
                users: Vec::new(),
                message: format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(e) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let mut ret = Vec::new();

    for user in users {
        let u = User {
            uid: user.uid,
            email: user.email,
            name: user.name,
        };
        ret.push(u);
    }

    let result = UserResult {
        result: "OK".to_string(),
        users: ret,
        message: "".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}
