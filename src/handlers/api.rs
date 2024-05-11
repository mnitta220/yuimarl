use crate::model;
use axum::extract::Form;
use firestore::*;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

#[derive(Serialize, Deserialize, Debug)]
pub struct FirebaseConfig {
    pub api_key: String,
    pub auth_domain: String,
    pub project_id: String,
    pub storage_bucket: String,
    pub messaging_sender_id: String,
    pub app_id: String,
    pub measurement_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UserByEmainInput {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub result: String,
    pub users: Vec<model::user::UserSub>,
    pub message: String,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub name: String,
}
*/

#[derive(Deserialize, Debug)]
pub struct MemberAddInput {
    pub members: String,
}

pub async fn firebase_config() -> String {
    tracing::debug!("GET /firebase_config");

    let config = FirebaseConfig {
        api_key: crate::API_KEY.get().unwrap().clone(),
        auth_domain: crate::AUTH_DOMAIN.get().unwrap().clone(),
        project_id: crate::GOOGLE_PROJECT_ID.get().unwrap().clone(),
        storage_bucket: crate::STORAGE_BUCKET.get().unwrap().clone(),
        messaging_sender_id: crate::MESSAGING_SENDER_ID.get().unwrap().clone(),
        app_id: crate::APP_ID.get().unwrap().clone(),
        measurement_id: crate::MEASUREMENT_ID.get().unwrap().clone(),
    };

    let buf = match serde_json::to_string(&config) {
        Ok(r) => r,
        Err(_) => "NG".to_string(),
    };

    buf
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

    /*
    let mut ret = Vec::new();

    for user in users {
        /*
        let u = model::user::User {
            uid: user.uid,
            email: user.email,
            name: user.name,
        };
        */
        ret.push(user);
    }
    */

    let result = UserResult {
        result: "OK".to_string(),
        users: users,
        message: "".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}

pub async fn member_add(cookies: Cookies, Form(input): Form<MemberAddInput>) -> String {
    tracing::debug!("GET /member_add");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return format!("メンバーの追加に失敗しました。 [{}]", e.to_string());
        }
    };

    let session_id = match super::session_info(cookies, true) {
        Ok(session_id) => session_id,
        Err(e) => {
            return format!("メンバーの追加に失敗しました。 [{}]", e.to_string());
        }
    };

    let session = match model::session::Session::find(&session_id, &db).await {
        Ok(s) => s,
        Err(e) => {
            return format!("メンバーの追加に失敗しました。 [{}]", e.to_string());
        }
    };

    let session = match session {
        Some(s) => s,
        None => {
            return format!("メンバーの追加に失敗しました。");
        }
    };

    let project_id = match &session.project_id {
        Some(p) => p,
        None => {
            return format!("メンバーの追加に失敗しました。");
        }
    };

    let r = match model::project::ProjectMember::add_project_member(project_id, &input.members, &db)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return format!("メンバーの追加に失敗しました。 [{}]", e.to_string());
        }
    };

    if r.len() == 0 {
        "OK".to_string()
    } else {
        r
    }
}
