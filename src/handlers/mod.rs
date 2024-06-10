use crate::model;
use anyhow::Result;
use firestore::*;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub mod agreement;
pub mod api;
pub mod contact;
pub mod db_check;
pub mod home;
pub mod login;
pub mod news;
pub mod project;
pub mod ticket;
pub mod ticket_list;
pub mod user_name;
pub mod validation;

const COOKIE_SESSION_ID: &str = "yuimarl_session-id";

fn get_session_id(cookies: Cookies, should_exist: bool) -> Result<String> {
    let mut session_id = match cookies.get(COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            if should_exist {
                return Err(anyhow::anyhow!("failed to get session"));
            }
            "".to_string()
        }
    };

    if session_id.len() == 0 {
        let id = Uuid::now_v7().to_string();
        let mut c = Cookie::new(COOKIE_SESSION_ID, id.clone());
        let mut expire = time::OffsetDateTime::now_utc();
        expire += time::Duration::weeks(52);
        c.set_expires(expire);
        cookies.add(c);
        session_id = id;
    }

    Ok(session_id)
}

async fn get_session_info(
    cookies: Cookies,
    should_exist: bool,
    db: &FirestoreDb,
) -> Result<model::session::Session> {
    let session_id = get_session_id(cookies, should_exist)?;
    let session = model::session::Session::find(&session_id, db).await?;

    let session = match session {
        Some(s) => s,
        None => return Err(anyhow::anyhow!("failed to get session")),
    };

    Ok(session)
}

fn session_delete(cookies: Cookies) {
    if let Some(_) = cookies.get(COOKIE_SESSION_ID) {
        cookies.add(Cookie::new(COOKIE_SESSION_ID, ""));
    }
}

pub async fn health() -> &'static str {
    "OK"
}
