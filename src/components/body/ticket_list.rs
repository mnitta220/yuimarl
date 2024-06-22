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

                                    *buf += r#"<input type="hidden" id="project_id" name="project_id" value=""#;
                                    *buf += &p.id;
                                    *buf += r#"">"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row pt-3">"#;
                            {
                                *buf += r#"<h3>チケット一覧</h3>"#;
                            }
                            *buf += r#"</div>"#;

                            // フィルターフォーム
                            *buf += r#"<form class="row py-2 mx-1 border bg-light" name="form_filter" id="form_filter" action="/ticket_list" method="POST">"#;
                            {
                                *buf += r#"<div class="col-lg-3">"#;
                                {
                                    // チケットID
                                    *buf += r#"<div class="row pb-1">"#;
                                    {
                                        *buf += r#"<label class="col-lg-6 col-form-label" for="ticketid">チケットID</label>"#;
                                        *buf += r#"<div class="col-lg-6">"#;
                                        {
                                            *buf += r#"<input class="form-control" id="ticketid" name="ticketid" type="text" maxlength="20" value=""#;
                                            *buf += &self.input.ticketid;
                                            *buf += r#"">"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    // 親チケットID
                                    *buf += r#"<div class="row pb-1">"#;
                                    {
                                        *buf += r#"<label class="col-lg-6 form-label" for="parentid">親チケットID</label>"#;
                                        *buf += r#"<div class="col-lg-6">"#;
                                        {
                                            *buf += r#"<input class="form-control" id="parentid" name="parentid" type="text" maxlength="20" value=""#;
                                            *buf += &self.input.parentid;
                                            *buf += r#"">"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-lg-7">"#;
                                {
                                    // チケット名
                                    *buf += r#"<div class="row pb-1">"#;
                                    {
                                        *buf += r#"<label class="col-lg-4 col-form-label" for="ticketname">チケット名&nbsp;&nbsp;"#;
                                        {
                                            *buf += r#"<small>(部分一致)</small>"#;
                                        }
                                        *buf += r#"</label>"#;
                                        *buf += r#"<div class="col-lg-8">"#;
                                        {
                                            *buf += r#"<input class="form-control" id="ticketname" name="ticketname" type="text" maxlength="100" value=""#;
                                            *buf += &self.input.ticketname;
                                            *buf += r#"">"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    // 担当者
                                    *buf += r#"<div class="row pb-1">"#;
                                    {
                                        *buf += r#"<label class="col-lg-4 col-form-label" for="parentid">担当者</label>"#;

                                        *buf += r#"<div id="charge1" class="col-lg-8"#;
                                        if !self.input.chargemail.is_empty() {
                                            *buf += r#" d-none"#;
                                        }
                                        *buf += r#"">"#;
                                        {
                                            *buf += r#"<a href="javascript:clickSelectCharge()" title="担当者を選択">"#;
                                            {
                                                *buf += r#"<img class="icon2" src="/static/ionicons/person-circle-outline.svg">"#;
                                            }
                                            *buf += r#"</a>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div id="charge2" class="col-lg-8"#;
                                        if self.input.chargemail.is_empty() {
                                            *buf += r#" d-none"#;
                                        }
                                        *buf += r#"">"#;
                                        {
                                            *buf += r#"<span id="charge-sel">"#;
                                            {
                                                *buf += r#"<small>["#;
                                                *buf += &self.input.chargemail;
                                                *buf += r#"]</small> "#;
                                                *buf += &self.input.chargename;
                                            }
                                            *buf += r#"</span>"#;
                                            *buf += r#" "#;
                                            *buf += r#"<a href="javascript:clickRemoveCharge()">"#;
                                            {
                                                *buf += r#"<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                                            }
                                            *buf += r#"</a>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-lg-2 pb-1">"#;
                                {
                                    // 完了済を表示
                                    *buf += r#"<div class="row">"#;
                                    {
                                        *buf += r#"<div class="col">"#;
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
                                    }
                                    *buf += r#"</div>"#;

                                    // 優先度順に表示
                                    *buf += r#"<div class="row pb-2">"#;
                                    {
                                        *buf += r#"<div class="col">"#;
                                        {
                                            *buf += r#"<input class="form-check-input" id="priorityorder" name="priorityorder" type="checkbox""#;
                                            if let Some(f) = &self.input.priorityorder {
                                                if f == "on" || f == "true" {
                                                    *buf += r#" checked="checked""#;
                                                }
                                            }
                                            *buf += r#">"#;
                                            *buf += r#"<label class="form-check-label" for="priorityorder">優先度順に表示</label>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    // フィルターボタン
                                    *buf += r#"<div class="row">"#;
                                    {
                                        *buf += r#"<div class="col d-flex align-items-end justify-content-end">"#;
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
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<input type="hidden" id="chargeuid" name="chargeuid" value=""#;
                                *buf += &self.input.chargeuid;
                                *buf += r#"">"#;
                                *buf += r#"<input type="hidden" id="chargemail" name="chargemail" value=""#;
                                *buf += &self.input.chargemail;
                                *buf += r#"">"#;
                                *buf += r#"<input type="hidden" id="chargename" name="chargename" value=""#;
                                *buf += &self.input.chargename;
                                *buf += r#"">"#;
                                *buf += r#"<input type="hidden" id="page" name="page" value=""#;
                                *buf += &self.input.page.to_string();
                                *buf += r#"">"#;
                            }
                            *buf += r#"</form>"#;

                            // ページング
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

                            // 担当者選択ダイアログ
                            *buf += r#"<div class="modal fade" id="inChargeModal" tabindex="-1" aria-labelledby="chargeModalLabel" aria-hidden="true">"#;
                            {
                                *buf += r#"<div class="modal-dialog modal-lg">"#;
                                {
                                    *buf += r#"<div class="modal-content">"#;
                                    {
                                        *buf += r#"<div class="modal-header">"#;
                                        {
                                            *buf += r#"<h1 class="modal-title fs-5" id="addMemberModalLabel">担当者を選択</h1>"#;
                                            *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="modal-body">"#;
                                        {
                                            *buf += r#"<div class="row py-3">"#;
                                            {
                                                *buf += r#"<div class="col">"#;
                                                {
                                                    *buf += r#"<div id="searched"></div>"#;
                                                }
                                                *buf += r#"</div>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="modal-footer">"#;
                                        {
                                            *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                                            *buf += r#"<button class="btn btn-primary" id="btnSelectMember" type="button">選択</button>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
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

            *buf += r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
            *buf += r#"<script src="/static/js/ticket_list1012.js"></script>"#;
        }
        *buf += r#"</body>"#;
    }
}
