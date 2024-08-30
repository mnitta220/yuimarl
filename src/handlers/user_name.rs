use crate::{
    model,
    pages::{login_page::LoginPage, page, user_name_page::UserNamePage},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize, Debug)]
pub struct UserNameInput {
    pub name: String,
}

pub async fn post(
    cookies: Cookies,
    Form(input): Form<UserNameInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /user_name: {}", input.name);

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

    let props = page::Props::new();

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

    return super::home::show_home(session, &db).await;
}
