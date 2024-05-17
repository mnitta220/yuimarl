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
                // お知らせ
                *buf += r#"<div class="pt-3">"#;
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
                                        *buf += r#"<div class="col">"#;
                                        {
                                            *buf += r#"<table class="table table-sm table-hover">"#;
                                            {
                                                *buf += r#"<tbody>"#;
                                                {
                                                    *buf += r#"<tr>"#;
                                                    {
                                                        *buf += r#"<td>"#;
                                                        {
                                                            *buf += r#"<div class="row">"#;
                                                            {
                                                                *buf += r#"<div class="col-md-3">"#;
                                                                {
                                                                    *buf += r#"<small>2024/09/06 17:03</small>"#;
                                                                }
                                                                *buf += r#"</div>"#;
                                                                *buf += r#"<div class="col-md-9">"#;
                                                                {
                                                                    *buf += r#"チケット&nbsp;"#;
                                                                    *buf += r#"<a href="">BN10 : たこやき模擬店</a>"#;
                                                                    *buf += r#"&nbsp;が更新されました。"#;
                                                                }
                                                                *buf += r#"</div>"#;
                                                            }
                                                            *buf += r#"</div>"#;
                                                        }
                                                        *buf += r#"</td>"#;
                                                        *buf += r#"<td class="text-right">"#;
                                                        {
                                                            *buf += r#"<a href="step02.html" title="閉じる">"#;
                                                            {
                                                                *buf += r#"<img class="icon" src="/static/ionicons/close-outline.svg">"#;
                                                            }
                                                            *buf += r#"</a>"#;
                                                        }
                                                        *buf += r#"</td>"#;
                                                    }
                                                    *buf += r#"</tr>"#;
                                                    *buf += r#"<tr>"#;
                                                    {
                                                        *buf += r#"<td>"#;
                                                        {
                                                            *buf += r#"<div class="row">"#;
                                                            {
                                                                *buf += r#"<div class="col-md-3">"#;
                                                                {
                                                                    *buf += r#"<small>2024/09/08 15:41</small>"#;
                                                                }
                                                                *buf += r#"</div>"#;
                                                                *buf += r#"<div class="col-md-9">"#;
                                                                {
                                                                    *buf += r#"プロジェクト&nbsp;"#;
                                                                    *buf += r#"<a href="">英語強化プロジェクト</a>"#;
                                                                    *buf += r#"&nbsp;に追加されました。"#;
                                                                }
                                                                *buf += r#"</div>"#;
                                                            }
                                                            *buf += r#"</div>"#;
                                                        }
                                                        *buf += r#"</td>"#;
                                                        *buf += r#"<td class="text-right">"#;
                                                        {
                                                            *buf += r#"<a href="step02.html" title="閉じる">"#;
                                                            {
                                                                *buf += r#"<img class="icon" src="/static/ionicons/close-outline.svg">"#;
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

                *buf += r#"<form action="/ticket/add" method="POST">"#;
                {
                    // プロジェクト
                    *buf += r#"<div class="py-3 bg-light">"#;
                    {
                        *buf += r#"<section id="project">"#;
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
                                        if let Some(p) = &props.project {
                                            *buf += r#"<div class="row pb-3">"#;
                                            {
                                                *buf += r#"<div class="col">"#;
                                                {
                                                    if let Some(n) = &p.project_name {
                                                        *buf += r#"<a href="/project/"#;
                                                        *buf += &p.id.clone().unwrap();
                                                        *buf += r#"">"#;
                                                        {
                                                            *buf += n;
                                                        }
                                                        *buf += r#"</a>"#;
                                                        if let Some(m) = &props.member {
                                                            *buf += r#"&nbsp;&nbsp;<small>"#;
                                                            {
                                                                *buf += r#"<span class="badge bg-secondary text-light">"#;
                                                                {
                                                                    *buf += &m.role_to_string();
                                                                }
                                                                *buf += r#"</span>"#;
                                                            }
                                                            *buf += r#"</small>"#;
                                                        }
                                                    }
                                                }
                                                *buf += r#"</div>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }

                                        *buf += r#"<div class="row">"#;
                                        {
                                            *buf += r#"<div class="col">"#;
                                            {
                                                *buf += r#"<a href="/project/list" title="プロジェクト一覧">"#;
                                                {
                                                    *buf += r#"<img class="icon3" src="/static/ionicons/list-outline.svg">"#;
                                                }
                                                *buf += r#"</a>"#;
                                                if let None = &props.project {
                                                    *buf += r#"<span class="btn-help">&nbsp;: プロジェクト一覧</span>"#;
                                                }
                                                *buf += r#"&nbsp;&nbsp;&nbsp;"#;
                                                *buf += r#"<a href="/project/add" title="プロジェクトを作成">"#;
                                                {
                                                    *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg">"#;
                                                }
                                                *buf += r#"</a>"#;
                                                if let None = &props.project {
                                                    *buf += r#"<span class="btn-help">&nbsp;: プロジェクトを作成</span>"#;
                                                }
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
                        }
                        *buf += r#"</section>"#;
                    }
                    *buf += r#"</div>"#;

                    // チケット
                    //if props.projects.len() > 0 {
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
                                        *buf += r#"<p class="mb-0">"#;
                                        {
                                            *buf += r#"<a href="">BN10</a>&nbsp;:&nbsp;"#;
                                            *buf += r#"たこやき模擬店"#;
                                        }
                                        *buf += r#"</p>"#;
                                        *buf += r#"<p class="mb-0">"#;
                                        {
                                            *buf += r#"<a href="">BN5</a>&nbsp;:&nbsp;"#;
                                            *buf += r#"文化祭出し物"#;
                                        }
                                        *buf += r#"</p>"#;
                                        *buf += r#"<p class="my-1">"#;
                                        {
                                            *buf += r#"<a href="step01.html">"#;
                                            {
                                                *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="チケットを追加">"#;
                                            }
                                            *buf += r#"</a>"#;
                                        }
                                        *buf += r#"</p>"#;
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
                    //}
                }
                *buf += r#"</form>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
        }
        *buf += r#"</body>"#;
    }
}
