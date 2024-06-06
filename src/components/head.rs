use crate::{components::Component, Props};

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
            *buf += r#"<link rel="stylesheet" href="/static/css/style0029.css">"#;
            *buf += r#"<link rel="icon" type="image/x-icon" href="/static/favicon2.ico">"#;
        }
        *buf += r#"</head>"#;
    }
}
