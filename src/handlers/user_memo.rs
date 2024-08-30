use crate::{model, pages::login_page::LoginPage, AppError};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize, Debug)]
pub struct UserMemoInput {
    pub markdown: String,
}

pub async fn post(
    cookies: Cookies,
    Form(input): Form<UserMemoInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /user_memo: {}", input.markdown);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };

    let memo = input.markdown.trim();

    match model::user::User::update_memo(&session.uid, memo, &db).await {
        Ok(_) => (),
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    }

    return super::home::show_home(session, &db).await;
}
