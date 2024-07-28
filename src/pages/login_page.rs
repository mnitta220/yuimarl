/// Component for rendering the login page
pub struct LoginPage {}

impl LoginPage {
    pub fn write() -> String {
        let mut buf = String::with_capacity(2_000);

        buf += r#"<!DOCTYPE html>"#;
        buf += r#"<html lang="ja">"#;
        {
            buf += r#"<head>"#;
            {
                buf += r#"<meta charset="utf-8">"#;
                buf += r#"<meta name="viewport" content="width=device-width, initial-scale=1">"#;
                buf += r#"<title>Yuimarl ログイン</title>"#;
                buf += r#"<script src="https://www.gstatic.com/firebasejs/10.12.2/firebase-app-compat.js"></script>"#;
                buf += r#"<script src="https://www.gstatic.com/firebasejs/10.12.2/firebase-auth-compat.js"></script>"#;
                buf += r#"<script src="https://www.gstatic.com/firebasejs/ui/6.1.0/firebase-ui-auth__ja.js"></script>"#;
                buf += r#"<link type="text/css" rel="stylesheet" href="https://www.gstatic.com/firebasejs/ui/6.1.0/firebase-ui-auth.css" />"#;
                buf += r#"<link rel="icon" type="image/x-icon" href="/static/favicon.ico">"#;
            }
            buf += r#"</head>"#;

            buf += r#"<body>"#;
            {
                buf += r#"<h1 style="text-align: center">ログイン - Yuimarl</h1>"#;
                buf += r#"<div id="firebaseui-auth-container"></div>"#;
                buf += r#"<div id="loader">Loading...</div>"#;

                buf += r#"<form id="loginForm" method="POST" action="/login">"#;
                {
                    buf += r#"<input type=hidden id="display_name" name="display_name">"#;
                    buf += r#"<input type=hidden id="email" name="email">"#;
                    buf += r#"<input type=hidden id="photo_url" name="photo_url">"#;
                    buf += r#"<input type=hidden id="uid" name="uid">"#;
                    buf += r#"<input type=hidden id="locale" name="locale">"#;
                }
                buf += r#"</form>"#;

                buf += r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
                buf += r#"<script src="/static/js/login1015.js"></script>"#;
            }
            buf += r#"</body>"#;
        }
        buf += r#"</html>"#;

        buf
    }
}
