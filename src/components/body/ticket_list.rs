use super::super::super::handlers::ticket::TicketListInput;
use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct TicketListBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub input: TicketListInput,
}

impl TicketListBody {
    pub fn new(input: TicketListInput) -> Self {
        TicketListBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
            input,
        }
    }
}

impl Component for TicketListBody {
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
                        if let Some(p) = &props.project {
                            *buf += r#"<div class="row">"#;
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

                            *buf += r#"<div class="row pt-3">"#;
                            {
                                *buf += r#"<h3>チケット一覧</h3>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<form class="row py-2 mx-1 border bg-light" action="/ticket_list" method="POST">"#;
                            {
                                *buf += r#"<div class="col-md-2 pb-1">"#;
                                {
                                    *buf += r#"<label class="form-label" for="ticketid">チケットID</label>"#;
                                    *buf += r#"<input class="form-control" id="ticketid" name="ticketid" type="text" maxlength="20" value=""#;
                                    *buf += &self.input.ticketid;
                                    *buf += r#"">"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-md-5 pb-1">"#;
                                {
                                    *buf += r#"<label class="form-label" for="ticketname">チケット名&nbsp;&nbsp;<small>(部分一致)</small></label>"#;
                                    *buf += r#"<input class="form-control" id="ticketname" name="ticketname" type="text" maxlength="100" value=""#;
                                    *buf += &self.input.ticketname;
                                    *buf += r#"">"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-md-2 pb-1">"#;
                                {
                                    *buf += r#"<label class="form-label" for="parentid">親チケットID</label>"#;
                                    *buf += r#"<input class="form-control" id="parentid" name="parentid" type="text" maxlength="20" value=""#;
                                    *buf += &self.input.parentid;
                                    *buf += r#"">"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-md-3 pb-1">"#;
                                {
                                    *buf += r#"<div class="pb-2">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="finished" name="finished" type="checkbox""#;
                                        if let Some(f) = &self.input.finished {
                                            if f == "on" || f == "true" {
                                                *buf += r#" checked="checked""#;
                                            }
                                        }
                                        *buf += r#">"#;
                                        *buf += r#"<label class="form-check-label" for="finished">完了済を表示</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="text-end">"#;
                                    {
                                        *buf += r#"<button class="btn btn-sm btn-primary" type="submit">"#;
                                        {
                                            *buf += r#"<img class="icon" src="/static/ionicons/funnel-outline.svg">&nbsp;フィルター"#;
                                        }
                                        *buf += r#"</button>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</form>"#;

                            *buf += r#"<div class="row pt-2">"#;
                            {
                                *buf += r#"<div class="col-md-9"></div>"#;
                                *buf += r#"<div class="col-md-3">"#;
                                {
                                    *buf += r#"<nav>"#;
                                    {
                                        *buf += r#"<ul class="pagination pagination-sm">"#;
                                        {
                                            *buf += r#"<li class="page-item disabled">"#;
                                            {
                                                *buf += r#"<a class="page-link" href="" tabindex="-1" aria-label="前へ">"#;
                                                {
                                                    *buf += r#"<span aria-hidden="true">&laquo;</span>"#;
                                                    *buf += r#"<span class="visually-hidden">前へ</span>"#;
                                                }
                                                *buf += r#"</a>"#;
                                            }
                                            *buf += r#"</li>"#;

                                            *buf += r#"<li class="page-item">"#;
                                            {
                                                *buf += r#"<a class="page-link active" href="">"#;
                                                {
                                                    *buf += r#"1"#;
                                                    *buf += r#"<span class="visually-hidden">(現ページ)</span>"#;
                                                }
                                                *buf += r#"</a>"#;
                                            }
                                            *buf += r#"</li>"#;

                                            *buf += r#"<li class="page-item">"#;
                                            {
                                                *buf += r#"<a class="page-link" href="">2</a>"#;
                                            }
                                            *buf += r#"</li>"#;

                                            *buf += r#"<li class="page-item">"#;
                                            {
                                                *buf += r#"<a class="page-link" href="">3</a>"#;
                                            }
                                            *buf += r#"</li>"#;

                                            *buf += r#"<li class="page-item">"#;
                                            {
                                                *buf += r#"<a class="page-link" href="" aria-label="次へ">"#;
                                                {
                                                    *buf += r#"<span aria-hidden="true">&raquo;</span>"#;
                                                    *buf += r#"<span class="visually-hidden">次へ</span>"#;
                                                }
                                                *buf += r#"</a>"#;
                                            }
                                            *buf += r#"</li>"#;
                                        }
                                        *buf += r#"</ul>"#;
                                    }
                                    *buf += r#"</nav>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row">"#;
                            {
                                *buf += r#"<div class="col">"#;
                                {
                                    *buf += r#"<div class="table-responsive">"#;
                                    {
                                        *buf += r#"<table class="table table-sm table-hover">"#;
                                        {
                                            *buf += r#"<thead>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf +=
                                                        r#"<th scope="col">ID / チケット名</th>"#;
                                                    *buf += r#"<th scope="col">親チケット</th>"#;
                                                    *buf += r#"<th scope="col">開始日</th>"#;
                                                    *buf += r#"<th scope="col">終了日</th>"#;
                                                    *buf += r#"<th scope="col">優先度</th>"#;
                                                    *buf += r#"<th class="text-right" scope="col">進捗率</th>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</thead>"#;

                                            *buf += r#"<tbody>"#;
                                            {
                                                for ticket in &props.tickets {
                                                    *buf += r#"<tr>"#;
                                                    {
                                                        *buf += r#"<td>"#;
                                                        {
                                                            *buf += r#"<a href="/ticket?id="#;
                                                            *buf += &ticket.id.clone().unwrap();
                                                            *buf += r#"">"#;
                                                            *buf += &ticket
                                                                .id_disp
                                                                .clone()
                                                                .unwrap_or_default();
                                                            *buf += r#"</a>&nbsp;"#;
                                                            *buf += &ticket
                                                                .name
                                                                .clone()
                                                                .unwrap_or_default();
                                                        }
                                                        *buf += r#"</td>"#;

                                                        *buf += r#"<td>"#;
                                                        {
                                                            if let Some(ref parent_id) =
                                                                ticket.parent_id
                                                            {
                                                                *buf += r#"<a href="/ticket?id="#;
                                                                *buf += parent_id;
                                                                *buf += r#"">"#;
                                                                *buf += &ticket
                                                                    .parent_id_disp
                                                                    .clone()
                                                                    .unwrap_or_default();
                                                                *buf += r#"</a>&nbsp;"#;
                                                                *buf += &ticket
                                                                    .parent_name
                                                                    .clone()
                                                                    .unwrap_or_default();
                                                            }
                                                        }
                                                        *buf += r#"</td>"#;

                                                        *buf += r#"<td>"#;
                                                        if let Some(s) = &ticket.start_date {
                                                            super::super::replace_slash(&s, buf);
                                                        }
                                                        *buf += r#"</td>"#;

                                                        *buf += r#"<td>"#;
                                                        if let Some(e) = &ticket.end_date {
                                                            super::super::replace_slash(&e, buf);
                                                        }
                                                        *buf += r#"</td>"#;

                                                        *buf += r#"<td>"#;
                                                        match &ticket.priority {
                                                            4 => *buf += r#"最優先"#,
                                                            3 => *buf += r#"高"#,
                                                            2 => *buf += r#"中"#,
                                                            1 => *buf += r#"低"#,
                                                            _ => *buf += r#""#,
                                                        }
                                                        *buf += r#"</td>"#;

                                                        *buf += r#"<td class="text-right">"#;
                                                        *buf += &ticket.progress.to_string();
                                                        *buf += r#"%</td>"#;
                                                    }
                                                    *buf += r#"</tr>"#;
                                                }
                                            }
                                            *buf += r#"</tbody>"#;
                                        }
                                        *buf += r#"</table>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row">"#;
                            {
                                *buf += r#"<div class="col">"#;
                                {
                                    *buf += r#"<a href="/ticket_add" title="チケットを追加">"#;
                                    {
                                        *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg">"#;
                                    }
                                    *buf += r#"</a>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        } else {
                            *buf += r#"<p>プロジェクトが登録されていません。</p>"#;
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
        }
        *buf += r#"</body>"#;
    }
}
