use super::super::Component;
use super::parts::{nav::Nav, news::News};
use crate::{components::footer::footer::Footer, model, Props};

pub struct HomeBody {
    pub nav: Box<dyn Component + Send>,
    pub news: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub owner_cnt: usize,
    pub memo: Option<String>,
}

impl HomeBody {
    pub fn new(owner_cnt: usize, memo: Option<String>) -> Self {
        HomeBody {
            nav: Box::new(Nav {}),
            news: Box::new(News {}),
            footer: Box::new(Footer {}),
            owner_cnt,
            memo,
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
                                                        *buf += &p.id;
                                                        *buf += r#"">"#;
                                                        {
                                                            super::super::escape_html(n, buf);
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
                    *buf += r#"<div class="pt-3">"#;
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
                                                        *buf += r#"<div class="row mb-0">"#;
                                                        {
                                                            *buf += r#"<div class="col box-"#;
                                                            match &ticket.color {
                                                                Some(c) => {
                                                                    *buf += c;
                                                                }
                                                                None => {
                                                                    *buf += "light";
                                                                }
                                                            };
                                                            *buf += r#"">"#;
                                                            {
                                                                *buf += r#"<a href="/ticket?id="#;
                                                                *buf += &ticket.id;
                                                                *buf += r#"">"#;
                                                                {
                                                                    *buf += &ticket
                                                                        .id_disp
                                                                        .clone()
                                                                        .unwrap();
                                                                }
                                                                *buf += r#"</a>&nbsp;:&nbsp;"#;

                                                                if let Some(name) = &ticket.name {
                                                                    super::super::escape_html(
                                                                        name, buf,
                                                                    );
                                                                }
                                                            }
                                                            *buf += r#"</div>"#;
                                                        }
                                                        *buf += r#"</div>"#;
                                                    }

                                                    *buf += r#"<div class="row mt-2">"#;
                                                    {
                                                        *buf += r#"<div class="col">"#;
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
                                                        *buf += r#"</div>"#;
                                                    }
                                                    *buf += r#"</div>"#;
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

                *buf += r#"<form id="post_memo" action="/post_memo" method="POST">"#;
                {
                    // メモ
                    *buf += r#"<div class="pt-3 mb-2 bg-light">"#;
                    {
                        *buf += r#"<section>"#;
                        {
                            *buf += r#"<div class="container">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-md-3">"#;
                                    {
                                        *buf += r#"<h3>メモ</h3>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="col-md-9 py-0">"#;
                                    {
                                        if let Some(m) = &self.memo {
                                            if m.trim().len() > 0 {
                                                *buf += r#"<div class="row pb-1">"#;
                                                {
                                                    *buf += r#"<div class="col px-2 mx-2 bg-light preview2" id="preview1"></div>"#;
                                                }
                                                *buf += r#"</div>"#;
                                            }
                                        }

                                        *buf += r#"<div class="row pb-1">"#;
                                        {
                                            *buf += r#"<div class="col">"#;
                                            {
                                                *buf += r#"<img class="icon3" style="cursor:pointer" id="icnEditMemo" src="/static/ionicons/create-outline3.svg" title="編集">"#;
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

                    *buf += r#"<div class="modal fade" id="memoModal" tabindex="-1" aria-labelledby="memoModalLabel" aria-hidden="true">"#;
                    {
                        *buf += r#"<div class="modal-dialog modal-xl">"#;
                        {
                            *buf += r#"<div class="modal-content">"#;
                            {
                                *buf += r#"<div class="modal-header">"#;
                                {
                                    *buf += r#"<h1 class="modal-title fs-5" id="memoModalLabel">メモ入力</h1>"#;
                                    *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="modal-body">"#;
                                {
                                    *buf += r#"<div class="row py-2" id="note1">"#;
                                    {
                                        *buf += r#"<div class="col-lg-6">"#;
                                        {
                                            *buf += r#"<small>［マークダウン］</small>"#;
                                            *buf += r#"<textarea class="form-control" id="markdown" name="markdown" rows="10" required>"#;
                                            if let Some(m) = &self.memo {
                                                *buf += &m;
                                            }
                                            *buf += r#"</textarea>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="col-lg-6">"#;
                                        {
                                            *buf += r#"<small>［プレビュー］</small>"#;
                                            *buf += r#"<div class="px-2 bg-light preview" id="preview2"></div>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="modal-footer">"#;
                                {
                                    *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                                    *buf += r#"<button id="btnUpdate" class="btn btn-primary" type="button">"#;
                                    {
                                        *buf += r#"<img class="icon" src="/static/ionicons/save-outline.svg">&nbsp;更新"#;
                                    }
                                    *buf += r#"</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
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
