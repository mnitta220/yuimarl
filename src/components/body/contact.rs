use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct ContactBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl ContactBody {
    pub fn new() -> Self {
        ContactBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for ContactBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="container">"#;
                {
                    *buf += r#"<h2>お問い合わせ</h2>"#;
                    *buf +=
                        r#"<p>Yuimarl への お問い合わせは、こちらのフォームをご利用ください。</p>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="py-3">"#;
                {
                    *buf += r#"<div class="container">"#;
                    {
                        *buf += r#"<h3 class="mb-3">お問い合わせフォーム</h3>"#;
                        *buf += r#"<form>"#;
                        {
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="name" class="col-md-3 col-form-label bg-light mb-1">"#;
                                {
                                    *buf += r#"お名前 <span class="badge bg-warning text-dark">必須</span>"#;
                                }
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-floating">"#;
                                    {
                                        *buf += r#"<input type="text" class="form-control" id="name" placeholder="みどり　はなこ" required>"#;
                                        *buf += r#"<label for="name">お名前を入力してください。</label>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="email" class="col-md-3 col-form-label bg-light mb-1">"#;
                                {
                                    *buf += r#"メールアドレス <span class="badge bg-warning text-dark">必須</span>"#;
                                }
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-floating">"#;
                                    {
                                        *buf += r#"<input type="email" class="form-control" id="email" placeholder="mail@example.com" "#;
                                        *buf += r#"required>"#;
                                        *buf += r#"<label for="email">有効な電子メールアドレスを入力してください。</label>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="category" class="col-md-3 col-form-label bg-light mb-1">"#;
                                {
                                    *buf += r#"お問い合わせ種類 <span class="badge bg-warning text-dark">必須</span>"#;
                                }
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-floating">"#;
                                    {
                                        *buf += r#"<select class="form-select" id="category" name="category">"#;
                                        {
                                            *buf += r#"<option value="category1">システムの仕様・運用について</option>"#;
                                            *buf += r#"<option value="category2">利用制限の解除について</option>"#;
                                            *buf += r#"<option value="category3">その他のお問い合わせ</option>"#;
                                        }
                                        *buf += r#"</select>"#;
                                        *buf += r#"<label for="category">お選びください。</label>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="message" class="col-md-3 col-form-label bg-light mb-1">"#;
                                {
                                    *buf += r#"お問い合わせ内容 <span class="badge bg-warning text-dark">必須</span>"#;
                                }
                                *buf += r#"</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-floating">"#;
                                    {
                                        *buf += r#"<textarea class="form-control" id="message" rows="8" name="message" "#;
                                        *buf += r#"placeholder="問い合わせ内容"></textarea>"#;
                                        *buf += r#"<label for="message">ご自由にお書きください。</label>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                            {
                                *buf += r#"<div class="col-md-9">"#;
                                {
                                    *buf += r#"<button type="submit" class="btn btn-primary">確認する</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;
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
