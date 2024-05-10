use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page},
    AppError,
};
use anyhow::Result;
use axum::response::Html;
use firestore::*;
use tower_cookies::Cookies;

pub async fn get_home(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session_id = match super::session_info(cookies, true) {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session_id);
    let session = model::session::Session::find(&session_id, &db).await?;

    let session = match session {
        Some(s) => s,
        None => return Ok(Html(LoginPage::write())),
    };

    let (project, member) = model::project::Project::last_project(&session, &db).await?;
    props.project = project;
    props.member = member;
    props.session = Some(session);
    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
