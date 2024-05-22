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

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };

    let mut props = page::Props::new(&session.id);
    props.title = Some("ホーム".to_string());

    let (project, member) = match model::project::Project::current_project(&session, &db).await {
        Ok((project, member)) => (project, member),
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    props.project = project;
    props.member = member;
    props.session = Some(session);
    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
