use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;
use chrono::{DateTime, FixedOffset};

pub struct ProjectListBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub owner_cnt: i32,
}

impl ProjectListBody {
    pub fn new(owner_cnt: i32) -> Self {
        ProjectListBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
            owner_cnt,
        }
    }
}

impl Component for ProjectListBody {
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
                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<h3>プロジェクト一覧</h3>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<table class="table table-hover">"#;
                                {
                                    *buf += r#"<thead>"#;
                                    {
                                        *buf += r#"<tr>"#;
                                        {
                                            *buf += r#"<th scope="col">プロジェクト名</th>"#;
                                            *buf += r#"<th scope="col">ロール</th>"#;
                                            *buf += r#"<th scope="col">最新使用日</th>"#;
                                            *buf += r#"<th scope="col"></th>"#;
                                        }
                                        *buf += r#"</tr>"#;
                                    }
                                    *buf += r#"</thead>"#;

                                    *buf += r#"<tbody>"#;
                                    {
                                        let mut i = 0;
                                        for prj in &props.project_members {
                                            if let None = prj.project_name {
                                                continue;
                                            }

                                            *buf += r#"<tr"#;
                                            if i == 0 {
                                                *buf += r#" class="table-light""#;
                                            }
                                            *buf += r#">"#;
                                            {
                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += r#"<a href="/project?id="#;
                                                    *buf += &prj.project_id;
                                                    *buf += r#"">"#;
                                                    if let Some(name) = &prj.project_name {
                                                        super::super::escape_html(&name, buf);
                                                    }
                                                    *buf += r#"</a>"#;
                                                }
                                                *buf += r#"</td>"#;

                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += &prj.role_to_string();
                                                }
                                                *buf += r#"</td>"#;

                                                *buf += r#"<td>"#;
                                                {
                                                    if let Some(dt) = &prj.last_used {
                                                        // UTCからJSTに変換
                                                        let jst: DateTime<FixedOffset> = dt
                                                            .with_timezone(
                                                                &FixedOffset::east_opt(9 * 3600)
                                                                    .unwrap(),
                                                            );
                                                        *buf += &jst.format("%Y/%m/%d").to_string();
                                                    }
                                                }
                                                *buf += r#"</td>"#;

                                                *buf += r#"<td>"#;
                                                if i == 0 {
                                                    *buf += r#"<div class="badge bg-secondary text-light">選択中</div>"#;
                                                } else {
                                                    *buf += r#"<a href="/project_select/"#;
                                                    *buf += &prj.project_id;
                                                    *buf += r#"" title="選択">"#;
                                                    {
                                                        *buf += r#"<img class="icon3" src="/static/ionicons/open-outline.svg">"#;
                                                    }
                                                    *buf += r#"</a>"#;
                                                    *buf += r#"<span class="btn-help">&nbsp;選択</span>"#;
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
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        if self.owner_cnt < 10 {
                            *buf += r#"<div class="row">"#;
                            {
                                *buf += r#"<div class="col">"#;
                                {
                                    *buf += r#"<a href="/project_add" title="プロジェクトを作成">"#;
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
