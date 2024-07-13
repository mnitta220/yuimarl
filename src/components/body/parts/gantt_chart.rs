use crate::{components::Component, Props};

pub struct GanttChart {
    pub can_update: bool,
}

impl Component for GanttChart {
    fn write(&self, props: &Props, buf: &mut String) {
        if let Some(p) = &props.project {
            *buf += r#"<form action="/project_note" method="POST">"#;
            {
                // ガントチャート
                *buf += r#"<div class="row py-2">"#;
                {
                    *buf += r#"<div class="col flexbox" id="flexbox">"#;
                    {
                        *buf += r#"<div class="ganttbase" id="ganttbase">"#;
                        {
                            *buf += r#"<div class="ganttframe" id="ganttframe"></div>"#;
                            *buf += r#"<canvas class="scrollH" id="scrollh"></canvas>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="scrollV">"#;
                        {
                            *buf += r#"<div class="sv1"></div>"#;
                            *buf += r#"<canvas class="sv2" id="scrollv"></canvas>"#;
                            *buf += r#"<div class="sv3"></div>"#;
                        }
                        *buf += r#"</div>"#;
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
