use crate::{components::Component, Props};

pub struct TicketNote {
    pub can_update: bool,
}

impl Component for TicketNote {
    fn write(&self, props: &Props, buf: &mut String) {
        // ノートが空の場合は、編集スイッチをONにする
        let mut edit_switch = true;
        if let Some(t) = &props.ticket {
            if let Some(note) = &t.note {
                if !note.is_empty() {
                    edit_switch = false;
                }
            }

            *buf += r#"<form action="/ticket_note" method="POST">"#;
            {
                if self.can_update {
                    // 編集スイッチ
                    *buf += r#"<div class="row py-2">"#;
                    {
                        *buf += r#"<div class="col">"#;
                        {
                            *buf += r#"<div class="form-check form-switch">"#;
                            {
                                *buf += r#"<input class="form-check-input" id="edit" type="checkbox" role="switch""#;
                                if edit_switch {
                                    *buf += r#" checked"#;
                                }
                                *buf += r#">"#;
                                *buf +=
                                    r#"<label class="form-check-label" for="edit">編集</label>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                } else {
                    edit_switch = false;
                }

                // ノート
                *buf += r#"<div class="row py-2"#;
                if !edit_switch {
                    *buf += r#" d-none"#;
                }
                *buf += r#"" id="note1">"#;
                {
                    *buf += r#"<div class="col-lg-6">"#;
                    {
                        *buf += r#"<small>［マークダウン］</small>"#;
                        *buf += r#"<textarea class="form-control" id="markdown" name="markdown" rows="10">"#;
                        if let Some(note) = &t.note {
                            *buf += note;
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

                *buf += r#"<div class="row py-2"#;
                if edit_switch {
                    *buf += r#" d-none"#;
                }
                *buf += r#"" id="note2">"#;
                {
                    *buf += r#"<div class="col">"#;
                    {
                        *buf += r#"<div class="px-2 bg-light preview" id="preview2"></div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;

                if self.can_update {
                    *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                    {
                        *buf += r#"<div class="col">"#;
                        {
                            *buf += r#"<button class="btn btn-primary" type="submit">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/save-outline.svg">&nbsp;更新"#;
                            }
                            *buf += r#"</button>&nbsp;&nbsp;"#;

                            *buf += r#"<a class="btn btn-primary" href="/ticket?id="#;
                            *buf += &t.id;
                            *buf += r#"&tab=note" role="button">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/refresh-outline.svg">&nbsp;再読み込み"#;
                            }
                            *buf += r#"</a>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }

                *buf += r#"<input type="hidden" name="ticket_id" value=""#;
                *buf += &t.id;
                *buf += r#"">"#;
                *buf += r#"<input type="hidden" name="project_id" value=""#;
                *buf += &t.project_id;
                *buf += r#"">"#;

                *buf += r#"<input type="hidden" name="timestamp" value=""#;
                if let Some(up) = &t.updated_at {
                    *buf += &up.timestamp_micros().to_string();
                }
                *buf += r#"">"#;
            }
            *buf += r#"</form>"#;
        }
    }
}
