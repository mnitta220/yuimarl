use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, user_name_page::UserNamePage},
    AppError,
};
use anyhow::Result;
use axum::response::Html;
use firestore::*;
use tower_cookies::Cookies;

pub async fn get_home(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /");

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

    return show_home(session, &db).await;
}

pub async fn show_home(
    mut session: model::session::Session,
    db: &FirestoreDb,
) -> Result<Html<String>, AppError> {
    let mut props = page::Props::new();
    let memo: Option<String>;

    let user = match model::user::User::find(&session.uid, &db).await {
        Ok(user) => user,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    if let Some(user) = user {
        if user.status != model::user::UserStatus::Approved as i32 {
            return Err(AppError(anyhow::anyhow!("このシステムを使用できません。")));
        }
        if user.name.trim().len() == 0 {
            let mut page = UserNamePage::new(props);
            return Ok(Html(page.write()));
        }

        if session.name.is_empty() {
            session.name = user.name.clone();
            if let Err(e) = model::session::Session::upsert(&session, &db).await {
                return Err(AppError(anyhow::anyhow!(e)));
            };
        }
        memo = user.memo;
    } else {
        return Ok(Html(LoginPage::write()));
    }

    let (project, member, tickets) =
        match model::project::Project::current_project_and_tickets(&session, &db).await {
            Ok((project, member, tickets)) => (project, member, tickets),
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    let owner_cnt = match model::project::Project::count_owner_projects(&session.uid, &db).await {
        Ok(cnt) => cnt,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let news = match model::news::News::get_news_list(&session.uid, &db).await {
        Ok(news) => news,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    props.screen = Some(crate::Screen::Home);
    props.project = project;
    props.project_member = member;
    props.tickets = tickets;
    props.session = Some(session);
    props.news = news;
    let mut page = HomePage::new(props, owner_cnt, memo);

    Ok(Html(page.write()))
}
