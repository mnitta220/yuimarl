use super::super::Component;
use super::parts::nav::Nav;
use crate::{components::footer::footer::Footer, Props};

pub struct UserNameBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl UserNameBody {
    pub fn new() -> Self {
        UserNameBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for UserNameBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            *buf += r#"<nav class="navbar navbar-expand-md navbar-dark bg-dark sticky-top">"#;
            {
                *buf += r#"<div class="container">"#;
                {
                    *buf += r#"<img class="mw-100" src="/static/images/logo2.png" alt="Yuimarl">"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</nav>"#;

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="container pt-3 pb-1">"#;
                {
                    *buf += r#"<h2>氏名入力</h2>"#;
                    *buf +=
                        r#"<p>あなたの氏名を入力してください。本名の入力をお願いいたします。</p>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="container">"#;
                {
                    *buf += r#"<form action="/user_name" method="POST">"#;
                    {
                        // 氏名
                        *buf += r#"<div class="row pt-1 pb-2">"#;
                        {
                            *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="name">氏名</label>"#;
                            *buf += r#"<div class="col-md-9 mb-1">"#;
                            {
                                *buf += r#"<input class="form-control" id="name" name="name" maxlength="60" value="" required>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<button class="btn btn-primary" type="submit">登録</button>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</form>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
        }
        *buf += r#"</body>"#;
    }
}
