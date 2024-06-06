use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, user_name_page::UserNamePage},
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

    return show_home(session, &db).await;
}

pub async fn show_home(
    session: model::session::Session,
    db: &FirestoreDb,
) -> Result<Html<String>, AppError> {
    let mut props = page::Props::new(&session.id);

    let user = match model::user::User::find(&session.uid, &db).await {
        Ok(user) => user,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    if let Some(user) = user {
        if user.name.trim().len() == 0 {
            let mut page = UserNamePage::new(props);
            return Ok(Html(page.write()));
        }
    } else {
        return Ok(Html(LoginPage::write()));
    }

    let (project, member, tickets) =
        match model::project::Project::current_project_and_tickets(&session, &db).await {
            Ok((project, member, tickets)) => (project, member, tickets),
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    props.project = project;
    props.project_member = member;
    props.tickets = tickets;
    props.session = Some(session);
    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
