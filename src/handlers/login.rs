use crate::{model, pages::login_page::LoginPage, AppError};
use anyhow::Result;
use axum::{
    extract::Form,
    response::{Html, Redirect},
};
use chrono::Utc;
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize, Debug)]
pub struct LoginInput {
    pub display_name: String,
    pub email: String,
    pub photo_url: String,
    pub uid: String,
    pub locale: String,
}

pub async fn get_login() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /login");

    Ok(Html(LoginPage::write()))
}

pub async fn post_login(
    cookies: Cookies,
    Form(input): Form<LoginInput>,
) -> Result<Redirect, AppError> {
    tracing::debug!("POST /login {}, {}", input.display_name, input.email);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session_id = super::session_info(cookies, false)?;

    let session = model::session::Session {
        id: session_id,
        uid: input.uid.clone(),
        name: input.display_name,
        email: input.email,
        photo_url: input.photo_url,
        project_id: None,
        created_at: Utc::now(),
    };
    if let Err(e) = model::session::Session::upsert(&session, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    };

    let user = match model::user::User::find(&input.uid, &db).await {
        Ok(u) => u,
        Err(e) => return Err(AppError(anyhow::anyhow!(e))),
    };

    if let None = user {
        return Ok(Redirect::to("/agree"));
    }

    Ok(Redirect::to("/"))
}

pub async fn get_logout(cookies: Cookies) -> Result<Redirect, AppError> {
    tracing::info!("GET /logout");

    super::session_delete(cookies);

    Ok(Redirect::to("/login"))
}
