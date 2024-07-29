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
    pub title: Option<String>,
    pub project: Option<model::project::Project>,
    pub project_member: Option<model::project::ProjectMember>,
    pub project_members: Vec<model::project::ProjectMember>,
    pub ticket: Option<model::ticket::Ticket>,
    pub ticket_members: Vec<model::ticket::TicketMember>,
    pub tickets: Vec<model::ticket::Ticket>,
    pub ticket_parent: Option<model::ticket::Ticket>,
    pub ticket_children: Vec<model::ticket::Ticket>,
    pub ticket_comments: Vec<model::comment::Comment>,
    pub gantt_tickets: Vec<model::gantt_ticket::GanttTicket>,
    pub gantt_start: Option<String>,
    pub gantt_end: Option<String>,
    pub news: Vec<model::news::News>,
    pub tab: Tab,
    pub action: crate::Action,
}

impl Props {
    pub fn new(session_id: &String) -> Self {
        Props {
            session_id: session_id.clone(),
            session: None,
            title: None,
            project: None,
            project_member: None,
            project_members: Vec::new(),
            ticket: None,
            ticket_members: Vec::new(),
            tickets: Vec::new(),
            ticket_parent: None,
            ticket_children: Vec::new(),
            ticket_comments: Vec::new(),
            gantt_tickets: Vec::new(),
            gantt_start: None,
            gantt_end: None,
            news: Vec::new(),
            tab: Tab::ProjectInfo,
            action: crate::Action::Read,
        }
    }
}

/// プロジェクト画面のタブ
#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    ProjectInfo = 1, // プロジェクト基本情報
    TicketInfo = 2,  // チケット基本情報
    Note = 3,        // ノート
    History = 4,     // 更新履歴
    Comment = 5,     // コメント
    GanttChart = 6,  // ガントチャート
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
