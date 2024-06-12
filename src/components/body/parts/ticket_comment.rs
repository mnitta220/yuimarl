use crate::{components::Component, handlers::validation, Props};
use chrono::{DateTime, FixedOffset};

pub struct TicketComment {
    pub can_update: bool,
    pub validation: Option<validation::ticket::TicketValidation>,
    pub is_edit: bool,
}

impl Component for TicketComment {
    fn write(&self, props: &Props, buf: &mut String) {
        if let Some(p) = &props.project {
            if let Some(t) = &props.ticket {
                *buf += r#"<form action="/comment_add" method="POST">"#;
                {
                    let mut i = 0;
                    for comment in &props.ticket_comments {
                        *buf += r#"<div class="row py-2 ticket-comment">"#;
                        {
                            // コメント表示
                            *buf += r#"<div class="col" id="view"#;
                            *buf += &i.to_string();
                            *buf += r#"">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-6"><b>"#;
                                    *buf += &comment.user_name;
                                    *buf += r#"</b></div>"#;
                                    *buf += r#"<div class="col-6 text-end">"#;
                                    {
                                        if let Some(s) = &props.session {
                                            if s.uid == comment.uid {
                                                *buf += r#"<a href="javascript:editComment("#;
                                                *buf += &i.to_string();
                                                *buf += r#")">"#;
                                                *buf += r#"<img class="icon" src="/static/ionicons/create-outline2.svg" title="編集">"#;
                                                *buf += r#"</a>&nbsp;"#;
                                                *buf += r#"<a href="javascript:deleteComment("#;
                                                *buf += &i.to_string();
                                                *buf += r#")">"#;
                                                *buf += r#"<img class="icon" src="/static/ionicons/trash-outline.svg" title="削除">"#;
                                                *buf += r#"</a>&nbsp;&nbsp;"#;
                                            }
                                        }

                                        *buf += r#"<small>"#;
                                        // UTCからJSTに変換
                                        let jst: DateTime<FixedOffset> =
                                            comment.timestamp.with_timezone(
                                                &FixedOffset::east_opt(9 * 3600).unwrap(),
                                            );
                                        *buf += &jst.format("%Y/%m/%d %H:%M").to_string();
                                        *buf += r#"</small>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col">"#;
                                    {
                                        super::super::super::escape_html2(&comment.comment, buf);
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // コメント編集
                            *buf += r#"<div class="col d-none" id="edit"#;
                            *buf += &i.to_string();
                            *buf += r#"">"#;
                            {
                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-6"><b>"#;
                                    *buf += &comment.user_name;
                                    *buf += r#"</b></div>"#;
                                    *buf += r#"<div class="col-6 text-end">"#;
                                    {
                                        *buf += r#"<small>"#;
                                        // UTCからJSTに変換
                                        let jst: DateTime<FixedOffset> =
                                            comment.timestamp.with_timezone(
                                                &FixedOffset::east_opt(9 * 3600).unwrap(),
                                            );
                                        *buf += &jst.format("%Y/%m/%d %H:%M").to_string();
                                        *buf += r#"</small>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="row">"#;
                                {
                                    *buf += r#"<div class="col-9">"#;
                                    {
                                        *buf += r#"<textarea class="form-control ticket-comment" id="message" rows="3" name="message">"#;
                                        {
                                            *buf += &comment.comment;
                                        }
                                        *buf += r#"</textarea>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="col-3 text-end">"#;
                                    {
                                        *buf += r#"<div class="row">"#;
                                        {
                                            *buf += r#"<div class="col d-flex justify-content-end px-1 pt-2">"#;
                                            {
                                                *buf += r#"<button class="btn btn-primary btn-sm" type="submit">更新</button>&nbsp;&nbsp;"#;
                                                *buf += r#"<a class="btn btn-secondary btn-sm" role="button" href="javascript:cancelComment("#;
                                                *buf += &i.to_string();
                                                *buf += r#")">キャンセル</a>"#;
                                                //*buf += r#"<button class="btn btn-secondary btn-sm" type="submit">キャンセル</button>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                        i += 1;
                    }

                    if self.can_update {
                        *buf += r#"<div class="row py-3 mt-2 bg-light" id="newComment">"#;
                        {
                            *buf += r#"<div class="col-9">"#;
                            {
                                *buf += r#"<textarea class="form-control ticket-comment"#;
                                if let Some(v) = &self.validation {
                                    if v.comment.is_some() {
                                        *buf += r#" is-invalid"#;
                                    }
                                }
                                *buf += r#"" id="comment" rows="3" name="comment"></textarea>"#;

                                if let Some(v) = &self.validation {
                                    if let Some(e) = &v.comment {
                                        *buf += r#"<div class="invalid-feedback">"#;
                                        *buf += e;
                                        *buf += r#"</div>"#;
                                    }
                                }
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-3 text-end">"#;
                            {
                                *buf += r#"<button class="btn btn-primary" type="submit" id="btnAddComment">"#;
                                {
                                    *buf += r#"<img class="icon" src="/static/ionicons/add-circle-outline2.svg">&nbsp;コメント"#;
                                }
                                *buf += r#"</button>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }

                    *buf += r#"<input type="hidden" id="project_id" name="project_id" value=""#;
                    *buf += &p.id;
                    *buf += r#"">"#;

                    *buf += r#"<input type="hidden" id="ticket_id" name="ticket_id" value=""#;
                    *buf += &t.id;
                    *buf += r#"">"#;
                }
                *buf += r#"</form>"#;
            }
        }
    }
}
