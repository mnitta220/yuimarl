use super::validation;
use crate::{
    model,
    pages::{login_page::LoginPage, notice_add::NoticeAddPage, notice_del::NoticeDelPage, page},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

pub async fn get_add() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /notice_add");

    let mut props = page::Props::new();
    props.title = Some(String::from("運用からのお知らせ"));
    let mut page = NoticeAddPage::new(props, None, None, None, None);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct NoticeAddInput {
    pub message: String,
    pub password: String,
}

pub async fn post_add(
    cookies: Cookies,
    Form(input): Form<NoticeAddInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /notice_add, message: {}, password: {}",
        input.message,
        input.password
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

    let v = match validation::notice::NoticeValidation::validate_post_add(&input, &session, &db)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError(e));
        }
    };

    let mut notice_id: Option<String> = None;
    if let Some(v) = &v {
        if v.result {
            let notice = match model::news::News::add_operation_notice(&input.message, &db).await {
                Ok(notice_id) => notice_id,
                Err(e) => {
                    return Err(AppError(e));
                }
            };
            notice_id = Some(notice);
        }
    }

    let mut props = page::Props::new();
    props.title = Some(String::from("運用からのお知らせ"));
    props.session = Some(session);
    let mut page = NoticeAddPage::new(
        props,
        Some(input.message),
        Some(input.password),
        v,
        notice_id,
    );

    Ok(Html(page.write()))
}

pub async fn get_del() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /notice_del");

    let mut props = page::Props::new();
    props.title = Some(String::from("運用からのお知らせ削除"));
    let mut page = NoticeDelPage::new(props, None, None, None);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct NoticeDelInput {
    pub notice_id: String,
    pub password: String,
}

pub async fn post_del(
    cookies: Cookies,
    Form(input): Form<NoticeDelInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /notice_del, notice_id: {}, password: {}",
        input.notice_id,
        input.password
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

    let v = match validation::notice::NoticeValidation::validate_post_del(&input, &session, &db)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError(e));
        }
    };

    if let Some(v) = &v {
        if v.result {
            if let Err(e) = model::news::News::del_operation_notice(&input.notice_id, &db).await {
                return Err(AppError(e));
            };
        }
    }

    let mut props = page::Props::new();
    props.title = Some(String::from("運用からのお知らせ削除"));
    props.session = Some(session);
    let mut page = NoticeDelPage::new(props, Some(input.notice_id), Some(input.password), v);

    Ok(Html(page.write()))
}
