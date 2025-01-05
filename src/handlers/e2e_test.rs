use super::validation;
use crate::{model, AppError};
use anyhow::Result;
use axum::{extract::Path, response::Html};
use firestore::*;
use tower_cookies::Cookies;

pub async fn get(
    cookies: Cookies,
    Path(e2e_password): Path<String>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /e2e_test/{}", e2e_password);
    println!("***1");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };
    println!("***2");

    if let Err(e) = validation::e2e_test::E2eTestValidation::validate(&e2e_password, &db).await {
        return Err(AppError(e));
    }
    println!("***3");
    let user = match model::user::User::e2e_test_user(&db).await {
        Ok(u) => u,
        Err(e) => {
            return Err(AppError(e));
        }
    };
    println!("***4");
    let session_id = super::get_session_id(cookies, false)?;

    let session = match model::session::Session::e2e_test_session(&db, &session_id, &user).await {
        Ok(s) => s,
        Err(e) => {
            return Err(AppError(e));
        }
    };
    println!("***5");
    return super::home::show_home(session, &db).await;
}
