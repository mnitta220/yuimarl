use crate::{components::Component, Props};

pub struct ProjectHistory {}

impl Component for ProjectHistory {
    fn write(&self, _props: &Props, buf: &mut String) {
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

                            *buf += r#"<tbody>"#;
                            {
                                *buf += r#"<tr>"#;
                                {
                                    *buf += r#"<td>2024/09/05 16:31</td>"#;
                                    *buf += r#"<td>山田太郎</td>"#;
                                    *buf += r#"<td>プロジェクト作成</td>"#;
                                }
                                *buf += r#"</tr>"#;
                                *buf += r#"<tr>"#;
                                {
                                    *buf += r#"<td>2024/09/05 16:31</td>"#;
                                    *buf += r#"<td>山田太郎</td>"#;
                                    *buf += r#"<td>プロジェクト作成</td>"#;
                                }
                                *buf += r#"</tr>"#;
                                *buf += r#"<tr>"#;
                                {
                                    *buf += r#"<td>2024/09/05 16:31</td>"#;
                                    *buf += r#"<td>山田太郎</td>"#;
                                    *buf += r#"<td>プロジェクト作成</td>"#;
                                }
                                *buf += r#"</tr>"#;
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
        }
        *buf += r#"</div>"#;
    }
}
