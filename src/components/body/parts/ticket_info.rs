use crate::{components::Component, Props};

pub struct TicketInfo {}

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
                if let Some(p) = &props.project {
                    if let Some(name) = &p.project_name {
                        *buf += &name;
                    }
                    if let Some(t) = &props.ticket {
                        *buf += r#" / "#;
                        if let Some(id) = &t.id_disp {
                            *buf += &id;
                        }
                    }
                    *buf += r#"<input type="hidden" id="project_id" name="project_id" value=""#;
                    if let Some(id) = &p.id {
                        *buf += &id;
                    }
                    *buf += r#"">"#;
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
                    if let Some(v) = &props.ticket_validation {
                        if v.name.is_some() {
                            *buf += r#" is-invalid"#;
                        }
                    }
                    *buf += r#"" id="name" name="name" type="text" maxlength="40" value=""#;
                    if let Some(t) = &props.ticket {
                        if let Some(n) = &t.name {
                            *buf += n;
                        }
                    }
                    *buf += r#"" required>"#;

                    if let Some(v) = &props.ticket_validation {
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
                    *buf += r#"<textarea class="form-control" id="description" name="description" rows="3">"#;
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
                                                    *buf += r#"<a href="javascript:removeCharge("#;
                                                    *buf += &i.to_string();
                                                    *buf += r#")">"#;
                                                    {
                                                        *buf += r#"<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                                                    }
                                                    *buf += r#"</a>"#;

                                                    if i != 0 {
                                                        *buf += r#"&nbsp;<a href="javascript:chargeSeqUp("#;
                                                        *buf += &i.to_string();
                                                        *buf += r#")">"#;
                                                        {
                                                            *buf += r#"<img class="icon" src="/static/ionicons/arrow-up-outline.svg" title="上に移動">"#;
                                                        }
                                                        *buf += r#"</a>"#;
                                                    }

                                                    if (i + 1) != props.ticket_members.len() {
                                                        *buf += r#"&nbsp;<a href="javascript:chargeSeqDown("#;
                                                        *buf += &i.to_string();
                                                        *buf += r#")">"#;
                                                        {
                                                            *buf += r#"<img class="icon" src="/static/ionicons/arrow-down-outline.svg" title="下に移動">"#;
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
                        *buf += r#"</div>"#;

                        *buf += r#"<a href="javascript:clickAddCharge();">"#;
                        {
                            *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="担当者を追加">"#;
                        }
                        *buf += r#"</a>"#;
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
                            *buf += r#"">"#;
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
                            *buf += r#"">"#;
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
                                        *buf += r#"">"#;
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
                                *buf += r#"<input class="form-check-input" id="finished" type="checkbox" value="">"#;
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
                        *buf += r#"<input class="form-check-input" id="priority1" name="priority" type="radio" value="1""#;
                        if let Some(t) = &props.ticket {
                            if t.priority == 1 {
                                *buf += r#" checked="checked""#;
                            }
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority1">最優先</label>"#;
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
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority2">高</label>"#;
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
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority3">中</label>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="form-check form-check-inline">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="priority4" name="priority" type="radio" value="4""#;
                        if let Some(t) = &props.ticket {
                            if t.priority == 4 {
                                *buf += r#" checked="checked""#;
                            }
                        }
                        *buf += r#">"#;
                        *buf += r#"<label class="form-check-label" for="priority4">低</label>"#;
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
                            *buf += r#"<a href="">"#;
                            *buf += &t.id_disp.clone().unwrap();
                            *buf += r#"</a>&nbsp;:&nbsp;"#;
                            *buf += &t.name.clone().unwrap();
                            *buf += r#"&nbsp;"#;
                            *buf += r#"<a href="javascript:removeParent();">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                            }
                            *buf += r#"</a>"#;

                            *buf += r#"<input type="hidden" id="parent" name="parent" value=""#;
                            if let Some(p) = &t.parent_id {
                                *buf += p;
                            }
                            *buf += r#"">"#;
                        } else {
                            *buf += r#"<p class="my-1">"#;
                            {
                                *buf += r#"<a href="javascript:clickAddParent();">"#;
                                {
                                    *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="親チケットを追加">"#;
                                }
                                *buf += r#"</a>"#;
                            }
                            *buf += r#"</p>"#;

                            *buf += r#"<input type="hidden" id="parent" name="parent" value="">"#;
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
                    *buf += r#"<p class="mb-0">"#;
                    {
                        *buf += r#"<a href="">"#;
                        *buf += r#"BN13"#;
                        *buf += r#"</a>&nbsp;:&nbsp;"#;
                        *buf += r#"備品準備"#;
                    }
                    *buf += r#"</p>"#;
                    *buf += r#"<p class="mb-0">"#;
                    {
                        *buf += r#"<a href="">"#;
                        *buf += r#"BN14"#;
                        *buf += r#"</a>&nbsp;:&nbsp;"#;
                        *buf += r#"食材購入"#;
                    }
                    *buf += r#"</p>"#;
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
                    *buf += r#"<a href="step01.html">"#;
                    {
                        *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="成果物を追加">"#;
                    }
                    *buf += r#"</a>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
            {
                if props.action == crate::Action::Create {
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
                } else {
                    if let Some(t) = &props.ticket {
                        *buf += r#"<div class="col-9">"#;
                        {
                            *buf +=
                                r#"<button id="btnSave" class="btn btn-primary" type="button">"#;
                            {
                                *buf +=
                                    r#"<img class="icon" src="/static/ionicons/save-outline.svg">"#;
                                *buf += r#"&nbsp;更新"#;
                            }
                            *buf += r#"</button>&nbsp;&nbsp;"#;
                            *buf += r#"<button class="btn btn-primary" type="submit">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/refresh-outline.svg">"#;
                                *buf += r#"&nbsp;再読み込み"#;
                            }
                            *buf += r#"</button>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="col-3 text-end">"#;
                        {
                            *buf += r#"<button class="btn btn-secondary" type="submit">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/trash-outline2.svg">"#;
                                *buf += r#"&nbsp;削除"#;
                            }
                            *buf += r#"</button>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<input type="hidden" id="ticket_id" name="ticket_id" value=""#;
                        if let Some(id) = &t.id {
                            *buf += id;
                        }
                        *buf += r#"">"#;
                        *buf += r#"<input type="hidden" name="timestamp" value=""#;
                        if let Some(up) = &t.updated_at {
                            *buf += &up.timestamp_micros().to_string();
                        }
                        *buf += r#"">"#;
                    }
                }
            }
            *buf += r#"</div>"#;

            *buf += r#"<input type="hidden" name="action" id="action" value=""#;
            *buf += &props.action.to_string();
            *buf += r#"">"#;
        }
        *buf += r#"</form>"#;

        // 担当者追加ダイアログ
        *buf += r#"<div class="modal fade" id="inChargeModal" tabindex="-1" aria-labelledby="chargeModalLabel" aria-hidden="true">"#;
        {
            *buf += r#"<div class="modal-dialog modal-lg">"#;
            {
                *buf += r#"<div class="modal-content">"#;
                {
                    *buf += r#"<div class="modal-header">"#;
                    {
                        *buf += r#"<h1 class="modal-title fs-5" id="addMemberModalLabel">担当者を追加</h1>"#;
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
                                //*buf +=
                                //    r#"<p class="text-danger">該当のチケットは存在しません。</p>"#;
                                //*buf += r#"<p>BN5 文化祭出し物</p>"#;
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
    }
}
