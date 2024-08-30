use super::validation;
use crate::{
    pages::{db_check::DbCheckPage, login_page::LoginPage, page},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

pub async fn get() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /db_check");

    let mut props = page::Props::new();
    props.title = Some(String::from("データベースチェック"));
    let mut page = DbCheckPage::new(props, None, None);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct DbCheckInput {
    pub db_check_password: String,
}

pub async fn post(
    cookies: Cookies,
    Form(input): Form<DbCheckInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /db_check, db_check_password: {}",
        input.db_check_password
    );

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

    let v =
        match validation::db_check::DbCheckValidation::validate_post(&input, &session, &db).await {
            Ok(v) => v,
            Err(e) => {
                return Err(AppError(e));
            }
        };

    let mut props = page::Props::new();
    props.title = Some(String::from("データベースチェック"));
    props.session = Some(session);
    let mut page = DbCheckPage::new(props, Some(input.db_check_password), v);

    Ok(Html(page.write()))
}
