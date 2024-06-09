use super::validation;
use crate::{
    model,
    pages::{
        login_page::LoginPage, page, ticket_list_page::TicketListPage, ticket_page::TicketPage,
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
        match model::project::Project::current_project_and_tickets(&session, &db).await {
            Ok((project, member, tickets)) => (project, member, tickets),
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    props.project = project;
    props.project_member = member;
    props.tickets = tickets;
    props.session = Some(session);

    let mut page = TicketPage::new(props, true, true);

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

    let (ticket, project, project_member, members, parent, children) =
        match model::ticket::Ticket::find_ticket_and_project(&id, &session.uid, &db).await {
            Ok(ticket) => ticket,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    let mut can_update = false;
    let mut can_delete = false;

    // チケットを更新できるのは、作成者、担当者、オーナー、管理者
    // チケットを削除できるのは、作成者、オーナー、管理者
    if let Some(pmem) = project_member {
        if let Some(r) = pmem.role {
            if r == model::project::ProjectRole::Owner as i32
                || r == model::project::ProjectRole::Administrator as i32
            {
                can_update = true;
                can_delete = true;
            }
        }
    }
    if can_delete == false {
        if let Some(t) = &ticket {
            if let Some(o) = &t.owner {
                if o == &session.uid {
                    can_update = true;
                    can_delete = true;
                }
            }
            let member = members.iter().find(|m| m.uid == session.uid);
            if member.is_some() {
                can_update = true;
            }
        }
    }

    props.ticket = ticket;
    props.project = project;
    props.ticket_members = members;
    props.ticket_parent = parent;
    props.ticket_children = children;

    props.session = Some(session);
    let mut page = TicketPage::new(props, can_update, can_delete);

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

    let mut ticket_members = Vec::new();
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let mem = members["members"].as_array().unwrap_or_else(|| &empty_vec);

    let mut i = 0;
    for m in mem {
        let mut member = model::ticket::TicketMember::new(
            "",
            &input.ticket_id,
            &input.project_id,
            m["uid"].as_str().unwrap(),
        );

        member.email = Some(String::from(m["email"].as_str().unwrap()));
        member.name = Some(String::from(m["name"].as_str().unwrap()));
        member.seq = i;
        ticket_members.push(member);
        i += 1;
    }

    let (validation, project, project_member, ticket) =
        match validation::ticket::TicketValidation::validate_post(
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

    if project.is_none() {
        return Ok(Html(LoginPage::write()));
    }

    if let Some(v) = validation {
        let mut can_update = false;
        let mut can_delete = false;

        // チケットを更新できるのは、作成者、担当者、オーナー、管理者
        // チケットを削除できるのは、作成者、オーナー、管理者
        if action == crate::Action::Create {
            can_update = true;
            can_delete = true;
        } else {
            if let Some(pmem) = project_member {
                if let Some(r) = pmem.role {
                    if r == model::project::ProjectRole::Owner as i32
                        || r == model::project::ProjectRole::Administrator as i32
                    {
                        can_update = true;
                        can_delete = true;
                    }
                }
            }

            if can_delete == false {
                if let Some(t) = &ticket {
                    if let Some(o) = &t.owner {
                        if o == &session.uid {
                            can_update = true;
                            can_delete = true;
                        }
                    }
                    let member = ticket_members.iter().find(|m| m.uid == session.uid);
                    if member.is_some() {
                        can_update = true;
                    }
                }
            }
        }

        let mut ticket_new = model::ticket::Ticket::new("", &input.project_id);
        if let Some(t) = ticket {
            ticket_new.id = t.id;
            ticket_new.owner = t.owner;
            ticket_new.created_at = t.created_at;
            ticket_new.updated_at = t.updated_at;
        }
        ticket_new.name = Some(input.name);
        ticket_new.description = Some(input.description);
        ticket_new.start_date = Some(input.start_date);
        ticket_new.end_date = Some(input.end_date);
        ticket_new.progress = input.progress.parse::<i32>().unwrap_or_default();
        ticket_new.priority = input.priority.parse::<i32>().unwrap_or_default();

        props.session = Some(session);
        props.action = action;
        props.project = project;
        props.ticket = Some(ticket_new);
        props.ticket_validation = Some(v);
        props.ticket_members = ticket_members;

        let mut page = TicketPage::new(props, can_update, can_delete);
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
        crate::Action::Delete => {
            // チケット削除
            if let Err(e) = model::ticket::Ticket::delete(&input, &db).await {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        }
        _ => {}
    }

    return super::home::show_home(session, &db).await;
}

#[derive(Deserialize, Debug)]
pub struct NoteInput {
    pub project_id: String,
    pub markdown: String,
    pub ticket_id: String,
    pub timestamp: String,
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

    let (validation, project, project_member, ticket) =
        match validation::ticket::TicketValidation::validate_post_note(&input, &session, &db).await
        {
            Ok(v) => v,
            Err(e) => {
                return Err(AppError(e));
            }
        };

    if project.is_none() {
        return Ok(Html(LoginPage::write()));
    }

    let mut props = page::Props::new(&session.id);

    if let Some(v) = validation {
        let mut can_update = false;
        let mut can_delete = false;
        // チケットを更新できるのは、作成者、担当者、オーナー、管理者
        // チケットを削除できるのは、作成者、オーナー、管理者
        if let Some(pmem) = &project_member {
            if let Some(r) = pmem.role {
                if r == model::project::ProjectRole::Owner as i32
                    || r == model::project::ProjectRole::Administrator as i32
                {
                    can_update = true;
                    can_delete = true;
                }
            }
        }

        if can_delete == false {
            if let Some(t) = &ticket {
                if let Some(o) = &t.owner {
                    if o == &session.uid {
                        can_update = true;
                        can_delete = true;
                    }
                }
            }

            if let Some(m) = &project_member {
                if let Some(r) = m.role {
                    if r == model::project::ProjectRole::Owner as i32
                        || r == model::project::ProjectRole::Administrator as i32
                    {
                        can_update = true;
                    }
                }
            }
        }

        if let Some(t) = ticket {
            let mut t = t.clone();
            t.note = Some(input.markdown);
            props.ticket = Some(t);
        }

        props.tab = crate::Tab::Note;
        props.session = Some(session);
        props.action = crate::Action::Update;
        props.project = project;
        props.ticket_validation = Some(v);

        let mut page = TicketPage::new(props, can_update, can_delete);
        return Ok(Html(page.write()));
    }

    match model::ticket::Ticket::update_note(&input, &session, &db).await {
        Ok(t) => t,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    return super::home::show_home(session, &db).await;
}

#[derive(Deserialize, Debug)]
pub struct TicketListInput {
    pub ticketid: String,
    pub ticketname: String,
    pub parentid: String,
    pub finished: Option<String>,
}

pub async fn get_list(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /ticket_list");

    let input = TicketListInput {
        ticketid: String::from(""),
        ticketname: String::from(""),
        parentid: String::from(""),
        finished: None,
    };

    return get_list_sub(cookies, input).await;
}

pub async fn post_list(
    cookies: Cookies,
    Form(input): Form<TicketListInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /post_list {:?}, {:?}, {:?}, {:?}",
        input.ticketid,
        input.ticketname,
        input.parentid,
        input.finished
    );

    return get_list_sub(cookies, input).await;
}

async fn get_list_sub(cookies: Cookies, input: TicketListInput) -> Result<Html<String>, AppError> {
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

    let (project, member) = match model::project::Project::current_project(&session, &db).await {
        Ok((project, member)) => (project, member),
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    if let Some(project) = &project {
        let tickets = match model::ticket::Ticket::search_list(&project.id, &input, &db).await {
            Ok(tickets) => tickets,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };
        props.tickets = tickets;
    }

    props.session = Some(session);
    props.project = project;
    props.project_member = member;

    let mut page = TicketListPage::new(props, input);

    Ok(Html(page.write()))
}
