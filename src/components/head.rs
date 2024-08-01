use crate::{components::Component, Props, Screen, Tab};

pub struct Head {}

impl Component for Head {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<head>"#;
        {
            *buf += r#"<meta charset="utf-8">"#;
            *buf += r#"<meta name="viewport" content="width=device-width, initial-scale=1">"#;
            *buf += r#"<title>"#;
            if let Some(title) = props.title.as_ref() {
                *buf += title;
                *buf += r#" - "#;
            }
            *buf += r#"Yuimarl</title>"#;
            *buf += r#"<link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" "#;
            *buf += r#"rel="stylesheet" "#;
            *buf += r#"integrity="sha384-9ndCyUaIbzAi2FUVXJi0CjmCapSmO7SnpJef0486qhLnuZ2cdeRhO02iuK6FUUVM" "#;
            *buf += r#"crossorigin="anonymous">"#;
            *buf += r#"<link rel="stylesheet" href="/static/css/style1013a.css">"#;
            *buf += r#"<link rel="icon" type="image/x-icon" href="/static/favicon2.ico">"#;

            if let Some(screen) = &props.screen {
                match screen {
                    Screen::ProjectInfo => {
                        match &props.tab {
                            Tab::Info => {
                                // 次の行は、vite/project-info フォルダでビルドして出力された index-XXXXXXXX.js を設定する。
                                // (参照) vite/project-info/README.md
                                *buf += r#"<script type="module" crossorigin src="/static/js/project-info/index-TqQ--d0M.js"></script>"#;
                            }
                            Tab::Note => {
                                // 次の行は、vite/note フォルダでビルドして出力された index-XXXXXXXX.js を設定する。
                                // (参照) vite/note/README.md
                                *buf += r#"<script type="module" crossorigin src="/static/js/note/index-ClrermOc.js"></script>"#;
                            }
                            Tab::GanttChart => {
                                // 以下の2行は、gantt-chart フォルダでビルドして出力された index-XXXXXXXX.js と index-XXXXXXXX.css を設定する。
                                // (参照) gantt-chart/README.md
                                *buf += r#"<script type="module" crossorigin src="/static/js/gantt-chart/index-BlGiFZq-.js"></script>"#;
                                *buf += r#"<link rel="stylesheet" crossorigin href="/static/js/gantt-chart/index-BZzXyAxC.css">"#;
                            }
                            _ => {}
                        }
                    }
                    Screen::TicketInfo => {
                        match &props.tab {
                            Tab::Info => {
                                // 次の行は、vite/ticket-info フォルダでビルドして出力された index-XXXXXXXX.js を設定する。
                                // (参照) vite/ticket-info/README.md
                                *buf += r#"<script type="module" crossorigin src="/static/js/ticket-info/index-Bry6yV1i.js"></script>"#;
                            }
                            Tab::Note => {
                                // 次の行は、vite/note フォルダでビルドして出力された index-XXXXXXXX.js を設定する。
                                // (参照) vite/note/README.md
                                *buf += r#"<script type="module" crossorigin src="/static/js/note/index-ClrermOc.js"></script>"#;
                            }
                            Tab::Comment => {
                                // 次の行は、vite/comment フォルダでビルドして出力された index-XXXXXXXX.js を設定する。
                                // (参照) vite/comment/README.md
                                *buf += r#"<script type="module" crossorigin src="/static/js/comment/index-pehMYcZQ.js"></script>"#;
                            }
                            _ => {}
                        }
                    }
                    Screen::TicketList => {
                        // 次の行は、vite/ticket-list フォルダでビルドして出力された index-XXXXXXXX.js を設定する。
                        // (参照) vite/ticket-list/README.md
                        *buf += r#"<script type="module" crossorigin src="/static/js/ticket-list/index-Lp42aFvT.js"></script>"#;
                    }
                }
            }
        }
        *buf += r#"</head>"#;
    }
}
