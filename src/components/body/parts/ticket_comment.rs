use crate::{components::Component, handlers::validation, Props};

pub struct TicketComment {
    pub can_update: bool,
    pub validation: Option<validation::ticket::TicketValidation>,
    pub is_edit: bool,
}

impl Component for TicketComment {
    fn write(&self, props: &Props, buf: &mut String) {
        let mut uid = "";
        if let Some(s) = &props.session {
            uid = &s.uid;
        }

        if let Some(p) = &props.project {
            if let Some(t) = &props.ticket {
                *buf += r#"<form id="post_comment" action="/post_comment" method="POST">"#;
                {
                    let mut i = 0;
                    for comment in &props.ticket_comments {
                        *buf += r#"<div class="row py-2 ticket-comment">"#;
                        {
                            *buf += r#"<div class="col" id="comment"#;
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
                                        if uid == comment.uid {
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

                                        *buf += r#"<small>"#;
                                        {
                                            super::super::super::utc_to_jst_time(
                                                &comment.timestamp,
                                                buf,
                                            );
                                            if comment.updated {
                                                *buf += r#" [更新済]"#;
                                            }
                                        }
                                        *buf += r#"</small>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="row">"#;
                                {
                                    *buf +=
                                        r#"<div class="col px-2 mx-2 bg-light preview2" id="pre"#;
                                    *buf += &i.to_string();
                                    *buf += r#"" name="pre"#;
                                    *buf += &i.to_string();
                                    *buf += r#""></div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<input type="hidden" id="cid"#;
                                *buf += &i.to_string();
                                *buf += r#"" name="cid"#;
                                *buf += &i.to_string();
                                *buf += r#"" value=""#;
                                *buf += &comment.id;
                                *buf += r#"">"#;

                                *buf += r#"<input type="hidden" id="mkd"#;
                                *buf += &i.to_string();
                                *buf += r#"" name="mkd"#;
                                *buf += &i.to_string();
                                *buf += r#"" value=""#;
                                *buf += &comment.comment;
                                *buf += r#"">"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                        i += 1;
                    }

                    if self.can_update {
                        *buf += r#"<div class="row py-2">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<a href="javascript:clickComment();">"#;
                                {
                                    *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="コメントを追加">"#;
                                }
                                *buf += r#"</a>"#;
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

                    *buf += r#"<input type="hidden" id="comment_id" name="comment_id" value="">"#;
                    *buf += r#"<input type="hidden" id="action" name="action" value="">"#;
                    *buf += r#"<input type="hidden" id="comment" name="comment" value="">"#;
                }
                *buf += r#"</form>"#;

                *buf += r#"<div class="modal fade" id="commentModal" tabindex="-1" aria-labelledby="commentModalLabel" aria-hidden="true">"#;
                {
                    *buf += r#"<div class="modal-dialog modal-xl">"#;
                    {
                        *buf += r#"<div class="modal-content">"#;
                        {
                            *buf += r#"<div class="modal-header">"#;
                            {
                                *buf += r#"<h1 class="modal-title fs-5" id="commentModalLabel">コメントを追加</h1>"#;
                                *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="modal-body">"#;
                            {
                                *buf += r#"<div class="row py-2" id="note1">"#;
                                {
                                    *buf += r#"<div class="col-lg-6">"#;
                                    {
                                        *buf += r#"<small>［マークダウン］</small>"#;
                                        *buf += r#"<textarea class="form-control" id="markdown" name="markdown" rows="10" required></textarea>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="col-lg-6">"#;
                                    {
                                        *buf += r#"<small>［プレビュー］</small>"#;
                                        *buf += r#"<div class="px-2 bg-light preview" id="preview1"></div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="modal-footer">"#;
                            {
                                *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                                *buf +=
                                    r#"<button id="btnAdd" class="btn btn-primary" type="button">"#;
                                {
                                    *buf += r#"<img class="icon" src="/static/ionicons/add-circle-outline2.svg">&nbsp;追加"#;
                                }
                                *buf += r#"</button>"#;
                                *buf += r#"<button id="btnUpdate" class="btn btn-primary" type="button">"#;
                                {
                                    *buf += r#"<img class="icon" src="/static/ionicons/save-outline.svg">&nbsp;更新"#;
                                }
                                *buf += r#"</button>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="modal fade" id="deleteCommentModal" tabindex="-1" aria-labelledby="deleteCommentModalLabel" aria-hidden="true">"#;
                {
                    *buf += r#"<div class="modal-dialog modal-md">"#;
                    {
                        *buf += r#"<div class="modal-content">"#;
                        {
                            *buf += r#"<div class="modal-header">"#;
                            {
                                *buf += r#"<h1 class="modal-title fs-5" id="deleteCommentModalLabel">コメント削除</h1>"#;
                                *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="modal-body">"#;
                            {
                                *buf += r#"<p>コメントを削除します。<br>よろしいですか？</p>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="modal-footer">"#;
                            {
                                *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                                *buf += r#"<button id="btnDelete" class="btn btn-danger" type="button">"#;
                                {
                                    *buf += r#"<img class="icon" src="/static/ionicons/trash-outline2.svg">&nbsp;削除"#;
                                }
                                *buf += r#"</button>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
        }
    }
}
