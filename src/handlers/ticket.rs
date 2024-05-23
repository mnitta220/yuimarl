use crate::{
    model,
    pages::{login_page::LoginPage, page, ticket_page::TicketPage},
    AppError,
};
use anyhow::Result;
use axum::response::Html;
use firestore::*;
use tower_cookies::Cookies;

pub async fn get_add(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /ticket_add");

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

    let mut props = page::Props::new(&session.id);
    props.title = Some("チケットを作成".to_string());
    props.action = crate::Action::Create;

    let (project, member) = match model::project::Project::current_project(&session, &db).await {
        Ok((project, member)) => (project, member),
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    props.project = project;
    props.member = member;
    props.session = Some(session);

    let mut page = TicketPage::new(props);

    Ok(Html(page.write()))
}

/*
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

    let mut session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session.id);
    let project = model::project::Project::find(&input.project, &db).await?;

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

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session.id);

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
        let mut ticket = model::ticket::Ticket::new();
        ticket.project_id = Some(project_id.clone());
        ticket.name = Some(input.name);
        ticket.note = Some(input.note);
        ticket.start_date = Some(input.start_date);
        ticket.end_date = Some(input.end_date);
        ticket.progress = progress;
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
    props.session = Some(session);
    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
*/
