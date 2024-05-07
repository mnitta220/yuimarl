use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct TicketBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl TicketBody {
    pub fn new() -> Self {
        TicketBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for TicketBody {
    fn write(&self, props: &Props, buf: &mut String) {
        let (progress, priority) = match &props.ticket {
            Some(t) => (t.progress, t.priority.clone()),
            None => (0, "0".to_string()),
        };

        *buf += r#"<body>"#;
        {
            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="py-3">"#;
                {
                    *buf += r#"<div class="container">"#;
                    {
                        *buf += r#"<h3 class="mb-3">チケット</h3>"#;

                        *buf += r#"<form action="/ticket/create" method="POST">"#;
                        {
                            // プロジェクト / チケットID
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="ticket-id" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"プロジェクト / チケットID"#;
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    if let Some(p) = &props.project {
                                        *buf += &p.project_name;
                                    }
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // チケット名
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="name" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"チケット名"#;
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input type="text" class="form-control"#;
                                    if let Some(v) = &props.ticket_validation {
                                        if let Some(_) = &v.name {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="name" name="name" maxlength="40" value=""#;
                                    if let Some(t) = &props.ticket {
                                        *buf += &t.name;
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
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="note" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"内容"#;
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    //*buf += r#"<div class="form-floating">"#;
                                    *buf += r#"<div>"#;
                                    {
                                        *buf += r#"<textarea class="form-control" id="note" name="note" rows="4">"#;
                                        if let Some(t) = &props.ticket {
                                            *buf += &t.note;
                                        }
                                        *buf += r#"</textarea>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 担当者
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="email" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"担当者"#;
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="table-responsive">"#;
                                    {
                                        *buf += r#"<table class="table table-hover">"#;
                                        {
                                            *buf += r#"<thead>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<th scope="col">名前</th>"#;
                                                    *buf +=
                                                        r#"<th scope="col">メールアドレス</th>"#;
                                                    *buf += r#"<th scope="col"></th>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</thead>"#;
                                            *buf += r#"<tbody>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>新田雅浩新田雅浩新田雅浩新田雅浩</td>"#;
                                                    *buf += r#"<td>masa.nitta@nifty.ne.jp</td>"#;
                                                    *buf += r#"<td class="text-nowrap">"#;
                                                    {
                                                        *buf += r#"<a href="" class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"削除</a>&nbsp;"#;
                                                        *buf += r#"<a href="" class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        {
                                                            *buf += r#"<svg xmlns="http://www.w3.org/2000/svg" width="11" height="11" "#;
                                                            *buf += r#"fill="currentColor" class="bi bi-arrow-up" viewBox="0 0 16 16">"#;
                                                            {
                                                                *buf +=
                                                                    r#"<path fill-rule="evenodd" "#;
                                                                *buf += r#"d="M8 15a.5.5 0 0 0 .5-.5V2.707l3.146 3.147a.5.5 0 0 0 .708-.708l-4-4a.5.5 0 0 0-.708 0l-4 4a.5.5 0 1 0 .708.708L7.5 2.707V14.5a.5.5 0 0 0 .5.5z" />"#;
                                                            }
                                                            *buf += r#"</svg>"#;
                                                        }
                                                        *buf += r#"</a>&nbsp;"#;
                                                        *buf += r#"<a href="" "#;
                                                        *buf += r#"class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        {
                                                            *buf += r#"<svg xmlns="http://www.w3.org/2000/svg" width="11" height="11" "#;
                                                            *buf += r#"fill="currentColor" class="bi bi-arrow-down" "#;
                                                            *buf += r#"viewBox="0 0 16 16">"#;
                                                            {
                                                                *buf +=
                                                                    r#"<path fill-rule="evenodd" "#;
                                                                *buf += r#"d="M8 1a.5.5 0 0 1 .5.5v11.793l3.146-3.147a.5.5 0 0 1 .708.708l-4 4a.5.5 0 0 1-.708 0l-4-4a.5.5 0 0 1 .708-.708L7.5 13.293V1.5A.5.5 0 0 1 8 1z" />"#;
                                                            }
                                                            *buf += r#"</svg>"#;
                                                        }
                                                        *buf += r#"</a>"#;
                                                    }
                                                    *buf += r#"</td>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>新田雅浩</td>"#;
                                                    *buf += r#"<td>masa.nitta@nifty.ne.jp</td>"#;
                                                    *buf += r#"<td class="text-nowrap">"#;
                                                    {
                                                        *buf += r#"<a href="" class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"削除</a>&nbsp;"#;
                                                        *buf += r#"<a href="" class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        {
                                                            *buf += r#"<svg xmlns="http://www.w3.org/2000/svg" width="11" height="11" "#;
                                                            *buf += r#"fill="currentColor" class="bi bi-arrow-up" viewBox="0 0 16 16">"#;
                                                            {
                                                                *buf +=
                                                                    r#"<path fill-rule="evenodd" "#;
                                                                *buf += r#"d="M8 15a.5.5 0 0 0 .5-.5V2.707l3.146 3.147a.5.5 0 0 0 .708-.708l-4-4a.5.5 0 0 0-.708 0l-4 4a.5.5 0 1 0 .708.708L7.5 2.707V14.5a.5.5 0 0 0 .5.5z" />"#;
                                                            }
                                                            *buf += r#"</svg>"#;
                                                        }
                                                        *buf += r#"</a>&nbsp;"#;
                                                        *buf += r#"<a href="" "#;
                                                        *buf += r#"class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        {
                                                            *buf += r#"<svg xmlns="http://www.w3.org/2000/svg" width="11" height="11" "#;
                                                            *buf += r#"fill="currentColor" class="bi bi-arrow-down" "#;
                                                            *buf += r#"viewBox="0 0 16 16">"#;
                                                            {
                                                                *buf +=
                                                                    r#"<path fill-rule="evenodd" "#;
                                                                *buf += r#"d="M8 1a.5.5 0 0 1 .5.5v11.793l3.146-3.147a.5.5 0 0 1 .708.708l-4 4a.5.5 0 0 1-.708 0l-4-4a.5.5 0 0 1 .708-.708L7.5 13.293V1.5A.5.5 0 0 1 8 1z" />"#;
                                                            }
                                                            *buf += r#"</svg>"#;
                                                        }
                                                        *buf += r#"</a>"#;
                                                    }
                                                    *buf += r#"</td>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</tbody>"#;
                                        }
                                        *buf += r#"</table>"#;
                                        *buf += r#"<button type="submit" class="btn btn-primary btn-sm" data-bs-toggle="tooltip" "#;
                                        *buf += r#"data-bs-placement="top" title="担当者を追加する">＋</button>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 開始日 / 終了日
                            *buf += r#"<fieldset class="py-2">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf +=
                                        r#"<legend class="col-form-label col-md-3 bg-light mb-1">"#;
                                    *buf += r#"開始日 / 終了日"#;
                                    *buf += r#"</legend>"#;
                                    *buf += r#"<div class="col-md-9 mb-1">"#;
                                    {
                                        *buf += r#"<dl class="row">"#;
                                        {
                                            *buf += r#"<div class="col-sm-6 mb-2">"#;
                                            {
                                                *buf += r#"<label for="start_date" class="form-label">開始日</label>"#;
                                                *buf += r#"<input type="date" class="form-control" id="start_date" name="start_date" value=""#;
                                                if let Some(t) = &props.ticket {
                                                    *buf += &t.start_date;
                                                }
                                                *buf += r#"">"#;
                                            }
                                            *buf += r#"</div>"#;

                                            *buf += r#"<div class="col-sm-6 mb-1">"#;
                                            {
                                                *buf += r#"<label for="end_date" class="form-label">終了日</label>"#;
                                                *buf += r#"<input type="date" class="form-control" id="end_date" name="end_date" value=""#;
                                                if let Some(t) = &props.ticket {
                                                    *buf += &t.end_date;
                                                }
                                                *buf += r#"">"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</dl>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</fieldset>"#;

                            // 進捗率
                            *buf += r#"<fieldset class="py-2">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf +=
                                        r#"<legend class="col-form-label col-md-3 bg-light mb-1">"#;
                                    *buf += r#"進捗率"#;
                                    *buf += r#"</legend>"#;
                                    *buf += r#"<div class="col-md-9 mb-1">"#;
                                    {
                                        *buf += r#"<dl class="row">"#;
                                        {
                                            *buf += r#"<div class="col-sm-6 mb-2">"#;
                                            {
                                                *buf += r#"<select class="form-select" id="progress" name="progress" value=""#;
                                                *buf += &progress.to_string();
                                                *buf += r#"">"#;
                                                {
                                                    let mut p = 0;
                                                    loop {
                                                        if p > 100 {
                                                            break;
                                                        };
                                                        *buf += r#"<option value=""#;
                                                        *buf += &p.to_string();
                                                        *buf += r#"""#;
                                                        if progress == p {
                                                            *buf += r#" selected"#;
                                                        }
                                                        *buf += r#">"#;
                                                        *buf += &p.to_string();
                                                        *buf += r#"%</option>"#;
                                                        p += 10;
                                                    }
                                                }
                                                *buf += r#"</select>"#;
                                            }
                                            *buf += r#"</div>"#;
                                            *buf += r#"<div class="col-sm-6 mb-1">"#;
                                            {
                                                *buf += r#"<div class="form-check">"#;
                                                {
                                                    *buf += r#"<input class="form-check-input" type="checkbox" id="finished" name="finished""#;
                                                    if progress == 100 {
                                                        *buf += r#" checked"#;
                                                    }
                                                    *buf += r#">"#;
                                                    *buf += r#"<label class="form-check-label" for="finished">"#;
                                                    *buf += r#"完了済"#;
                                                    *buf += r#"</label>"#;
                                                }
                                                *buf += r#"</div>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</dl>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</fieldset>"#;

                            // 優先度
                            *buf += r#"<fieldset class="py-2">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf +=
                                        r#"<legend class="col-form-label col-md-3 bg-light mb-1">"#;
                                    *buf += r#"優先度"#;
                                    *buf += r#"</legend>"#;
                                    *buf += r#"<div class="col-md-9 mb-1">"#;
                                    {
                                        *buf += r#"<div class="form-check form-check-inline">"#;
                                        {
                                            *buf += r#"<input class="form-check-input" type="radio" name="priority" id="priority1" "#;
                                            *buf += r#"value="1""#;
                                            if priority == "1" {
                                                *buf += r#" checked"#;
                                            }
                                            *buf += r#">"#;
                                            *buf += r#"<label class="form-check-label" for="priority1">最優先</label>"#;
                                        }
                                        *buf += r#"</div>"#;
                                        *buf += r#"<div class="form-check form-check-inline">"#;
                                        {
                                            *buf += r#"<input class="form-check-input" type="radio" name="priority" id="priority2" "#;
                                            *buf += r#"value="2""#;
                                            if priority == "2" {
                                                *buf += r#" checked"#;
                                            }
                                            *buf += r#">"#;
                                            *buf += r#"<label class="form-check-label" for="priority2">高</label>"#;
                                        }
                                        *buf += r#"</div>"#;
                                        *buf += r#"<div class="form-check form-check-inline">"#;
                                        {
                                            *buf += r#"<input class="form-check-input" type="radio" name="priority" id="priority3" "#;
                                            *buf += r#"value="3""#;
                                            if priority == "3" {
                                                *buf += r#" checked"#;
                                            }
                                            *buf += r#">"#;
                                            *buf += r#"<label class="form-check-label" for="priority3">中</label>"#;
                                        }
                                        *buf += r#"</div>"#;
                                        *buf += r#"<div class="form-check form-check-inline">"#;
                                        {
                                            *buf += r#"<input class="form-check-input" type="radio" name="priority" id="priority4" "#;
                                            *buf += r#"value="4""#;
                                            if priority == "4" {
                                                *buf += r#" checked"#;
                                            }
                                            *buf += r#">"#;
                                            *buf += r#"<label class="form-check-label" for="priority4">低</label>"#;
                                        }
                                        *buf += r#"</div>"#;
                                        *buf += r#"<div class="form-check form-check-inline">"#;
                                        {
                                            *buf += r#"<input class="form-check-input" type="radio" name="priority" id="priority0" "#;
                                            *buf += r#"value="0""#;
                                            if priority == "0" {
                                                *buf += r#" checked"#;
                                            }
                                            *buf += r#">"#;
                                            *buf += r#"<label class="form-check-label" for="priority0">未設定</label>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</fieldset>"#;

                            // 親チケット
                            *buf += r#"<fieldset class="py-2">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf +=
                                        r#"<legend class="col-form-label col-md-3 bg-light mb-1">"#;
                                    *buf += r#"親チケット"#;
                                    *buf += r#"</legend>"#;
                                    *buf += r#"<div class="col-md-9 mb-1">"#;
                                    {
                                        *buf += r#"<dl class="row">"#;
                                        {
                                            *buf += r#"<dt class="col-md-3"><a href="">YM1000</a></dt>"#;
                                            *buf += r#"<dd class="col-md-9">ドリンクサービスクーポン配布中です</dd>"#;
                                        }
                                        *buf += r#"</dl>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</fieldset>"#;

                            // 子チケット
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="category" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"子チケット"#;
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<dl class="row">"#;
                                    {
                                        *buf +=
                                            r#"<dt class="col-md-3"><a href="">YM1000</a></dt>"#;
                                        *buf += r#"<dd class="col-md-9">ドリンクサービスクーポン配布中です</dd>"#;
                                        *buf +=
                                            r#"<dt class="col-md-3"><a href="">YM1001</a></dt>"#;
                                        *buf += r#"この季節だけの花木を追加しました"#;
                                    }
                                    *buf += r#"</dl>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 作成するボタン
                            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                            {
                                *buf += r#"<div class="col-md-9">"#;
                                {
                                    *buf += r#"<button type="submit" class="btn btn-primary">作成する</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;
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
