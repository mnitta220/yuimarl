use crate::{components::Component, model::news, Props};
use chrono::{DateTime, FixedOffset};

pub struct News {}

impl Component for News {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<table class="table table-sm table-hover">"#;
        {
            *buf += r#"<tbody>"#;
            {
                for news in &props.news {
                    *buf += r#"<tr>"#;
                    {
                        *buf += r#"<td>"#;
                        {
                            *buf += r#"<div class="row">"#;
                            {
                                *buf += r#"<div class="col-md-3">"#;
                                {
                                    *buf += r#"<small>"#;
                                    // UTCからJSTに変換
                                    let jst: DateTime<FixedOffset> = news
                                        .timestamp
                                        .with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
                                    *buf += &jst.format("%Y/%m/%d %H:%M").to_string();
                                    *buf += r#"</small>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-md-9">"#;
                                {
                                    match news.event_conv() {
                                        news::NewsEvent::ProjectMemberAdd => {
                                            *buf += r#"プロジェクト 「 <a href="/project?id="#;
                                            *buf += &news.project_id;
                                            *buf += r#"">"#;
                                            super::super::super::escape_html(
                                                &news.project_name,
                                                buf,
                                            );
                                            *buf += r#"</a> 」 に追加されました。"#;
                                        }
                                        news::NewsEvent::ProjectRoleUpdate => {}
                                        news::NewsEvent::ProjectMemberDelete => {
                                            *buf += r#"プロジェクト 「 "#;
                                            super::super::super::escape_html(
                                                &news.project_name,
                                                buf,
                                            );
                                            *buf += r#" 」 のメンバーから削除されました。"#;
                                        }
                                        news::NewsEvent::TicketMemberAdd => {
                                            if let Some(t) = &news.ticket {
                                                *buf += r#"チケット 「 <a href="/ticket?id="#;
                                                *buf += &t.id;
                                                *buf += r#"">"#;
                                                *buf += &t.id_disp;
                                                *buf += r#" : "#;
                                                super::super::super::escape_html(&t.name, buf);
                                                *buf += r#"</a> 」 の担当者に追加されました。"#;
                                            }
                                        }
                                        news::NewsEvent::TicketMemberDelete => {}
                                        news::NewsEvent::TicketUpdate => {
                                            if let Some(t) = &news.ticket {
                                                *buf += r#"チケット 「 <a href="/ticket?id="#;
                                                *buf += &t.id;
                                                *buf += r#"">"#;
                                                *buf += &t.id_disp;
                                                *buf += r#" : "#;
                                                super::super::super::escape_html(&t.name, buf);
                                                *buf += r#"</a> 」 が更新されました。"#;
                                            }
                                        }
                                        news::NewsEvent::ProjectDelete => {
                                            *buf += r#"プロジェクト 「 "#;
                                            super::super::super::escape_html(
                                                &news.project_name,
                                                buf,
                                            );
                                            *buf += r#" 」 が削除されました。"#;
                                        }
                                        _ => {}
                                    }
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</td>"#;

                        *buf += r#"<td class="text-right">"#;
                        {
                            *buf += r#"<a href="/news_del/"#;
                            *buf += &news.id;
                            *buf += r#"" title="閉じる">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/close-outline.svg">"#;
                            }
                            *buf += r#"</a>"#;
                        }
                        *buf += r#"</td>"#;
                    }
                    *buf += r#"</tr>"#;
                }
            }
            *buf += r#"</tbody>"#;
        }
        *buf += r#"</table>"#;
    }
}
