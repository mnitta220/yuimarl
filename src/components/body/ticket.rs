use super::super::Component;
use super::parts::{
    nav::Nav, ticket_comment::TicketComment, ticket_history::TicketHistory,
    ticket_info::TicketInfo, ticket_note::TicketNote,
};
use crate::{components::footer::footer::Footer, handlers::validation, Props, Tab};

pub struct TicketBody {
    pub nav: Box<dyn Component + Send>,
    pub ticket_info: Box<dyn Component + Send>,
    pub ticket_note: Box<dyn Component + Send>,
    pub ticket_comment: Box<dyn Component + Send>,
    pub ticket_history: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub validation: Option<validation::ticket::TicketValidation>,
}

impl TicketBody {
    pub fn new(
        can_update: bool,
        can_delete: bool,
        validation: Option<validation::ticket::TicketValidation>,
    ) -> Self {
        TicketBody {
            nav: Box::new(Nav {}),
            ticket_info: Box::new(TicketInfo {
                can_update,
                can_delete,
                validation: validation.clone(),
            }),
            ticket_note: Box::new(TicketNote { can_update }),
            ticket_comment: Box::new(TicketComment { can_update }),
            ticket_history: Box::new(TicketHistory {}),
            footer: Box::new(Footer {}),
            validation,
        }
    }
}

impl Component for TicketBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="py-3">"#;
                {
                    *buf += r#"<div class="container">"#;
                    {
                        *buf += r#"<h3 class="mb-3">チケット"#;
                        if props.action == crate::Action::Create {
                            *buf += r#"を作成"#;
                        }
                        *buf += r#"</h3>"#;

                        if let Some(v) = &self.validation {
                            if let Some(e) = &v.info {
                                *buf += r#"<div class="row p-2">"#;
                                {
                                    *buf += r#"<div class="alert alert-danger text-start" role="alert">"#;
                                    *buf += e;
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                        }

                        if props.action != crate::Action::Create {
                            if let Some(t) = &props.ticket {
                                *buf += r#"<div class="pb-2">"#;
                                {
                                    *buf += r#"<ul class="nav nav-tabs">"#;
                                    {
                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::Info {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/ticket?id="#;
                                            *buf += &t.id;
                                            *buf += r#"&tab=info">基本情報</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::Note {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/ticket?id="#;
                                            *buf += &t.id;
                                            *buf += r#"&tab=note">ノート</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::Comment {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/ticket?id="#;
                                            *buf += &t.id;
                                            *buf += r#"&tab=comment">コメント</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::History {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/ticket?id="#;
                                            *buf += &t.id;
                                            *buf += r#"&tab=history">更新履歴</a>"#;
                                        }
                                        *buf += r#"</li>"#;
                                    }
                                    *buf += r#"</ul>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                        }

                        match &props.tab {
                            Tab::Info => {
                                self.ticket_info.write(props, buf);
                            }
                            Tab::Note => {
                                self.ticket_note.write(props, buf);
                            }
                            Tab::Comment => {
                                self.ticket_comment.write(props, buf);
                            }
                            Tab::History => {
                                self.ticket_history.write(props, buf);
                            }
                            _ => {}
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;
            *buf += r#"<input type="hidden" id="project_id" name="project_id" value=""#;
            if let Some(p) = &props.project {
                *buf += &p.id;
            }
            *buf += r#"">"#;

            self.footer.write(props, buf);

            match &props.tab {
                Tab::Info => {
                    crate::components::footer::ticket_info::ticket_info_footer(buf);
                }
                Tab::Comment => {
                    crate::components::footer::ticket_comment::ticket_comment_footer(buf);
                }
                _ => {}
            }
        }
        *buf += r#"</body>"#;
    }
}
