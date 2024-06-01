use crate::{components::Component, Props};

pub struct TicketComment {}

impl Component for TicketComment {
    fn write(&self, _props: &Props, buf: &mut String) {
        *buf += r#"<form action="/ticket_add" method="POST">"#;
        {
            *buf += r#"<div class="row py-2 ticket-comment">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col-6"><b>殿馬一人</b></div>"#;
                        *buf +=
                            r#"<div class="col-6 text-end"><small>2024/09/04 18:49</small></div>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col">"#;
                        {
                            *buf += r#"たこやきのレシピを見つけました。<br>"#;
                            *buf += r#"<a href="https://www.otafuku.co.jp/recipe/cook/taco/taco01.html" target="_blank">https://www.otafuku.co.jp/recipe/cook/taco/taco01.html</a>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="row py-2 ticket-comment">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col-6"><b>山田太郎</b></div>"#;
                        *buf += r#"<div class="col-6 text-end">"#;
                        {
                            *buf += r#"<a href="">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/create-outline2.svg" title="編集">"#;
                            }
                            *buf += r#"</a>&nbsp;"#;
                            *buf += r#"<a href="">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/trash-outline.svg" title="削除">"#;
                            }
                            *buf += r#"</a>&nbsp;&nbsp;"#;
                            *buf += r#"<small>2024/09/05 16:23</small>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col">ありがとう！😍<br>チケットのノートに追加しました。</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="row py-2 ticket-comment">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col-6"><b>里中智</b></div>"#;
                        *buf +=
                            r#"<div class="col-6 text-end"><small>2024/09/05 17:05</small></div>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col">「薄力粉」って知りませんでした。</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
            {
                *buf += r#"<div class="col-9">"#;
                {
                    *buf += r#"<textarea class="form-control ticket-comment" id="message" rows="3" name="message"></textarea>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="col-3 text-end">"#;
                {
                    *buf += r#"<button class="btn btn-primary" type="submit">"#;
                    {
                        *buf += r#"<img class="icon" src="/static/ionicons/add-circle-outline2.svg">&nbsp;コメント"#;
                    }
                    *buf += r#"</button>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</form>"#;
    }
}
