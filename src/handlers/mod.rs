use anyhow::Result;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub mod agreement;
pub mod api;
pub mod contact;
pub mod home;
pub mod login;
pub mod project;
pub mod ticket;

const COOKIE_SESSION_ID: &str = "yuimarl_session-id";

fn session_info(cookies: Cookies, should_exist: bool) -> Result<String> {
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
        cookies.add(Cookie::new(COOKIE_SESSION_ID, id.clone()));
        session_id = id;
    }

    Ok(session_id)
}

fn session_delete(cookies: Cookies) {
    if let Some(_) = cookies.get(COOKIE_SESSION_ID) {
        cookies.add(Cookie::new(COOKIE_SESSION_ID, ""));
    }
}
