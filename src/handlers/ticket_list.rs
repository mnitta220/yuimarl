use crate::{
    model,
    pages::{
        login_page::LoginPage,
        page,
        ticket_list_page::{TicketListPage, PAGE_COUNT},
    },
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize, Debug)]
pub struct TicketListInput {
    pub ticketid: String,
    pub ticketname: String,
    pub parentid: String,
    pub chargeuid: String,
    pub chargemail: String,
    pub chargename: String,
    pub finished: Option<String>,
    pub priorityorder: Option<String>,
    pub page: usize,
}

pub struct TicketListProps {
    pub current_page: usize,
    pub total_page: usize,
    pub data_count: usize,
}

pub async fn get_list(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /ticket_list");

    let input = TicketListInput {
        ticketid: String::from(""),
        ticketname: String::from(""),
        parentid: String::from(""),
        chargeuid: String::from(""),
        chargemail: String::from(""),
        chargename: String::from(""),
        finished: None,
        priorityorder: None,
        page: 1,
    };

    return get_list_sub(cookies, input).await;
}

pub async fn post_list(
    cookies: Cookies,
    Form(input): Form<TicketListInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /post_list {:?}, {:?}, {:?}, {:?}, {:?}, {}, {}",
        input.ticketid,
        input.ticketname,
        input.parentid,
        input.finished,
        input.priorityorder,
        input.chargeuid,
        input.page
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
    let mut props = page::Props::new();
    props.title = Some("チケット一覧".to_string());

    let (project, member) = match model::project::Project::current_project(&session, &db).await {
        Ok((project, member)) => (project, member),
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let mut list_props = TicketListProps {
        current_page: input.page,
        total_page: 1,
        data_count: 0,
    };

    if let Some(project) = &project {
        let tickets = match model::ticket::Ticket::search_list(&project.id, &input, &db).await {
            Ok(tickets) => tickets,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };
        list_props.data_count = tickets.len();
        list_props.total_page = (list_props.data_count + PAGE_COUNT - 1) / PAGE_COUNT;
        props.tickets = tickets;
    }

    props.session = Some(session);
    props.project = project;
    props.project_member = member;
    props.screen = Some(crate::Screen::TicketList);

    let mut page = TicketListPage::new(props, input, list_props);

    Ok(Html(page.write()))
}
