use super::validation;
use crate::{
    model,
    pages::{
        home_page::HomePage, login_page::LoginPage, page, ticket_list_page::TicketListPage,
        ticket_page::TicketPage,
    },
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, extract::Query, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    tab: Option<String>,
}

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

    let (project, member, tickets) =
        match model::project::Project::current_project(&session, &db).await {
            Ok((project, member, tickets)) => (project, member, tickets),
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    props.project = project;
    props.project_member = member;
    props.tickets = tickets;
    props.session = Some(session);

    let mut page = TicketPage::new(props);

    Ok(Html(page.write()))
}

pub async fn get(cookies: Cookies, Query(params): Query<Params>) -> Result<Html<String>, AppError> {
    let id = params.id.unwrap_or_default();
    let tab = params.tab.unwrap_or_default();
    tracing::debug!("GET /ticket id={} tab={}", id, tab);

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
    props.action = crate::Action::Update;
    props.title = Some("チケット".to_string());

    match tab.as_ref() {
        "note" => {
            props.tab = crate::Tab::Note;
        }
        "comment" => {
            props.tab = crate::Tab::Comment;
        }
        "history" => {
            props.tab = crate::Tab::History;
        }
        _ => {
            props.tab = crate::Tab::Info;
        }
    }

    let (ticket, project, members, parent, children) =
        match model::ticket::Ticket::find_ticket_and_project(&id, &db).await {
            Ok(ticket) => ticket,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    props.ticket = ticket;
    props.project = project;
    props.ticket_members = members;
    props.ticket_parent = parent;
    props.ticket_children = children;

    props.session = Some(session);
    let mut page = TicketPage::new(props);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct TicketInput {
    pub action: String,
    pub name: String,
    pub description: String,
    pub members: String,
    pub start_date: String,
    pub end_date: String,
    pub progress: String,
    pub priority: String,
    pub parent: String,
    pub deliverables: String,
    pub project_id: String,
    pub ticket_id: String,
    pub timestamp: String,
}

pub async fn post(
    cookies: Cookies,
    Form(input): Form<TicketInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /ticket {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        input.action,
        input.name,
        input.description,
        input.members,
        input.start_date,
        input.end_date,
        input.progress,
        input.priority,
        input.parent,
        input.deliverables,
        input.project_id,
        input.ticket_id,
        input.timestamp
    );

    let members = format!(r#"{{"members":{}}}"#, input.members);
    let members: serde_json::Value = match serde_json::from_str(&members) {
        Ok(m) => m,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

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

    let action = match input.action.as_ref() {
        "Create" => crate::Action::Create,
        "Update" => crate::Action::Update,
        "Delete" => crate::Action::Delete,
        _ => {
            return Err(AppError(anyhow::anyhow!(format!(
                "invalid action: {}",
                input.action
            ))));
        }
    };

    let project = model::project::Project::find(&input.project_id, &db).await?;
    if project.is_none() {
        return Ok(Html(LoginPage::write()));
    }

    let mut ticket_members = Vec::new();
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let mem = members["members"].as_array().unwrap_or_else(|| &empty_vec);

    let mut i = 0;
    for m in mem {
        let mut member = model::ticket::TicketMember::new(String::from(m["uid"].as_str().unwrap()));
        member.seq = i;
        ticket_members.push(member);
        i += 1;
    }

    let v = match validation::ticket::TicketValidation::validate_post(
        &input,
        &session,
        action.clone(),
        &db,
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError(e));
        }
    };

    if let Some(v) = v {
        let mut ticket = model::ticket::Ticket::new();
        ticket.name = Some(input.name);
        ticket.description = Some(input.description);
        ticket.start_date = Some(input.start_date);
        ticket.end_date = Some(input.end_date);
        ticket.progress = input.progress.parse::<i32>().unwrap_or_default();
        ticket.priority = input.priority.parse::<i32>().unwrap_or_default();

        props.session = Some(session);
        props.ticket = Some(ticket);
        props.ticket_validation = Some(v);
        props.ticket_members = ticket_members;
        let mut page = TicketPage::new(props);
        return Ok(Html(page.write()));
    }

    match action {
        crate::Action::Create => {
            // チケット作成
            if let Err(e) = model::ticket::Ticket::insert(
                &input,
                &session,
                project.as_ref().unwrap(),
                &ticket_members,
                &db,
            )
            .await
            {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        }
        crate::Action::Update => {
            // チケット更新
            if let Err(e) =
                model::ticket::Ticket::update(&input, &session, &ticket_members, &db).await
            {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        }
        _ => {}
    }

    let (project, member, tickets) =
        model::project::Project::current_project(&session, &db).await?;
    props.action = action;
    props.project = project;
    props.project_member = member;
    props.tickets = tickets;
    props.session = Some(session);
    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct NoteInput {
    pub markdown: String,
    pub ticket_id: String,
}

pub async fn post_note(
    cookies: Cookies,
    Form(input): Form<NoteInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /post_note {}", input.markdown,);

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
    match model::ticket::Ticket::update_note(&input, &session, &db).await {
        Ok(t) => t,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let (project, member, tickets) =
        model::project::Project::current_project(&session, &db).await?;
    props.action = crate::Action::Update;
    props.project = project;
    props.project_member = member;
    props.tickets = tickets;
    props.session = Some(session);

    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}

pub async fn get_list(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /ticket_list");

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
    props.title = Some("チケット一覧".to_string());

    /*
    let projects = match model::project::ProjectMember::my_projects(&session, &db).await {
        Ok(projects) => projects,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };
    */

    props.session = Some(session);
    //props.project_members = projects;
    let mut page = TicketListPage::new(props);

    Ok(Html(page.write()))
}
