use super::validation;
use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, ticket_page::TicketPage},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use firestore::*;
use serde::Deserialize;
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
    props.project_member = member;
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
    //pub project_id: String,
    //pub timestamp: String,
}

pub async fn post(
    cookies: Cookies,
    Form(input): Form<TicketInput>,
) -> Result<Html<String>, AppError> {
    tracing::info!(
        "POST /ticket {}, {}, {}, {}, {}, {}, {}, {}",
        input.action,
        input.name,
        input.description,
        input.members,
        input.start_date,
        input.end_date,
        input.progress,
        input.priority
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

    let project_id = match &session.project_id {
        Some(p) => p,
        None => return Ok(Html(LoginPage::write())),
    };
    let project = model::project::Project::find(&project_id, &db).await?;
    if project.is_none() {
        return Ok(Html(LoginPage::write()));
    }

    let mut ticket_members = Vec::new();
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let mem = members["members"].as_array().unwrap_or_else(|| &empty_vec);

    let mut i = 0;
    for m in mem {
        let mut member = model::ticket::TicketMember::new(String::from(m["uid"].as_str().unwrap()));
        //member.email = Some(String::from(m["email"].as_str().unwrap()));
        //member.name = Some(String::from(m["name"].as_str().unwrap()));
        //member.role = Some(m["role"].as_i64().unwrap() as i32);
        member.seq = Some(i);
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
        ticket.priority = Some(input.priority);

        //project.id = Some(input.project_id.clone());
        //project.project_name = Some(project_name);
        //project.prefix = Some(input.prefix);
        //let t = input.timestamp.parse::<i64>().unwrap_or_default();
        //project.updated_at = DateTime::from_timestamp_micros(t);
        props.session = Some(session);
        props.ticket = Some(ticket);
        props.ticket_validation = Some(v);
        props.ticket_members = ticket_members;
        let mut page = TicketPage::new(props);
        return Ok(Html(page.write()));
    }

    match action {
        crate::Action::Create => {
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
        _ => {}
    }

    let (project, member) = model::project::Project::current_project(&session, &db).await?;
    props.action = action;
    props.project = project;
    props.project_member = member;
    props.session = Some(session);
    let mut page = HomePage::new(props);

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
