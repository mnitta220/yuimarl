use super::super::super::handlers::ticket_list::{TicketListInput, TicketListProps};
use super::super::super::pages::ticket_list_page::PAGE_COUNT;
use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::{model, Props};

pub struct TicketListBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub input: TicketListInput,
    pub list_props: TicketListProps,
    pub start_pos: usize,
    pub end_pos: usize,
}

impl TicketListBody {
    pub fn new(input: TicketListInput, list_props: TicketListProps) -> Self {
        let start_pos = (list_props.current_page - 1) * PAGE_COUNT;
        let end_pos = start_pos + PAGE_COUNT;

        TicketListBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
            input,
            list_props,
            start_pos,
            end_pos,
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

                            *buf += r#"<div class="row pt-3">"#;
                            {
                                *buf += r#"<h3>チケット一覧</h3>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<form class="row py-2 mx-1 border bg-light" name="form_filter" id="form_filter" action="/ticket_list" method="POST">"#;
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
                                        *buf += r#"<button id="btnFilter" class="btn btn-sm btn-primary" type="button">"#;
                                        {
                                            *buf += r#"<img class="icon" src="/static/ionicons/funnel-outline.svg">&nbsp;フィルター"#;
                                        }
                                        *buf += r#"</button>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                                *buf += r#"<input type="hidden" id="page" name="page" value=""#;
                                *buf += &self.input.page.to_string();
                                *buf += r#"">"#;
                            }
                            *buf += r#"</form>"#;

                            if self.list_props.total_page > 1 {
                                *buf += r#"<div class="row pt-2">"#;
                                {
                                    *buf += r#"<div class="col">"#;
                                    {
                                        *buf += r#"<div class="d-flex justify-content-end px-1">"#;
                                        {
                                            *buf += r#"<nav>"#;
                                            {
                                                *buf += r#"<ul class="pagination pagination-sm">"#;
                                                {
                                                    // 前へ
                                                    *buf += r#"<li class="page-item"#;
                                                    if self.list_props.current_page == 1 {
                                                        *buf += r#" disabled"#;
                                                    }
                                                    *buf += r#"">"#;
                                                    {
                                                        *buf += r#"<a class="page-link" href="javascript:pageChange("#;
                                                        *buf += &(self.list_props.current_page - 1)
                                                            .to_string();
                                                        *buf += r#")" tabindex="-1" aria-label="前へ">"#;
                                                        {
                                                            *buf += r#"<span aria-hidden="true">&laquo;</span>"#;
                                                        }
                                                        *buf += r#"</a>"#;
                                                    }
                                                    *buf += r#"</li>"#;

                                                    // 2ページ前
                                                    if self.list_props.current_page > 2 {
                                                        *buf += r#"<li class="page-item">"#;
                                                        {
                                                            let page =
                                                                self.list_props.current_page - 2;
                                                            *buf += r#"<a class="page-link" href="javascript:pageChange("#;
                                                            *buf += &page.to_string();
                                                            *buf += r#")">"#;
                                                            *buf += &page.to_string();
                                                            *buf += r#"</a>"#;
                                                        }
                                                        *buf += r#"</li>"#;
                                                    }

                                                    // 1ページ前
                                                    if self.list_props.current_page > 1 {
                                                        *buf += r#"<li class="page-item">"#;
                                                        {
                                                            let page =
                                                                self.list_props.current_page - 1;
                                                            *buf += r#"<a class="page-link" href="javascript:pageChange("#;
                                                            *buf += &page.to_string();
                                                            *buf += r#")">"#;
                                                            *buf += &page.to_string();
                                                            *buf += r#"</a>"#;
                                                        }
                                                        *buf += r#"</li>"#;
                                                    }

                                                    // 現ページ
                                                    *buf += r#"<li class="page-item">"#;
                                                    {
                                                        *buf += r#"<a class="page-link active" href="">"#;
                                                        {
                                                            *buf += &self
                                                                .list_props
                                                                .current_page
                                                                .to_string();
                                                        }
                                                        *buf += r#"</a>"#;
                                                    }
                                                    *buf += r#"</li>"#;

                                                    // 1ページ次
                                                    if (self.list_props.current_page + 1)
                                                        <= self.list_props.total_page
                                                    {
                                                        *buf += r#"<li class="page-item">"#;
                                                        {
                                                            let page =
                                                                self.list_props.current_page + 1;
                                                            *buf += r#"<a class="page-link" href="javascript:pageChange("#;
                                                            *buf += &page.to_string();
                                                            *buf += r#")">"#;
                                                            *buf += &page.to_string();
                                                            *buf += r#"</a>"#;
                                                        }
                                                        *buf += r#"</li>"#;
                                                    }

                                                    // 2ページ次
                                                    if (self.list_props.current_page + 2)
                                                        <= self.list_props.total_page
                                                    {
                                                        *buf += r#"<li class="page-item">"#;
                                                        {
                                                            let page =
                                                                self.list_props.current_page + 2;
                                                            *buf += r#"<a class="page-link" href="javascript:pageChange("#;
                                                            *buf += &page.to_string();
                                                            *buf += r#")">"#;
                                                            *buf += &page.to_string();
                                                            *buf += r#"</a>"#;
                                                        }
                                                        *buf += r#"</li>"#;
                                                    }

                                                    // 次へ
                                                    *buf += r#"<li class="page-item"#;
                                                    if self.list_props.current_page
                                                        == self.list_props.total_page
                                                    {
                                                        *buf += r#" disabled"#;
                                                    }
                                                    *buf += r#"">"#;
                                                    {
                                                        *buf += r#"<a class="page-link" href="javascript:pageChange("#;
                                                        *buf += &(self.list_props.current_page + 1)
                                                            .to_string();
                                                        *buf += r#")" aria-label="次へ">"#;
                                                        {
                                                            *buf += r#"<span aria-hidden="true">&raquo;</span>"#;
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
                                }
                                *buf += r#"</div>"#;
                            }

                            *buf += r#"<div class="row pt-3">"#;
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
                                                let mut i = self.start_pos;
                                                loop {
                                                    if i >= self.end_pos {
                                                        break;
                                                    }
                                                    let ticket = &props.tickets.get(i);
                                                    if let Some(ticket) = ticket {
                                                        *buf += r#"<tr>"#;
                                                        {
                                                            *buf += r#"<td>"#;
                                                            {
                                                                *buf += r#"<a href="/ticket?id="#;
                                                                *buf += &ticket.id;
                                                                *buf += r#"">"#;
                                                                *buf += &ticket
                                                                    .id_disp
                                                                    .clone()
                                                                    .unwrap_or_default();
                                                                *buf += r#"</a>&nbsp;"#;
                                                                if let Some(ref name) = ticket.name
                                                                {
                                                                    super::super::escape_html(
                                                                        &name, buf,
                                                                    );
                                                                }
                                                            }
                                                            *buf += r#"</td>"#;

                                                            *buf += r#"<td>"#;
                                                            {
                                                                if let Some(ref parent_id) =
                                                                    ticket.parent_id
                                                                {
                                                                    *buf +=
                                                                        r#"<a href="/ticket?id="#;
                                                                    *buf += parent_id;
                                                                    *buf += r#"">"#;
                                                                    *buf += &ticket
                                                                        .parent_id_disp
                                                                        .clone()
                                                                        .unwrap_or_default();
                                                                    *buf += r#"</a>&nbsp;"#;
                                                                    if let Some(ref name) =
                                                                        ticket.parent_name
                                                                    {
                                                                        super::super::escape_html(
                                                                            &name, buf,
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                            *buf += r#"</td>"#;

                                                            *buf += r#"<td>"#;
                                                            if let Some(s) = &ticket.start_date {
                                                                super::super::replace_slash(
                                                                    &s, buf,
                                                                );
                                                            }
                                                            *buf += r#"</td>"#;

                                                            *buf += r#"<td>"#;
                                                            if let Some(e) = &ticket.end_date {
                                                                super::super::replace_slash(
                                                                    &e, buf,
                                                                );
                                                            }
                                                            *buf += r#"</td>"#;

                                                            *buf += r#"<td>"#;
                                                            *buf += &ticket.priority_to_string();
                                                            *buf += r#"</td>"#;

                                                            *buf += r#"<td class="text-right">"#;
                                                            *buf += &ticket.progress.to_string();
                                                            *buf += r#"%</td>"#;
                                                        }
                                                        *buf += r#"</tr>"#;
                                                    } else {
                                                        break;
                                                    }
                                                    i += 1;
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

                            if let Some(m) = &props.project_member {
                                if let Some(r) = m.role {
                                    if r == model::project::ProjectRole::Owner as i32
                                        || r == model::project::ProjectRole::Administrator as i32
                                        || r == model::project::ProjectRole::Member as i32
                                    {
                                        let ticket_cnt = p.ticket_number.unwrap_or_default();
                                        let ticket_limit = p.ticket_limit.unwrap_or_default();
                                        if ticket_cnt < ticket_limit {
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
                                        }
                                    }
                                }
                            }
                        } else {
                            *buf += r#"<p>プロジェクトが登録されていません。</p>"#;
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;

            *buf += r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
            *buf += r#"<script src="/static/js/ticket_list0040.js"></script>"#;

            self.footer.write(props, buf);
        }
        *buf += r#"</body>"#;
    }
}
