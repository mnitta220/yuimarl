use crate::{model, pages::login_page::LoginPage, AppError};
use anyhow::Result;
use axum::{extract::Path, response::Html};
use firestore::*;
use tower_cookies::Cookies;

pub async fn del_news(cookies: Cookies, Path(id): Path<String>) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /del_news/{}", id);

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

    if let Err(e) = model::news::News::delete(&id, &session.uid, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    return super::home::show_home(session, &db).await;
}
