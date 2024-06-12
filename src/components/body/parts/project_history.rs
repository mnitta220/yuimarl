use crate::{components::Component, model, Props};

pub struct ProjectHistory {}

impl Component for ProjectHistory {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<div class="pb-2">"#;
        {
            *buf += r#"<div class="row">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<div class="form-floating">"#;
                    {
                        *buf += r#"<table class="table table-hover">"#;
                        {
                            *buf += r#"<thead>"#;
                            {
                                *buf += r#"<tr>"#;
                                {
                                    *buf += r#"<th scope="col">日時 (JST)</th>"#;
                                    *buf += r#"<th scope="col">更新者</th>"#;
                                    *buf += r#"<th scope="col">更新箇所</th>"#;
                                }
                                *buf += r#"</tr>"#;
                            }
                            *buf += r#"</thead>"#;

                            if let Some(p) = &props.project {
                                if let Some(h) = &p.history {
                                    let histories: Vec<model::history::History> =
                                        match serde_json::from_str(&h) {
                                            Ok(h) => h,
                                            Err(_) => Vec::new(),
                                        };
                                    *buf += r#"<tbody>"#;
                                    {
                                        for history in histories {
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<td>"#;
                                                super::super::super::utc_to_jst_time(
                                                    &history.timestamp,
                                                    buf,
                                                );
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>"#;
                                                *buf += &history.user_name;
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>"#;
                                                *buf += &history.history_to_string();
                                                *buf += r#"</td>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                        }
                                    }
                                    *buf += r#"</tbody>"#;
                                }
                            }
                        }
                        *buf += r#"</table>"#;
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
