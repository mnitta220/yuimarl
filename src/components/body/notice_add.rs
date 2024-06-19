use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::{handlers::validation, Props};

pub struct NoticeAddBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub message: Option<String>,
    pub password: Option<String>,
    pub validation: Option<validation::notice::NoticeValidation>,
    pub notice_id: Option<String>,
}

impl NoticeAddBody {
    pub fn new(
        message: Option<String>,
        password: Option<String>,
        validation: Option<validation::notice::NoticeValidation>,
        notice_id: Option<String>,
    ) -> Self {
        NoticeAddBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
            message,
            password,
            validation,
            notice_id,
        }
    }
}

impl Component for NoticeAddBody {
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
                        *buf += r#"<h3 class="mb-3">運用からのお知らせ</h3>"#;
                        *buf +=
                            r#"<p>システムの運用担当者から、利用者全員への通知を行います。<br>"#;
                        *buf += r#"通知メッセージを入力し、システム環境変数に設定した「NOTICE_PASSWORD」の値を入力して、「通知」をクリックしてください。</p>"#;

                        *buf += r#"<form action="/notice_add" method="POST">"#;
                        {
                            // 通知メッセージ
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="message">通知メッセージ (HTML タグ)</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<textarea class="form-control"#;
                                    if let Some(v) = &self.validation {
                                        if v.message.is_some() {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="message" name="message" rows="3" required>"#;
                                    if let Some(m) = &self.message {
                                        *buf += m;
                                    }
                                    *buf += r#"</textarea>"#;

                                    if let Some(v) = &self.validation {
                                        if let Some(e) = &v.message {
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
                                    *buf += r#"<button class="btn btn-primary" type="submit">通知</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;

                        if let Some(n) = &self.notice_id {
                            *buf += r#"<div class="alert alert-primary mt-3">"#;
                            {
                                *buf += r#"通知が行われました。<br>通知IDは 「 "#;
                                *buf += n;
                                *buf += r#" 」 です。"#;
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
