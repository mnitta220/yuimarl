use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct AgreementBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub disagree: bool,
}

impl AgreementBody {
    pub fn new(disagree: bool) -> Self {
        AgreementBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
            disagree,
        }
    }
}

impl Component for AgreementBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            *buf += r#"<nav class="navbar navbar-expand-md navbar-dark bg-dark sticky-top">"#;
            {
                *buf += r#"<div class="container">"#;
                {
                    *buf += r#"<img class="mw-100" src="/static/images/logo.png" alt="Yuimarl">"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</nav>"#;

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="container pt-3 pb-1">"#;
                {
                    *buf += r#"<h2>利用規約</h2>"#;
                    *buf += r#"<p>"#;
                    *buf += r#"「Yuimarl（ゆいまーる）」（以下、「当システム」と記述します）をお使いいただくためには、以下の利用規約に同意していただく必要があります。"#;
                    *buf += r#"</p>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div>"#;
                {
                    *buf += r#"<div class="container">"#;
                    {
                        *buf += r#"<ol>"#;
                        {
                            *buf += r#"<li>"#;
                            *buf += r#"運用者（新田システム事務所）は、必要に応じて、当システムに保存されているデータを閲覧する場合があります。部外者、他人に絶対に見られたくない情報は、このシステムに登録しないでください。"#;
                            *buf += r#"</li>"#;

                            *buf += r#"<li>"#;
                            *buf += r#"データ消失、情報漏洩、一時的または永久的なシステム停止などにより、利用者が不利益をこうむったとしても、運用者は一切の補償を行いません。"#;
                            *buf += r#"</li>"#;

                            *buf += r#"<li>"#;
                            *buf += r#"当システムにおいて、誹謗中傷、反社会的活動、公序良俗に反する不適切な使用などが発覚した場合は、使用を停止させていただく場合があります。"#;
                            *buf += r#"</li>"#;

                            *buf += r#"<li>"#;
                            *buf += r#"長期間利用されていないアカウントやデータは、予告なく削除させていただく場合があります。"#;
                            *buf += r#"</li>"#;
                        }
                        *buf += r#"</ol>"#;

                        *buf += r#"<div class="pt-3 pb-4">"#;
                        {
                            *buf += r#"<form action="/agree" method="POST">"#;
                            {
                                *buf += r#"<button type="submit" class="btn btn-primary">同意する</button>&nbsp;&nbsp;"#;
                                *buf += r#"<a class="btn btn-secondary" href="/agree/no" role="button">同意しない</a>"#;
                            }
                            *buf += r#"</form>"#;
                        }
                        *buf += r#"</div>"#;

                        if self.disagree {
                            *buf += r#"<div class="alert alert-danger mb-3" role="alert">"#;
                            *buf += r#"利用規約に同意していただけない場合は、このシステムを使用することができません。"#;
                            *buf += r#"</div>"#;
                        }

                        *buf += r#"<div class="alert alert-info mb-4" role="alert">利用者が&nbsp;"#;
                        *buf += r#"<a href="https://mnitta220.github.io/yuimarl/deploy.html" target="_blank">ご自身でこのシステムのサーバーを構築する方法</a>"#;
                        *buf += r#"&nbsp;を公開しています。可能であれば、それによって利用することもご検討ください。"#;
                        *buf += r#"</div>"#;
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
