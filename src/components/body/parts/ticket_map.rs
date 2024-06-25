use crate::{components::Component, Props};

pub struct TicketMap {
    pub can_update: bool,
}

impl Component for TicketMap {
    fn write(&self, props: &Props, buf: &mut String) {
        if let Some(p) = &props.project {
            *buf += r#"<form action="/project_note" method="POST">"#;
            {
                // チケットマップ
                *buf += r#"<div class="row py-2">"#;
                {
                    *buf += r#"<div class="col">"#;
                    {
                        *buf += r#"<canvas class="px-2 ticketmap" id="cnvs"></canvas>"#;
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

                            *buf += r#"<a class="btn btn-primary" href="/project?id="#;
                            *buf += &p.id;
                            *buf += r#"&tab=map" role="button">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/refresh-outline.svg">&nbsp;再読み込み"#;
                            }
                            *buf += r#"</a>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
            }
            *buf += r#"</form>"#;
        }
    }
}
