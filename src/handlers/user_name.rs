use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, user_name_page::UserNamePage},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

/*
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
*/

#[derive(Deserialize, Debug)]
pub struct UserNameInput {
    pub name: String,
}

pub async fn post(
    cookies: Cookies,
    Form(input): Form<UserNameInput>,
) -> Result<Html<String>, AppError> {
    tracing::info!("POST /user_name: {}", input.name);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let mut session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };

    let mut props = page::Props::new(&session.id);

    let name = input.name.trim();

    if name.len() == 0 {
        let mut page = UserNamePage::new(props);
        return Ok(Html(page.write()));
    }

    match model::user::User::update_name(&session.uid, name, &db).await {
        Ok(_) => (),
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    }

    match model::session::Session::update_name(&mut session, name, &db).await {
        Ok(_) => (),
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
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
