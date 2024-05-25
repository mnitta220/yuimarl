use super::super::Component;
use super::parts::{footer::Footer, nav::Nav, ticket_info::TicketInfo};
use crate::Props;
use crate::Tab;

pub struct TicketBody {
    pub nav: Box<dyn Component + Send>,
    pub ticket_info: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl TicketBody {
    pub fn new() -> Self {
        TicketBody {
            nav: Box::new(Nav {}),
            ticket_info: Box::new(TicketInfo {}),
            footer: Box::new(Footer {}),
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

                        if props.action != crate::Action::Create {
                            *buf += r#"<div class="pb-2">"#;
                            {
                                *buf += r#"<ul class="nav nav-tabs">"#;
                                {
                                    *buf += r#"<li class="nav-item"><a class="nav-link active" href="">基本情報</a></li>"#;
                                    *buf += r#"<li class="nav-item"><a class="nav-link" href="">ノート</a></li>"#;
                                    *buf += r#"<li class="nav-item"><a class="nav-link" href="">コメント</a></li>"#;
                                    *buf += r#"<li class="nav-item"><a class="nav-link" href="">更新履歴</a></li>"#;
                                }
                                *buf += r#"</ul>"#;
                            }
                            *buf += r#"</div>"#;
                        }

                        match &props.tab {
                            Tab::Info => {
                                self.ticket_info.write(props, buf);
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
                if let Some(id) = &p.id {
                    *buf += id;
                }
            }
            *buf += r#"">"#;

            self.footer.write(props, buf);
            *buf += r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
            match &props.tab {
                Tab::Info => {
                    *buf += r#"<script src="/static/js/ticket0014.js"></script>"#;
                }
                _ => {}
            }
        }
        *buf += r#"</body>"#;
    }
}
