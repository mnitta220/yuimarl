use crate::{
    model,
    pages::{
        agreement_page::AgreementPage, home_page::HomePage, login_page::LoginPage, page,
        user_name_page::UserNamePage,
    },
    AppError,
};
use anyhow::Result;
use axum::response::Html;
use firestore::*;
use tower_cookies::Cookies;

pub async fn get() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /agree");

    let props = page::Props::new();
    let mut page = AgreementPage::new(props, false);

    Ok(Html(page.write()))
}

pub async fn get_disagree() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /agree");

    let props = page::Props::new();
    let mut page = AgreementPage::new(props, true);

    Ok(Html(page.write()))
}

pub async fn post(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /agree");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session = match super::get_session_info(cookies, false, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new();

    if let Err(e) = model::user::User::insert(&session, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    if session.name.trim().is_empty() {
        let mut page = UserNamePage::new(props);
        return Ok(Html(page.write()));
    }

    props.session = Some(session);

    let mut page = HomePage::new(props, 0, None);

    Ok(Html(page.write()))
}
