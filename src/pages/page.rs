use crate::{
    components::{head::Head, Component},
    model,
};

// buffer size for outputting HTML content.
// specify a sufficient size according to the characteristics of the system.
const PAGE_BUFFER_SIZE: usize = 20_000;

pub struct Props {
    pub session_id: String,
    pub session: Option<model::session::Session>,
    pub project: Option<model::project::Project>,
    pub member: Option<model::project::ProjectMember>,
    pub project_validation: Option<model::project::ProjectValidation>,
    //pub projects: Vec<model::project::Project>,
    pub ticket: Option<model::ticket::Ticket>,
    pub ticket_validation: Option<model::ticket::TicketValidation>,
    pub tickets: Vec<model::ticket::Ticket>,
}

impl Props {
    pub fn new(session_id: &String) -> Self {
        Props {
            session_id: session_id.clone(),
            session: None,
            project: None,
            member: None,
            project_validation: None,
            //projects: Vec::new(),
            ticket: None,
            ticket_validation: None,
            tickets: Vec::new(),
        }
    }
}

pub struct Page {
    pub head: Box<dyn Component + Send>,
    pub body: Option<Box<dyn Component + Send>>,
}

impl Page {
    pub fn new() -> Page {
        Page {
            head: Box::new(Head {}),
            body: None,
        }
    }

    // output HTML content to buffer.
    pub fn write(&mut self, props: &Props) -> String {
        // buffer for outputting HTML content.
        let mut buf = String::with_capacity(PAGE_BUFFER_SIZE);

        buf += r#"<!DOCTYPE html>"#;
        buf += r#"<html lang="ja">"#;
        {
            self.head.write(props, &mut buf);

            if let Some(body) = &self.body {
                body.write(props, &mut buf);
            }
        }
        buf += r#"</html>"#;

        buf
    }
}
