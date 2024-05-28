use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct DbCheckBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl DbCheckBody {
    pub fn new() -> Self {
        DbCheckBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for DbCheckBody {
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
                        *buf += r#"<h3 class="mb-3">データベースチェック</h3>"#;
                        *buf += r#"<p>システム管理者が、Firestore のドキュメントとインデックスの作成状態のチェックを行います。<br>"#;
                        *buf += r#"システム環境変数に設定した「DB_CHECK_PASSWORD」の値を入力して、「実行」をクリックしてください。</p>"#;

                        if let Some(v) = &props.db_check_validation {
                            if let Some(e) = &v.info {
                                *buf += r#"<div class="row p-2">"#;
                                {
                                    *buf += r#"<div class="alert alert-danger text-start" role="alert">"#;
                                    *buf += e;
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                        }

                        *buf += r#"<form action="/db_check" method="POST">"#;
                        {
                            // DB_CHECK_PASSWORD
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="db_check_password">DB_CHECK_PASSWORD</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control"#;
                                    if let Some(v) = &props.db_check_validation {
                                        if v.db_check_password.is_some() {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="db_check_password" name="db_check_password" type="password" maxlength="40" value=""#;
                                    if let Some(p) = &props.db_check_password {
                                        *buf += p;
                                    }
                                    *buf += r#"" required>"#;

                                    if let Some(v) = &props.db_check_validation {
                                        if let Some(e) = &v.db_check_password {
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
                                    *buf += r#"<button class="btn btn-primary" type="submit">実行</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;

                        if let Some(v) = &props.db_check_validation {
                            if v.result {
                                *buf += r#"<div class="alert alert-primary mt-3">チェックが正常に完了しました。</div>"#;
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
