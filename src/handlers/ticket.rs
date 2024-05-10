use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, ticket_page::TicketPage},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use chrono::Utc;
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize, Debug)]
pub struct TicketAddInput {
    pub project: String,
}

pub async fn post_add_ticket(
    cookies: Cookies,
    Form(input): Form<TicketAddInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /ticket/add {}", input.project);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session_id = match super::session_info(cookies, true) {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };

    let mut props = page::Props::new(&session_id);
    let session = model::session::Session::find(&session_id, &db).await?;
    let project = model::project::Project::find(&input.project, &db).await?;

    let mut session = match session {
        Some(s) => s,
        None => return Ok(Html(LoginPage::write())),
    };
    if let Err(e) =
        model::project::ProjectMember::update_last_used(&input.project, &session.uid, &db).await
    {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    session.project_id = Some(input.project);
    if let Err(e) = model::session::Session::update_project(&session, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }
    props.session = Some(session);
    props.project = project;
    let mut page = TicketPage::new(props);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct TicketCreateInput {
    pub name: String,
    pub note: String,
    pub start_date: String,
    pub end_date: String,
    pub progress: String,
    pub priority: String,
}

pub async fn post_create_ticket(
    cookies: Cookies,
    Form(input): Form<TicketCreateInput>,
) -> Result<Html<String>, AppError> {
    tracing::info!(
        "POST /ticket/create name={} note={} start_date={} end_date={} progress={} priority={}",
        input.name,
        input.note,
        input.start_date,
        input.end_date,
        input.progress,
        input.priority
    );

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session_id = match super::session_info(cookies, true) {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session_id);
    let session = model::session::Session::find(&session_id, &db).await?;

    let session = match session {
        Some(s) => s,
        None => return Ok(Html(LoginPage::write())),
    };

    let project_id = match &session.project_id {
        Some(p) => p,
        None => return Ok(Html(LoginPage::write())),
    };
    let project = model::project::Project::find(&project_id, &db).await?;
    if project.is_none() {
        return Ok(Html(LoginPage::write()));
    }

    let name = input.name.trim().to_string();
    let progress = match input.progress.parse::<i32>() {
        Ok(p) => p,
        Err(_) => 0,
    };

    if name.len() == 0 {
        let ticket = model::ticket::Ticket {
            id: "".to_string(),
            project_id: project_id.clone(),
            id_disp: "".to_string(),
            name: input.name,
            note: input.note,
            start_date: input.start_date,
            end_date: input.end_date,
            progress: progress,
            priority: input.priority,
            parent: "".to_string(),
            owner: session.uid.clone(),
            created_at: Utc::now(),
        };
        let validation = model::ticket::TicketValidation {
            name: Some("入力してください".to_string()),
        };
        props.session = Some(session);
        props.project = project;
        props.ticket = Some(ticket);
        props.ticket_validation = Some(validation);
        let mut page = TicketPage::new(props);
        return Ok(Html(page.write()));
    }

    if let Err(e) =
        model::ticket::Ticket::insert(&input, &session, project.as_ref().unwrap(), &db).await
    {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    props.project = project;
    //let projects = model::project::Project::my_projects(&session, &db).await?;
    //props.projects = projects;
    props.session = Some(session);
    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
