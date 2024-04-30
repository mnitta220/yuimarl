use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, project_page::ProjectPage},
    AppError,
};
use axum::extract::Form;
use firestore::*;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

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

#[derive(Deserialize, Debug)]
pub struct MemberAddInput {
    pub members: String,
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
