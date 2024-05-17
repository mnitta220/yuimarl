use crate::{components::Component, Props};

pub struct ProjectNote {}

impl Component for ProjectNote {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<form action="/project/upd_note" method="POST">"#;
        {
            // 編集スイッチ
            *buf += r#"<div class="row py-2">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<div class="form-check form-switch">"#;
                    {
                        *buf += r#"<input class="form-check-input" id="edit" type="checkbox" role="switch">"#;
                        *buf += r#"<label class="form-check-label" for="edit">編集</label>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            // ノート
            *buf += r#"<div class="row py-2 d-none" id="note1">"#;
            {
                *buf += r#"<div class="col-lg-6">"#;
                {
                    *buf += r#"<small>［マークダウン］</small>"#;
                    *buf += r#"<textarea class="form-control" id="markdown" name="markdown" rows="10">"#;
                    if let Some(p) = &props.project {
                        if let Some(note) = &p.note {
                            *buf += note;
                        }
                    }
                    *buf += r#"</textarea>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="col-lg-6">"#;
                {
                    *buf += r#"<small>［プレビュー］</small>"#;
                    *buf += r#"<div class="px-2 bg-light preview" id="preview1"></div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="row py-2" id="note2">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<div class="px-2 bg-light preview" id="preview2"></div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<button class="btn btn-primary" type="submit">"#;
                    {
                        *buf += r#"<img class="icon" src="/static/ionicons/save-outline.svg">&nbsp;更新"#;
                    }
                    *buf += r#"</button>&nbsp;&nbsp;"#;
                    *buf += r#"<button class="btn btn-primary" type="submit">"#;
                    {
                        *buf += r#"<img class="icon" src="/static/ionicons/refresh-outline.svg">&nbsp;再読み込み"#;
                    }
                    *buf += r#"</button>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<input type="hidden" name="project_id" value=""#;
            if let Some(p) = &props.project {
                if let Some(id) = &p.id {
                    *buf += id;
                }
            }
            *buf += r#"">"#;
        }
        *buf += r#"</form>"#;
    }
}
