// ローディング画面
pub fn loading(buf: &mut String) {
    *buf += r#"<div class="position-absolute top-0 start-0 w-100 h-100 d-none" id="loading""#;
    *buf += r#"style="background: rgba(0, 0, 0, .5); z-index: 10000;">"#;
    {
        *buf +=
            r#"<div class="text-center position-absolute top-50 start-50 w-100 translate-middle">"#;
        {
            *buf += r#"<div class="spinner-border text-light" role="status">"#;
            {
                *buf += r#"<span class="sr-only"></span>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</div>"#;
    }
    *buf += r#"</div>"#;
}
