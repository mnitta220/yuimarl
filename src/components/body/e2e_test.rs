use super::super::Component;
use super::parts::nav::Nav;
use crate::{components::footer::footer::Footer, handlers::validation, Props};

pub struct E2eTestBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub password: Option<String>,
    pub validation: Option<validation::e2e_test::E2eTestValidation>,
}

impl E2eTestBody {
    pub fn new(
        password: Option<String>,
        validation: Option<validation::e2e_test::E2eTestValidation>,
    ) -> Self {
        E2eTestBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
            password,
            validation,
        }
    }
}

impl Component for E2eTestBody {
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
                        *buf += r#"<h3 class="mb-3">E2Eテスト</h3>"#;
                        *buf += r#"<p>E2Eテストを実行します。<br>"#;
                        *buf += r#"システム環境変数に設定した「E2E_TEST_PASSWORD」の値を入力し、ログインユーザーを選択して、「実行」をクリックしてください。</p>"#;

                        if let Some(v) = &self.validation {
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

                        *buf += r#"<form action="/e2e_test" method="POST">"#;
                        {
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="e2e_test_password">E2E_TEST_PASSWORD</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control"#;
                                    if let Some(v) = &self.validation {
                                        if v.e2e_test_password.is_some() {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="e2e_test_password" name="e2e_test_password" type="password" maxlength="40" value=""#;
                                    if let Some(p) = &self.password {
                                        *buf += p;
                                    }
                                    *buf += r#"" required>"#;

                                    if let Some(v) = &self.validation {
                                        if let Some(e) = &v.e2e_test_password {
                                            *buf += r#"<div class="invalid-feedback">"#;
                                            *buf += e;
                                            *buf += r#"</div>"#;
                                        }
                                    }
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="password">ログインユーザー</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<table class="table table-hover">"#;
                                    {
                                        *buf += r#"<thead>"#;
                                        {
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<th scope="col">選択</th>"#;
                                                *buf += r#"<th scope="col">メールアドレス</th>"#;
                                                *buf += r#"<th scope="col">名前</th>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                        }
                                        *buf += r#"</thead>"#;
                                        *buf += r#"<tbody>"#;
                                        {
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += r#"<input class="form-check-input" id="user" type="radio" name="user" value="1" checked>"#;
                                                }
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>taro.yamada@e2e_test.com</td>"#;
                                                *buf += r#"<td>山田太郎</td>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += r#"<input class="form-check-input" id="user" type="radio" name="user" value="2">"#;
                                                }
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>kazuto.tonoma@e2e_test.com</td>"#;
                                                *buf += r#"<td>殿馬一人</td>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += r#"<input class="form-check-input" id="user" type="radio" name="user" value="3">"#;
                                                }
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>masami.iwaki@e2e_test.com</td>"#;
                                                *buf += r#"<td>岩鬼正美</td>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += r#"<input class="form-check-input" id="user" type="radio" name="user" value="4">"#;
                                                }
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>satoru.satonaka@e2e_test.com</td>"#;
                                                *buf += r#"<td>里中智</td>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                        }
                                        *buf += r#"</tbody>"#;
                                    }
                                    *buf += r#"</table>"#;
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

                        if let Some(v) = &self.validation {
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
