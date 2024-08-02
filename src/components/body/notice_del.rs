use super::super::Component;
use super::parts::nav::Nav;
use crate::{components::footer::footer::Footer, handlers::validation, Props};

pub struct NoticeDelBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub notice_id: Option<String>,
    pub password: Option<String>,
    pub validation: Option<validation::notice::NoticeValidation>,
}

impl NoticeDelBody {
    pub fn new(
        notice_id: Option<String>,
        password: Option<String>,
        validation: Option<validation::notice::NoticeValidation>,
    ) -> Self {
        NoticeDelBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
            notice_id,
            password,
            validation,
        }
    }
}

impl Component for NoticeDelBody {
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
                        *buf += r#"<h3 class="mb-3">運用からのお知らせ削除</h3>"#;
                        *buf += r#"<p>通知済みのメッセージを削除するには、通知 ID とシステム環境変数「NOTICE_PASSWORD」の値を入力して、「削除」をクリックしてください。</p>"#;

                        *buf += r#"<form action="/notice_del" method="POST">"#;
                        {
                            // 通知メッセージID
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="notice-id">通知メッセージ ID</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control"#;
                                    if let Some(v) = &self.validation {
                                        if v.notice_id.is_some() {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="notice_id" name="notice_id" type="text" maxlength="40" value=""#;
                                    if let Some(p) = &self.notice_id {
                                        *buf += p;
                                    }
                                    *buf += r#"" required>"#;

                                    if let Some(v) = &self.validation {
                                        if let Some(e) = &v.notice_id {
                                            *buf += r#"<div class="invalid-feedback">"#;
                                            *buf += e;
                                            *buf += r#"</div>"#;
                                        }
                                    }
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // NOTICE_PASSWORD
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="password">NOTICE_PASSWORD</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control"#;
                                    if let Some(v) = &self.validation {
                                        if v.notice_password.is_some() {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="password" name="password" type="password" maxlength="40" value=""#;
                                    if let Some(p) = &self.password {
                                        *buf += p;
                                    }
                                    *buf += r#"" required>"#;

                                    if let Some(v) = &self.validation {
                                        if let Some(e) = &v.notice_password {
                                            *buf += r#"<div class="invalid-feedback">"#;
                                            *buf += e;
                                            *buf += r#"</div>"#;
                                        }
                                    }
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                            {
                                *buf += r#"<div class="col">"#;
                                {
                                    *buf += r#"<button class="btn btn-primary" type="submit">削除</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;

                        if let Some(v) = &self.validation {
                            if v.result {
                                if let Some(n) = &self.notice_id {
                                    *buf += r#"<div class="alert alert-primary mt-3">"#;
                                    {
                                        *buf += r#"通知ID 「 "#;
                                        *buf += n;
                                        *buf += r#" 」 が削除されました。"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                            }
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
