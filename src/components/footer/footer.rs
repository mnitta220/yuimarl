use crate::{components::Component, Props};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Footer {}

impl Component for Footer {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<footer class="py-1 mt-3 bg-dark text-light">"#;
        {
            *buf += r#"<div class="container text-center pt-1">"#;
            {
                *buf += r#"<ul class="nav justify-content-center">"#;
                {
                    *buf += r#"<li class="nav-item">"#;
                    {
                        *buf += r#"<a class="nav-link" href="/">ホーム</a>"#;
                    }
                    *buf += r#"</li>"#;

                    *buf += r#"<li class="nav-item">"#;
                    {
                        *buf += r#"<a class="nav-link" href="https://mnitta220.github.io/yuimarl/" target="_blank">製品ホームページ</a>"#;
                    }
                    *buf += r#"</li>"#;

                    *buf += r#"<li class="nav-item">"#;
                    {
                        *buf += r#"<a class="nav-link" href="https://mnitta220.github.io/yuimarl/agreement.html" target="_blank">利用規約</a>"#;
                    }
                    *buf += r#"</li>"#;

                    *buf += r#"<li class="nav-item">"#;
                    {
                        *buf += r#"<a class="nav-link" href="https://mnitta220.github.io/yuimarl/index.html#contact" target="_blank">お問い合わせ</a>"#;
                    }
                    *buf += r#"</li>"#;

                    *buf += r#"<li class="nav-item">"#;
                    {
                        *buf += r#"<a class="nav-link" href="https://mnitta220.github.io/" target="_blank">新田システム事務所</a>"#;
                    }
                    *buf += r#"</li>"#;
                }
                *buf += r#"</ul>"#;

                *buf += r#"<p style="color: #a0a0a0">"#;
                {
                    *buf += r#"<small>"#;
                    {
                        *buf += r#"Yuimarl version "#;
                        *buf += VERSION;
                        *buf += r#"&nbsp;&nbsp;&nbsp;Copyright &copy; 2024-2025 Masahiro Nitta"#;
                    }
                    *buf += r#"</small>"#;
                }
                *buf += r#"</p>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</footer>"#;

        /*
        if let Some(t) = &props.toast_message {
            // トースト表示
            *buf += r#"<div id="toast_container">"#;
            {
                *buf += r#"<div id="toast_message" class="toast fade show" role="alert" aria-live="assertive" aria-atomic="true" data-testid="toast" data-bs-delay="2000">"#;
                {
                    *buf += r#"<div class="toast-body">"#;
                    *buf += &t;
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<script>"#;
            {
                *buf += r#"window.addEventListener("load", () => {"#;
                {
                    *buf += r#"setTimeout(function () { "#;
                    {
                        *buf += r#"const toastMessage = document.querySelector(`#toast_message`);"#;
                        *buf += r#"if (toastMessage) {"#;
                        {
                            *buf += r#"toastMessage.style.visibility = "hidden";"#;
                        }
                        *buf += r#"}"#;
                    }
                    *buf += r#"}, 2000);"#;
                }
                *buf += r#"});"#;
            }
            *buf += r#"</script>"#;
        }
        */

        if let Some(t) = &props.toast_message {
            *buf += r#"<div id="toast_container">"#;
            {
                *buf += r#"<div id="toast_message" class="toast fade show text-bg-secondary border-0" role="alert" aria-live="assertive" aria-atomic="true">"#;
                {
                    *buf += r#"<div id="toast_body" class="toast-body" data-testid="toast">"#;
                    *buf += &t;
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
            /*
            if t.len() > 0 {
                // トースト表示
                *buf += r#"<div id="toast"><span id="toast_message" data-testid="toast">"#;
                *buf += &t;
                *buf += r#"</span></div>"#;

                *buf += r#"<script>"#;
                {
                    *buf += r#"window.addEventListener("load", () => {"#;
                    {
                        *buf += r#"setTimeout(function () { "#;
                        {
                            *buf += r#"toast.style.visibility = "visible"; "#;
                            *buf += r#"setTimeout(function () { "#;
                            {
                                *buf += r#"toast.style.visibility = "hidden"; "#;
                            }
                            *buf += r#"}, 1500);"#;
                        }
                        *buf += r#"}, 100);"#;
                    }
                    *buf += r#"});"#;
                }
                *buf += r#"</script>"#;
            }
            */
            // トースト表示
            *buf += r#"<script>"#;
            {
                *buf += r#"window.addEventListener("load", () => {"#;
                {
                    *buf += r#"const toastMessage = document.querySelector(`#toast_message`);"#;
                    *buf += r#"if (toastMessage) {"#;
                    {
                        *buf += r#"setTimeout(function () { "#;
                        {
                            *buf += r#"toastMessage.classList.remove("show");"#;
                        }
                        *buf += r#"}, 2000);"#;
                    }
                    *buf += r#"}"#;
                }
                *buf += r#"});"#;
            }
            *buf += r#"</script>"#;
        }

        *buf += r#"<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js" "#;
        *buf += r#"integrity="sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz" "#;
        *buf += r#"crossorigin="anonymous"></script>"#;
    }
}
