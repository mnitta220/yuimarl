use super::super::Component;
use super::parts::{footer::Footer, nav::Nav, news::News};
use crate::{model, Props};

pub struct HomeBody {
    pub nav: Box<dyn Component + Send>,
    pub news: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub owner_cnt: usize,
}

impl HomeBody {
    pub fn new(owner_cnt: usize) -> Self {
        HomeBody {
            nav: Box::new(Nav {}),
            news: Box::new(News {}),
            footer: Box::new(Footer {}),
            owner_cnt,
        }
    }
}

impl Component for HomeBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                // お知らせ
                if props.news.len() > 0 {
                    *buf += r#"<div class="pt-3">"#;
                    {
                        *buf += r#"<section id="news">"#;
                        {
                            *buf += r#"<div class="container">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-md-3">"#;
                                    {
                                        *buf += r#"<h3>お知らせ</h3>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="col-md-9 py-0">"#;
                                    {
                                        *buf += r#"<dl class="row">"#;
                                        {
                                            *buf += r#"<div class="col">"#;
                                            {
                                                self.news.write(props, buf);
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</dl>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</section>"#;
                    }
                    *buf += r#"</div>"#;
                }

                *buf += r#"<form action="/ticket/add" method="POST">"#;
                {
                    // プロジェクト
                    *buf += r#"<div class="py-3 bg-light">"#;
                    {
                        *buf += r#"<section id="project">"#;
                        {
                            *buf += r#"<div class="container">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-md-3">"#;
                                    {
                                        *buf += r#"<h3>プロジェクト</h3>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="col-md-9">"#;
                                    {
                                        if let Some(p) = &props.project {
                                            *buf += r#"<div class="row pb-3">"#;
                                            {
                                                *buf += r#"<div class="col">"#;
                                                {
                                                    if let Some(n) = &p.project_name {
                                                        *buf += r#"<a href="/project?id="#;
                                                        *buf += &p.id.clone().unwrap();
                                                        *buf += r#"">"#;
                                                        {
                                                            *buf += n;
                                                        }
                                                        *buf += r#"</a>"#;

                                                        if let Some(m) = &props.project_member {
                                                            *buf += r#"&nbsp;&nbsp;<small>"#;
                                                            {
                                                                *buf += r#"<span class="badge bg-secondary text-light">"#;
                                                                {
                                                                    *buf += &m.role_to_string();
                                                                }
                                                                *buf += r#"</span>"#;
                                                            }
                                                            *buf += r#"</small>"#;
                                                        }
                                                    }
                                                }
                                                *buf += r#"</div>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }

                                        *buf += r#"<div class="row">"#;
                                        {
                                            *buf += r#"<div class="col">"#;
                                            {
                                                *buf += r#"<a href="/project_list" title="プロジェクト一覧">"#;
                                                {
                                                    *buf += r#"<img class="icon3" src="/static/ionicons/list-outline.svg">"#;
                                                }
                                                *buf += r#"</a>"#;
                                                if let None = &props.project {
                                                    *buf += r#"<span class="btn-help">&nbsp;: プロジェクト一覧</span>"#;
                                                }
                                                *buf += r#"&nbsp;&nbsp;&nbsp;"#;

                                                if self.owner_cnt < 10 {
                                                    *buf += r#"<a href="/project_add" title="プロジェクトを作成">"#;
                                                    {
                                                        *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg">"#;
                                                    }
                                                    *buf += r#"</a>"#;
                                                    if let None = &props.project {
                                                        *buf += r#"<span class="btn-help">&nbsp;: プロジェクトを作成</span>"#;
                                                    }
                                                }
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</section>"#;
                    }
                    *buf += r#"</div>"#;

                    // チケット
                    *buf += r#"<div class="py-3">"#;
                    {
                        *buf += r#"<section id="tickets" class="mb-3">"#;
                        {
                            *buf += r#"<div class="container">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-md-3">"#;
                                    {
                                        *buf += r#"<h3>チケット</h3>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="col-md-9">"#;
                                    {
                                        if let Some(project) = &props.project {
                                            if let Some(m) = &props.project_member {
                                                if let Some(role) = m.role {
                                                    for ticket in props.tickets.iter() {
                                                        *buf += r#"<p class="mb-0">"#;
                                                        {
                                                            *buf += r#"<a href="/ticket?id="#;
                                                            *buf += &ticket.id.clone().unwrap();
                                                            *buf += r#"">"#;
                                                            {
                                                                *buf += &ticket
                                                                    .id_disp
                                                                    .clone()
                                                                    .unwrap();
                                                            }
                                                            *buf += r#"</a>&nbsp;:&nbsp;"#;
                                                            *buf += &ticket.name.clone().unwrap();
                                                        }
                                                        *buf += r#"</p>"#;
                                                    }

                                                    *buf += r#"<p class="mt-2">"#;
                                                    {
                                                        *buf += r#"<a href="/ticket_list">"#;
                                                        {
                                                            *buf += r#"<img class="icon3" src="/static/ionicons/list-outline.svg" title="チケット一覧">"#;
                                                        }
                                                        *buf += r#"</a>"#;

                                                        if role == model::project::ProjectRole::Owner as i32
                                                            || role == model::project::ProjectRole::Administrator as i32
                                                            || role == model::project::ProjectRole::Member as i32
                                                        {
                                                            let ticket_cnt = project.ticket_number.unwrap_or_default();
                                                            let ticket_limit = project.ticket_limit.unwrap_or_default();
                                                            if ticket_cnt < ticket_limit {
                                                                *buf += r#"&nbsp;&nbsp;&nbsp;<a href="/ticket_add">"#;
                                                                {
                                                                    *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="チケットを追加">"#;
                                                                }
                                                                *buf += r#"</a>"#;
                                                            }
                                                        }
                                                    }
                                                    *buf += r#"</p>"#;
                                                }
                                            }
                                        }
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</section>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</form>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
        }
        *buf += r#"</body>"#;
    }
}
