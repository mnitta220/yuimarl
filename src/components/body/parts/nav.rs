use crate::{components::Component, Props};

pub struct Nav {}

impl Component for Nav {
    fn write(&self, _props: &Props, buf: &mut String) {
        *buf += r#"<nav class="navbar navbar-expand-md navbar-dark bg-dark sticky-top">"#;
        {
            *buf += r#"<div class="container">"#;
            {
                *buf += r#"<a class="navbar-brand" href="/">"#;
                {
                    *buf += r#"<img class="mw-100" src="/static/images/logo2.png" alt="Yuimarl">"#;
                }
                *buf += r#"</a>"#;

                *buf += r#"<button class="navbar-toggler" type="button" "#;
                *buf += r##"data-bs-toggle="collapse" data-bs-target="#navbar-content" "##;
                *buf += r#"aria-controls="navbar-content" aria-expanded="false" aria-label="Toggle navigation">"#;
                {
                    *buf += r#"<span class="navbar-toggler-icon"></span>"#;
                }
                *buf += r#"</button>"#;

                *buf += r#"<div class="collapse navbar-collapse" id="navbar-content">"#;
                {
                    *buf += r#"<ul class="navbar-nav me-auto">"#;
                    {
                        *buf += r#"<li class="nav-item active">"#;
                        {
                            *buf += r#"<a class="nav-link" href="/">ホーム<span class="visually-hidden">(current)</span></a>"#;
                        }
                        *buf += r#"</li>"#;

                        *buf += r#"<li class="nav-item">"#;
                        {
                            *buf += r##"<a class="nav-link" href="#about">プロジェクト</a>"##;
                        }
                        *buf += r#"</li>"#;

                        *buf += r#"<li class="nav-item">"#;
                        {
                            *buf += r##"<a class="nav-link" href="#items">チケット</a>"##;
                        }
                        *buf += r#"</li>"#;

                        *buf += r#"<li class="nav-item">"#;
                        {
                            *buf += r##"<a class="nav-link" href="#items">利用ガイド</a>"##;
                        }
                        *buf += r#"</li>"#;

                        *buf += r#"<li class="nav-item dropdown">"#;
                        {
                            *buf += r##"<a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" "##;
                            *buf += r#"data-bs-toggle="dropdown" aria-haspopup="true" aria-expanded="false">"#;
                            *buf += r#"インフォメーション"#;
                            *buf += r#"</a>"#;
                            *buf +=
                                r#"<div class="dropdown-menu" aria-labelledby="navbarDropdown">"#;
                            {
                                *buf += r##"<a class="dropdown-item" href="#shop">お店</a>"##;
                                *buf += r##"<a class="dropdown-item" href="#access">アクセス</a>"##;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</li>"#;
                    }
                    *buf += r#"</ul>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</nav>"#;
    }
}
