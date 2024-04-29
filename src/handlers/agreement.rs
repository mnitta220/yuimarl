use crate::{
    model,
    pages::{agreement_page::AgreementPage, home_page::HomePage, login_page::LoginPage, page},
    AppError,
};
use anyhow::Result;
use axum::response::Html;
use firestore::*;
use tower_cookies::Cookies;

pub async fn get_agree(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /agree");

    let session_id = match super::session_info(cookies, false) {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let props = page::Props::new(&session_id);
    let mut page = AgreementPage::new(props, false);

    Ok(Html(page.write()))
}

pub async fn get_disagree(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /agree");

    let session_id = match super::session_info(cookies, false) {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let props = page::Props::new(&session_id);
    let mut page = AgreementPage::new(props, true);

    Ok(Html(page.write()))
}

pub async fn post_agree(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /agree");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session_id = match super::session_info(cookies, false) {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session_id);
    let session = model::session::Session::find(&session_id, &db).await?;

    let session = match session {
        Some(s) => s,
        None => return Ok(Html(LoginPage::write())),
    };
    if let Err(e) = model::user::User::insert(&session, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    let projects = model::project::Project::my_projects(&session, &db).await?;
    props.projects = projects;
    props.session = Some(session);

    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
