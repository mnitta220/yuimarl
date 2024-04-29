use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct HomeBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl HomeBody {
    pub fn new() -> Self {
        HomeBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for HomeBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="py-3">"#;
                {
                    *buf += r#"<section id="news">"#;
                    {
                        *buf += r#"<div class="container">"#;
                        {
                            *buf += r#"<div class="row">"#;
                            {
                                *buf += r#"<div class="col-md-3">"#;
                                {
                                    *buf += r#"<h3>お知らせ</h3>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-md-9 py-0">"#;
                                {
                                    *buf += r#"<dl class="row">"#;
                                    {
                                        *buf += r#"<dt class="col-md-4">2021年○月○日</dt>"#;
                                        *buf += r#"<dd class="col-md-8">ドリンクサービスクーポン配布中ですドリンクサービスクーポン配布中ですドリンクサービスクーポン配布中です</dd>"#;
                                        *buf += r#"<dt class="col-md-4">2021年○月○日</dt>"#;
                                        *buf += r#"<dd class="col-md-8">この季節だけの花木を追加しました</dd>"#;
                                        *buf += r#"<dt class="col-md-4">2021年○月○日</dt>"#;
                                        *buf += r#"<dd class="col-md-8">新しいプランター入荷しました</dd>"#;
                                    }
                                    *buf += r#"</dl>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</section>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<form action="/ticket/add" method="POST">"#;
                {
                    *buf += r#"<div class="py-3 bg-light">"#;
                    {
                        *buf += r#"<section id="news">"#;
                        {
                            *buf += r#"<div class="container">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-md-3">"#;
                                    {
                                        *buf += r#"<h3>プロジェクト</h3>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="col-md-9">"#;
                                    {
                                        *buf += r#"<dl class="row">"#;
                                        {
                                            if props.projects.len() > 0 {
                                                *buf += r#"<div class="col-md-8 mb-2">"#;
                                                {
                                                    *buf += r#"<select class="form-select" id="project" name="project">"#;
                                                    {
                                                        for project in &props.projects {
                                                            *buf += r#"<option value=""#;
                                                            *buf += &project.id;
                                                            *buf += r#"">"#;
                                                            *buf += &project.project_name;
                                                            *buf += r#"</option>"#;
                                                        }
                                                    }
                                                    *buf += r#"</select>"#;
                                                }
                                                *buf += r#"</div>"#;
                                            }

                                            *buf += r#"<div class="col-md-4 mb-1">"#;
                                            {
                                                *buf += r#"<a href="/project/add" class="btn btn-primary">プロジェクトを作成</a>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</dl>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</section>"#;
                    }
                    *buf += r#"</div>"#;

                    if props.projects.len() > 0 {
                        *buf += r#"<div class="py-3">"#;
                        {
                            *buf += r#"<section id="tickets" class="mb-3">"#;
                            {
                                *buf += r#"<div class="container">"#;
                                {
                                    *buf += r#"<div class="row">"#;
                                    {
                                        *buf += r#"<div class="col-md-3">"#;
                                        {
                                            *buf += r#"<h3>チケット</h3>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="col-md-9">"#;
                                        {
                                            *buf += r#"<dl class="row">"#;
                                            {
                                                *buf += r##"<dt class="col-md-3"><a href="#">YM1000</a></dt>"##;
                                                *buf += r#"<dd class="col-md-9">ドリンクサービスクーポン配布中です</dd>"#;
                                                *buf += r##"<dt class="col-md-3"><a href="#">YM1001</a></dt>"##;
                                                *buf += r#"<dd class="col-md-9">この季節だけの花木を追加しました</dd>"#;
                                                *buf += r##"<dt class="col-md-3"><a href="#">YM1002</a></dt>"##;
                                                *buf += r#"<dd class="col-md-9">新しいプランター入荷しました</dd>"#;
                                            }
                                            *buf += r#"</dl>"#;

                                            *buf += r#"<button type="submit" class="btn btn-primary">チケットを追加</button>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</section>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                }
                *buf += r#"</form>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
        }
        *buf += r#"</body>"#;
    }
}
