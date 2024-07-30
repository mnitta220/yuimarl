use crate::{components::Component, handlers::validation, model, Props};

pub struct TicketInfo {
    pub can_update: bool,
    pub can_delete: bool,
    pub validation: Option<validation::ticket::TicketValidation>,
}

impl Component for TicketInfo {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<form name="post_ticket" id="post_ticket" action="/ticket" method="POST">"#;
        {
            // プロジェクト / チケットID
            *buf += r#"<div class="row py-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="ticket-id">プロジェクト"#;
                if props.action != crate::Action::Create {
                    *buf += r#" / チケットID"#;
                }
                *buf += r#"</label>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    if let Some(p) = &props.project {
                        if let Some(name) = &p.project_name {
                            super::super::super::escape_html(&name, buf);
                        }
                        if let Some(t) = &props.ticket {
                            if let Some(id) = &t.id_disp {
                                *buf += r#" / "#;
                                *buf += &id;
                            }
                        }
                        *buf += r#"<input type="hidden" id="project_id" name="project_id" value=""#;
                        *buf += &p.id;
                        *buf += r#"">"#;
                    }
                    *buf += r#"<input class="invisible" type="button" id="btnAddParentInvisible" value="">"#;
                    *buf +=
                        r#"<input class="invisible" type="button" id="btnRemoveParent" value="">"#;
                    *buf +=
                        r#"<input class="invisible" type="button" id="btnRemoveCharge" value="">"#;
                    *buf += r#"<input class="invisible" type="button" id="btnUpCharge" value="">"#;
                    *buf +=
                        r#"<input class="invisible" type="button" id="btnDownCharge" value="">"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // チケット名
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="name">チケット名</label>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<input class="form-control"#;
                    if let Some(v) = &self.validation {
                        if v.name.is_some() {
                            *buf += r#" is-invalid"#;
                        }
                    }
                    *buf += r#"" id="name" name="name" type="text" maxlength="100" value=""#;
                    if let Some(t) = &props.ticket {
                        if let Some(n) = &t.name {
                            *buf += n;
                        }
                    }
                    *buf += r#"" required"#;
                    if !self.can_update {
                        *buf += r#" disabled"#;
                    }
                    *buf += r#">"#;

                    if let Some(v) = &self.validation {
                        if let Some(e) = &v.name {
                            *buf += r#"<div class="invalid-feedback">"#;
                            *buf += e;
                            *buf += r#"</div>"#;
                        }
                    }
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // 内容
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="description">内容</label>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<textarea class="form-control" id="description" name="description" rows="3""#;
                    if !self.can_update {
                        *buf += r#" disabled"#;
                    }
                    *buf += r#">"#;
                    if let Some(t) = &props.ticket {
                        if let Some(d) = &t.description {
                            *buf += d;
                        }
                    }
                    *buf += r#"</textarea>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // 担当者
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="member">担当者</label>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<div class="form-floating">"#;
                    {
                        *buf += r#"<div id="chargeTbl">"#;
                        {
                            if props.ticket_members.len() > 0 {
                                *buf += r#"<table class="table table-hover">"#;
                                {
                                    *buf += r#"<thead>"#;
                                    {
                                        *buf += r#"<tr>"#;
                                        {
                                            *buf += r#"<th scope="col">メールアドレス</th>"#;
                                            *buf += r#"<th scope="col">名前</th>"#;
                                            *buf += r#"<th scope="col"></th>"#;
                                        }
                                        *buf += r#"</tr>"#;
                                    }
                                    *buf += r#"</thead>"#;

                                    *buf += r#"<tbody>"#;
                                    {
                                        let mut i = 0;
                                        for m in &props.ticket_members {
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<td>"#;
                                                if let Some(e) = &m.email {
                                                    *buf += e;
                                                }
                                                *buf += r#"</td>"#;

                                                *buf += r#"<td>"#;
                                                if let Some(n) = &m.name {
                                                    *buf += n;
                                                }
                                                *buf += r#"</td>"#;

                                                *buf += r#"<td>"#;
                                                {
                                                    if self.can_update {
                                                        //*buf += r#"<img class="icon" style="cursor:pointer" id="icnRemoveMem"#;
                                                        //*buf += &i.to_string();
                                                        //*buf += r#"" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                                                        *buf += r#"<a href="javascript:clickRemoveCharge(">"#;
                                                        *buf += &i.to_string();
                                                        *buf += r#");">"#;
                                                        *buf += r#"<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                                                        *buf += r#"</a>"#;

                                                        if i != 0 {
                                                            //*buf += r#"&nbsp;<img class="icon" style="cursor:pointer" id="icnUpMem"#;
                                                            //*buf += &i.to_string();
                                                            //*buf += r#"" src="/static/ionicons/arrow-up-outline.svg" title="上に移動">"#;
                                                            *buf += r#"&nbsp;<a href="javascript:clickUpCharge(">"#;
                                                            *buf += &i.to_string();
                                                            *buf += r#");">"#;
                                                            *buf += r#"<img class="icon" src="/static/ionicons/arrow-up-outline.svg" title="上に移動">"#;
                                                            *buf += r#"</a>"#;
                                                        }

                                                        if (i + 1) != props.ticket_members.len() {
                                                            //*buf += r#"&nbsp;<img class="icon" style="cursor:pointer" id="icnDownMem"#;
                                                            //*buf += &i.to_string();
                                                            //*buf += r#"" src="/static/ionicons/arrow-down-outline.svg" title="下に移動">"#;
                                                            *buf += r#"&nbsp;<a href="javascript:clickDownCharge(">"#;
                                                            *buf += &i.to_string();
                                                            *buf += r#");">"#;
                                                            *buf += r#"<img class="icon" src="/static/ionicons/arrow-down-outline.svg" title="下に移動">"#;
                                                            *buf += r#"</a>"#;
                                                        }
                                                    }
                                                }
                                                *buf += r#"</td>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                            i += 1;
                                        }
                                    }
                                    *buf += r#"</tbody>"#;
                                }
                                *buf += r#"</table>"#;
                            }
                        }
                        *buf += r#"</div>"#;

                        if self.can_update {
                            *buf += r#"<img class="icon3" style="cursor:pointer" id="icnAddCharge" src="/static/ionicons/add-circle-outline.svg" title="担当者を追加">"#;
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<input type="hidden" id="members" name="members" value=""#;
                if let Ok(r) = serde_json::to_string(&props.ticket_members) {
                    super::super::super::escape_html(&r, buf);
                }
                *buf += r#"">"#;
            }
            *buf += r#"</div>"#;

            // 開始日 / 終了日
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<legend class="col-form-label col-md-3 bg-light mb-1">開始日 / 終了日</legend>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col-sm-6 mb-2">"#;
                        {
                            *buf += r#"<label class="form-label" for="startdate">開始日</label>"#;
                            *buf += r#"<input class="form-control" id="start_date" name="start_date" type="date" value=""#;
                            if let Some(t) = &props.ticket {
                                if let Some(s) = &t.start_date {
                                    *buf += s;
                                }
                            }
                            *buf += r#"""#;
                            if !self.can_update {
                                *buf += r#" disabled"#;
                            }
                            *buf += r#">"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="col-sm-6 mb-1">"#;
                        {
                            *buf += r#"<label class="form-label" for="enddate">終了日</label>"#;
                            *buf += r#"<input class="form-control" id="end_date" name="end_date" type="date" value=""#;
                            if let Some(t) = &props.ticket {
                                if let Some(e) = &t.end_date {
                                    *buf += e;
                                }
                            }
                            *buf += r#"""#;
                            if !self.can_update {
                                *buf += r#" disabled"#;
                            }
                            *buf += r#">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // 進捗率
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<legend class="col-form-label col-md-3 bg-light mb-1">進捗率</legend>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col-sm-6 mb-2">"#;
                        {
                            *buf += r#"<table>"#;
                            {
                                *buf += r#"<tr>"#;
                                {
                                    *buf += r#"<td>"#;
                                    {
                                        *buf += r#"<input class="form-control" id="progress" name="progress" type="number" min="0" max="100" value=""#;
                                        if let Some(t) = &props.ticket {
                                            *buf += t.progress.to_string().as_ref();
                                        } else {
                                            *buf += "0";
                                        }
                                        *buf += r#"""#;
                                        if !self.can_update {
                                            *buf += r#" disabled"#;
                                        }
                                        *buf += r#">"#;
                                    }
                                    *buf += r#"</td>"#;
                                    *buf += r#"<td>&nbsp;%</td>"#;
                                }
                                *buf += r#"</tr>"#;
                            }
                            *buf += r#"</table>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="col-sm-6 mb-1">"#;
                        {
                            *buf += r#"<div class="form-check">"#;
                            {
                                *buf += r#"<input class="form-check-input" id="finished" type="checkbox""#;
                                if let Some(t) = &props.ticket {
                                    if t.progress == 100 {
                                        *buf += r#" checked="checked""#;
                                    }
                                }
                                if !self.can_update {
                                    *buf += r#" disabled"#;
                                }
                                *buf += r#">"#;
                                *buf += r#"<label class="form-check-label" for="finished">完了済</label>"#;
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

            // 優先度
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<legend class="col-form-label col-md-3 bg-light mb-1">優先度</legend>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<div class="form-check form-check-inline">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="priority4" name="priority" type="radio" value="4""#;
                        if let Some(t) = &props.ticket {
                            if t.priority == 4 {
                                *buf += r#" checked="checked""#;
                            }
                        }
                        if !self.can_update {
                            *buf += r#" disabled"#;
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority4">最優先</label>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="form-check form-check-inline">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="priority3" name="priority" type="radio" value="3""#;
                        if let Some(t) = &props.ticket {
                            if t.priority == 3 {
                                *buf += r#" checked="checked""#;
                            }
                        }
                        if !self.can_update {
                            *buf += r#" disabled"#;
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority3">高</label>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="form-check form-check-inline">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="priority2" name="priority" type="radio" value="2""#;
                        if let Some(t) = &props.ticket {
                            if t.priority == 2 {
                                *buf += r#" checked="checked""#;
                            }
                        }
                        if !self.can_update {
                            *buf += r#" disabled"#;
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority2">中</label>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="form-check form-check-inline">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="priority1" name="priority" type="radio" value="1""#;
                        if let Some(t) = &props.ticket {
                            if t.priority == 1 {
                                *buf += r#" checked="checked""#;
                            }
                        }
                        if !self.can_update {
                            *buf += r#" disabled"#;
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority1">低</label>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="form-check form-check-inline">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="priority0" name="priority" type="radio" value="0""#;
                        if let Some(t) = &props.ticket {
                            if t.priority == 0 {
                                *buf += r#" checked="checked""#;
                            }
                        } else {
                            *buf += r#" checked="checked""#;
                        }
                        if !self.can_update {
                            *buf += r#" disabled"#;
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority0">未設定</label>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // 親チケット
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">親チケット</label>"#;
                *buf += r#"<div class="col-md-9">"#;
                {
                    *buf += r#"<div id="parentTicket">"#;
                    {
                        if let Some(t) = &props.ticket_parent {
                            *buf += r#"<a href="/ticket?id="#;
                            *buf += &t.id;
                            *buf += r#"">"#;
                            *buf += &t.id_disp.clone().unwrap();
                            *buf += r#"</a>&nbsp;:&nbsp;"#;

                            if let Some(ref name) = t.name {
                                super::super::super::escape_html(&name, buf);
                            }

                            if self.can_update {
                                *buf += r#"&nbsp;"#;
                                *buf += r#"<a href="javascript:clickRemoveParent();">"#;
                                {
                                    *buf += r#"<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                                }
                                *buf += r#"</a>"#;
                                //*buf += r#"<img class="icon" style="cursor:pointer" id="icnRemoveParent" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;

                                //*buf += r#"<input class="invisible" type="button" id="btnAddParentInvisible" value="">"#;
                                //*buf += r#"<input class="invisible" type="button" id="btnRemoveParent" value="">"#;
                                *buf += r#"<input type="hidden" id="parent" name="parent" value=""#;
                                *buf += &t.id;
                                *buf += r#"">"#;
                            }
                        } else {
                            *buf += r#"<p class="my-1">"#;
                            {
                                if self.can_update {
                                    *buf += r#"<a href="javascript:clickAddParentInvisible();">"#;
                                    {
                                        *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="親チケットを追加">"#;
                                    }
                                    *buf += r#"</a>"#;
                                    //*buf += r#"<img class="icon3" style="cursor:pointer" id="icnAddParent" src="/static/ionicons/add-circle-outline.svg" title="親チケットを追加">"#;

                                    //*buf += r#"<input class="invisible" type="button" id="btnAddParentInvisible" value="">"#;
                                    //*buf += r#"<input class="invisible" type="button" id="btnRemoveParent" value="">"#;
                                    *buf += r#"<input type="hidden" id="parent" name="parent" value="">"#;
                                }
                            }
                            *buf += r#"</p>"#;
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // 子チケット
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">子チケット</label>"#;
                *buf += r#"<div class="col-md-9 pb-md-2">"#;
                {
                    for child in &props.ticket_children {
                        *buf += r#"<p class="mb-0">"#;
                        {
                            if let Some(id_disp) = &child.id_disp {
                                *buf += r#"<a href="/ticket?id="#;
                                *buf += &child.id;
                                *buf += r#"">"#;
                                *buf += &id_disp;
                                *buf += r#"</a>&nbsp;:&nbsp;"#;
                            }

                            if let Some(name) = &child.name {
                                super::super::super::escape_html(&name, buf);
                            }
                        }
                        *buf += r#"</p>"#;
                    }
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // 成果物
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">成果物</label>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<div class="form-floating">"#;
                    {
                        *buf += r#"<div id="deliverablesTbl">"#;
                        {
                            if let Some(t) = &props.ticket {
                                if let Some(d) = &t.deliverables {
                                    let deliverables: Vec<model::deliverable::Deliverable> =
                                        match serde_json::from_str(&d) {
                                            Ok(h) => h,
                                            Err(_) => Vec::new(),
                                        };
                                    if deliverables.len() > 0 {
                                        *buf += r#"<table class="table table-hover">"#;
                                        {
                                            *buf += r#"<thead>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<th scope="col">成果物名</th>"#;
                                                    *buf += r#"<th scope="col">ファイルパス / URL</th>"#;
                                                    *buf += r#"<th scope="col"></th>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</thead>"#;

                                            *buf += r#"<tbody>"#;
                                            {
                                                let mut i = 0;
                                                for deliverable in deliverables {
                                                    *buf += r#"<tr>"#;
                                                    {
                                                        *buf += r#"<td>"#;
                                                        super::super::super::escape_html(
                                                            &deliverable.name,
                                                            buf,
                                                        );
                                                        *buf += r#"</td>"#;

                                                        *buf += r#"<td>"#;
                                                        if deliverable.path.len() > 10
                                                            && (&deliverable.path[0..7]
                                                                == "http://"
                                                                || &deliverable.path[0..8]
                                                                    == "https://")
                                                        {
                                                            *buf += r#"<a href=""#;
                                                            *buf += &deliverable.path;
                                                            *buf += r#"" target="_blank">"#;
                                                            *buf += &deliverable.path;
                                                            *buf += r#"</a>"#;
                                                        } else {
                                                            super::super::super::escape_html(
                                                                &deliverable.path,
                                                                buf,
                                                            );
                                                        }
                                                        *buf += r#"</td>"#;

                                                        *buf += r#"<td>"#;
                                                        {
                                                            if self.can_update {
                                                                *buf += r#"<a href="javascript:removeDeliverable("#;
                                                                *buf += &i.to_string();
                                                                *buf += r#")">"#;
                                                                {
                                                                    *buf += r#"<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                                                                }
                                                                *buf += r#"</a>"#;
                                                            }
                                                        }
                                                        *buf += r#"</td>"#;
                                                    }
                                                    *buf += r#"</tr>"#;
                                                    i += 1;
                                                }
                                            }
                                            *buf += r#"</tbody>"#;
                                        }
                                        *buf += r#"</table>"#;
                                    }
                                }
                            }
                        }
                        *buf += r#"</div>"#;

                        if self.can_update {
                            *buf += r#"<a href="javascript:clickDeliverables();">"#;
                            {
                                *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="成果物を追加">"#;
                            }
                            *buf += r#"</a>"#;
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<input type="hidden" id="deliverables" name="deliverables" value=""#;
                if let Some(t) = &props.ticket {
                    if let Some(d) = &t.deliverables {
                        super::super::super::escape_html(&d, buf);
                    } else {
                        *buf += r#"[]"#;
                    }
                } else {
                    *buf += r#"[]"#;
                }
                *buf += r#"">"#;
            }
            *buf += r#"</div>"#;

            // ガントチャート
            *buf += r#"<div class="row pt-1 pb-2">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">ガントチャート</label>"#;
                *buf += r#"<div class="col-md-9 mb-1">"#;
                {
                    *buf += r#"<div class="form-check">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="ganttchart" name="ganttchart" type="checkbox" value="on""#;
                        if let Some(t) = &props.ticket {
                            if let Some(g) = &t.ganttchart {
                                if *g {
                                    *buf += r#" checked="checked""#;
                                }
                            }
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="ganttchart">ガントチャートに表示する</label>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            if props.action == crate::Action::Create {
                *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                {
                    *buf += r#"<div class="col">"#;
                    {
                        *buf += r#"<button id="btnSave" class="btn btn-primary" type="button">"#;
                        {
                            *buf += r#"<img class="icon" src="/static/ionicons/create-outline.svg">&nbsp;作成"#;
                        }
                        *buf += r#"</button>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<input type="hidden" name="ticket_id" value="">"#;
                    *buf += r#"<input type="hidden" name="timestamp" value="">"#;
                }
                *buf += r#"</div>"#;
            } else {
                if self.can_update {
                    *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                    {
                        if let Some(t) = &props.ticket {
                            *buf += r#"<div class="col-9">"#;
                            {
                                *buf += r#"<button id="btnSave" class="btn btn-primary" type="button">"#;
                                {
                                    *buf += r#"<img class="icon" src="/static/ionicons/save-outline.svg">"#;
                                    *buf += r#"&nbsp;更新"#;
                                }
                                *buf += r#"</button>&nbsp;&nbsp;"#;

                                *buf += r#"<a class="btn btn-primary" href="/ticket?id="#;
                                *buf += &t.id;
                                *buf += r#"" role="button">"#;
                                {
                                    *buf += r#"<img class="icon" src="/static/ionicons/refresh-outline.svg">&nbsp;再読み込み"#;
                                }
                                *buf += r#"</a>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-3 text-end">"#;
                            {
                                if self.can_delete {
                                    *buf += r##"<button class="btn btn-secondary" type="button" data-bs-toggle="modal" data-bs-target="#ticketDelModal">"##;
                                    {
                                        *buf += r#"<img class="icon" src="/static/ionicons/trash-outline2.svg">&nbsp;削除"#;
                                    }
                                    *buf += r#"</button>"#;
                                }
                            }
                            *buf += r#"</div>"#;

                            *buf +=
                                r#"<input type="hidden" id="ticket_id" name="ticket_id" value=""#;
                            *buf += &t.id;
                            *buf += r#"">"#;
                            *buf += r#"<input type="hidden" name="timestamp" value=""#;
                            if let Some(up) = &t.updated_at {
                                *buf += &up.timestamp_micros().to_string();
                            }
                            *buf += r#"">"#;
                        }
                    }
                    *buf += r#"</div>"#;
                }
            }

            // 背景色
            let mut color: Option<String> = None;
            let mut exist = false;
            let mut session_uid = "".to_string();
            if let Some(s) = &props.session {
                session_uid = s.uid.clone();
                for member in &props.ticket_members {
                    if member.uid == s.uid {
                        color = member.color.clone();
                        exist = true;
                        break;
                    }
                }
            }
            let color = match color {
                Some(c) => c,
                None => String::from("light"),
            };

            *buf += r#"<div id="back_color" class="row pt-3"#;
            if !exist {
                *buf += r#" d-none"#;
            }
            *buf += r#"">"#;
            {
                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">背景色</label>"#;
                *buf += r#"<div class="col-md-9 mb-1 flexbox">"#;
                {
                    *buf += r#"<a href="javascript:clickColor('info');">"#;
                    {
                        *buf += r#"<div class="box-info color-box">"#;
                        {
                            *buf += r#"<img id="img-info" class="icon"#;
                            if color != "info" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<a href="javascript:clickColor('primary');">"#;
                    {
                        *buf += r#"<div class="box-primary color-box">"#;
                        {
                            *buf += r#"<img id="img-primary" class="icon"#;
                            if color != "primary" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<a href="javascript:clickColor('warning');">"#;
                    {
                        *buf += r#"<div class="box-warning color-box">"#;
                        {
                            *buf += r#"<img id="img-warning" class="icon"#;
                            if color != "warning" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<a href="javascript:clickColor('success');">"#;
                    {
                        *buf += r#"<div class="box-success color-box">"#;
                        {
                            *buf += r#"<img id="img-success" class="icon"#;
                            if color != "success" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<a href="javascript:clickColor('danger');">"#;
                    {
                        *buf += r#"<div class="box-danger color-box">"#;
                        {
                            *buf += r#"<img id="img-danger" class="icon"#;
                            if color != "danger" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<a href="javascript:clickColor('secondary');">"#;
                    {
                        *buf += r#"<div class="box-secondary color-box">"#;
                        {
                            *buf += r#"<img id="img-secondary" class="icon"#;
                            if color != "secondary" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<a href="javascript:clickColor('dark');">"#;
                    {
                        *buf += r#"<div class="box-dark color-box">"#;
                        {
                            *buf += r#"<img id="img-dark" class="icon"#;
                            if color != "dark" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<a href="javascript:clickColor('light');">"#;
                    {
                        *buf +=
                            r#"<div class="box-light color-box" style="border: solid 1px #aaa;">"#;
                        {
                            *buf += r#"<img id="img-light" class="icon"#;
                            if color != "light" {
                                *buf += r#" d-none"#;
                            }
                            *buf += r#"" src="/static/ionicons/checkmark-outline.svg">"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</a>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<input type="hidden" name="color" id="color" value=""#;
            *buf += &color;
            *buf += r#"">"#;

            *buf += r#"<input type="hidden" name="session_uid" id="session_uid" value=""#;
            *buf += &session_uid;
            *buf += r#"">"#;

            *buf += r#"<input type="hidden" name="action" id="action" value=""#;
            *buf += &props.action.to_string();
            *buf += r#"">"#;

            *buf += r#"<input type="hidden" name="selectedIndex" id="selectedIndex" value="">"#;
        }
        *buf += r#"</form>"#;

        // 担当者追加ダイアログ
        *buf += r#"<div class="modal fade" id="addChargeModal" tabindex="-1" aria-labelledby="chargeModalLabel" aria-hidden="true">"#;
        {
            *buf += r#"<div class="modal-dialog modal-lg">"#;
            {
                *buf += r#"<div class="modal-content">"#;
                {
                    *buf += r#"<div class="modal-header">"#;
                    {
                        *buf += r#"<h1 class="modal-title fs-5" id="addChargeModalLabel">担当者を追加</h1>"#;
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
                        *buf += r#"<button class="btn btn-primary" id="btnMemberAdd" type="button">担当者に追加</button>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</div>"#;

        // 親チケット追加ダイアログ
        *buf += r#"<div class="modal fade" id="addParentModal" tabindex="-1" aria-labelledby="addParentModalLabel" aria-hidden="true">"#;
        {
            *buf += r#"<div class="modal-dialog modal-md">"#;
            {
                *buf += r#"<div class="modal-content">"#;
                {
                    *buf += r#"<div class="modal-header">"#;
                    {
                        *buf += r#"<h1 class="modal-title fs-5" id="addMemberModalLabel">親チケットを追加</h1>"#;
                        *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="modal-body">"#;
                    {
                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<label class="col-md-3 col-form-label mb-1" for="search_id">チケットID</label>"#;
                            *buf += r#"<div class="col-md-6 mb-1">"#;
                            {
                                *buf += r#"<input class="form-control" id="search_id" name="search_id" type="text" maxlength="20" value="">"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-md-3 mb-1">"#;
                            {
                                *buf += r#"<button class="btn btn-info" id="search-parent" type="button">検索</button>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row py-3">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<div id="searchedParent"></div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="modal-footer">"#;
                    {
                        *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                        *buf += r#"<button class="btn btn-primary" id="btnAddParent" type="button" disabled>親チケットを追加</button>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</div>"#;

        // 成果物追加ダイアログ
        *buf += r#"<div class="modal fade" id="deliverablesModal" tabindex="-1" aria-labelledby="deliverablesModalLabel" aria-hidden="true">"#;
        {
            *buf += r#"<div class="modal-dialog modal-lg">"#;
            {
                *buf += r#"<div class="modal-content">"#;
                {
                    *buf += r#"<div class="modal-header">"#;
                    {
                        *buf += r#"<h1 class="modal-title fs-5" id="deliverablesModalLabel">成果物を追加</h1>"#;
                        *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="modal-body">"#;
                    {
                        *buf += r#"<div class="row py-1">"#;
                        {
                            *buf += r#"<label class="col-md-3 col-form-label mb-1" for="deliverable-name">成果物名</label>"#;
                            *buf += r#"<div class="col-md-9 mb-1">"#;
                            {
                                *buf += r#"<input class="form-control" id="deliverable-name" type="text" maxlength="100" value="" required>"#;
                                *buf += r#"<div class="invalid-feedback d-none" id="deliverable-feedback">入力してください。</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                        *buf += r#"<div class="row py-1">"#;
                        {
                            *buf += r#"<label class="col-md-3 col-form-label mb-1" for="deliverable-path">ファイルパス / URL</label>"#;
                            *buf += r#"<div class="col-md-9 mb-1">"#;
                            {
                                *buf += r#"<input class="form-control" id="deliverable-path" type="text" maxlength="300" value="">"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="modal-footer">"#;
                    {
                        *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                        *buf += r#"<button class="btn btn-primary" id="btnAddDeliverable" type="button">"#;
                        {
                            *buf += r#"<img class="icon" src="/static/ionicons/add-circle-outline2.svg">&nbsp;追加"#;
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

        // チケット削除ダイアログ
        *buf += r#"<div class="modal fade" id="ticketDelModal" tabindex="-1" aria-labelledby="ticketDelModalLabel" aria-hidden="true">"#;
        {
            *buf += r#"<div class="modal-dialog">"#;
            {
                *buf += r#"<div class="modal-content">"#;
                {
                    *buf += r#"<div class="modal-header">"#;
                    {
                        *buf += r#"<h1 class="modal-title fs-5" id="ticketDelModalLabel">チケット削除</h1>"#;
                        *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="Close"></button>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="modal-body">"#;
                    {
                        *buf += r#"<p>チケットを削除してもよろしいですか？</p>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="modal-footer">"#;
                    {
                        *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                        *buf +=
                            r#"<button id="btnTicketDel" class="btn btn-danger" type="button">"#;
                        {
                            *buf += r#"<img class="icon" src="/static/ionicons/trash-outline2.svg">&nbsp;削除"#;
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

        *buf += r#"<script>
        function clickRemoveCharge(id) {
          const btnRemoveCharge = document.querySelector(`#btnRemoveCharge`);
          if (btnRemoveCharge) {
            const selectedIndex = document.querySelector(`#selectedIndex`);
            if (selectedIndex) {
              selectedIndex.value = `${id}`;
              btnRemoveCharge.click();
            }
          }
        }

        function clickUpCharge(id) {
          const btnUpCharge = document.querySelector(`#btnUpCharge`);
          if (btnUpCharge) {
            const selectedIndex = document.querySelector(`#selectedIndex`);
            if (selectedIndex) {
              selectedIndex.value = `${id}`;
              btnUpCharge.click();
            }
          }
        }

        function clickDownCharge(id) {
          const btnDownCharge = document.querySelector(`#btnDownCharge`);
          if (btnDownCharge) {
            const selectedIndex = document.querySelector(`#selectedIndex`);
            if (selectedIndex) {
              selectedIndex.value = `${id}`;
              btnDownCharge.click();
            }
          }
        }

        function clickAddParentInvisible() {
            const btnAddParentInvisible = document.querySelector(`#btnAddParentInvisible`);
            if (btnAddParentInvisible) {
              btnAddParentInvisible.click();
            }
        }

        function clickRemoveParent() {
            const btnRemoveParent = document.querySelector(`#btnRemoveParent`);
            if (btnRemoveParent) {
              btnRemoveParent.click();
            }
        }
        </script>"#;
    }
}
