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

pub async fn get(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /db_check");

    let session_id = match super::get_session_id(cookies, false) {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session_id);
    props.title = Some(String::from("データベースチェック"));
    let mut page = DbCheckPage::new(props);

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

    let mut props = page::Props::new(&session.id);
    props.title = Some(String::from("データベースチェック"));
    props.db_check_password = Some(input.db_check_password);
    props.db_check_validation = v;
    props.session = Some(session);
    let mut page = DbCheckPage::new(props);

    Ok(Html(page.write()))
}
