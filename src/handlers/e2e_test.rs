use super::validation;
use crate::{
    model,
    pages::{e2e_test::E2eTestPage, page},
    AppError,
};
use anyhow::Result;
use axum::{
    extract::Form,
    response::{Html, Redirect},
};
use firestore::*;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub async fn get() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /e2e_test");

    let mut props = page::Props::new();
    props.title = Some(String::from("E2Eテスト"));
    let mut page = E2eTestPage::new(props, None, None);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct E2eTestInput {
    pub e2e_test_password: String,
}

pub async fn post(cookies: Cookies, Form(input): Form<E2eTestInput>) -> Result<Redirect, AppError> {
    tracing::debug!(
        "POST /e2e_test, e2e_test_password: {}",
        input.e2e_test_password
    );

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let v = match validation::e2e_test::E2eTestValidation::validate_post(&input).await {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError(e));
        }
    };

    if let Some(validation) = v.as_ref() {
        if validation.result == false {
            return Ok(Redirect::to("/e2e_test"));
        }
    }
    let user = match model::user::User::e2e_test_user(&db).await {
        Ok(u) => u,
        Err(e) => {
            return Err(AppError(e));
        }
    };

    let session_id = Uuid::now_v7().to_string();
    let session = model::session::Session {
        id: session_id.to_string(),
        uid: user.uid.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
        photo_url: "".to_string(),
        e2e_test: Some(true),
        created_at: chrono::Utc::now(),
    };

    let mut c = Cookie::new(super::COOKIE_SESSION_ID, session_id);
    let mut expire = time::OffsetDateTime::now_utc();
    expire += time::Duration::weeks(52);
    c.set_expires(expire);
    cookies.add(c);

    if let Err(e) = model::session::Session::upsert(&session, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    Ok(Redirect::to("/"))
}
