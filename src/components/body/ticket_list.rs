use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct TicketListBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl TicketListBody {
    pub fn new() -> Self {
        TicketListBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for TicketListBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="py-3">"#;
                {
                    *buf += r#"<div class="container">"#;
                    {
                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<a href="/project/list">文化祭実行プロジェクト</a>"#;
                                *buf += r#"&nbsp;&nbsp;"#;
                                *buf += r#"<small>"#;
                                {
                                    *buf += r#"<span class="badge bg-secondary text-light">オーナー</span>"#;
                                }
                                *buf += r#"</small>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row pt-3">"#;
                        {
                            *buf += r#"<h3>チケット一覧</h3>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row py-2 mx-1 border bg-light">"#;
                        {
                            *buf += r#"<div class="col-md-2 pb-1">"#;
                            {
                                *buf += r#"<label class="form-label" for="ticketid">チケットID</label>"#;
                                *buf += r#"<input class="form-control" id="ticketid" type="text" maxlength="20">"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-md-5 pb-1">"#;
                            {
                                *buf += r#"<label class="form-label" for="ticketname">チケット名&nbsp;&nbsp;<small>(部分一致)</small></label>"#;
                                *buf += r#"<input class="form-control" id="ticketname" type="text" maxlength="100">"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-md-2 pb-1">"#;
                            {
                                *buf += r#"<label class="form-label" for="parentid">親チケットID</label>"#;
                                *buf += r#"<input class="form-control" id="parentid" type="text" maxlength="20">"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-md-3 pb-1">"#;
                            {
                                *buf += r#"<div class="pb-2">"#;
                                {
                                    *buf += r#"<input class="form-check-input" id="finished" type="checkbox">"#;
                                    *buf += r#"<label class="form-check-label" for="finished">完了済を表示</label>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="text-end">"#;
                                {
                                    *buf +=
                                        r#"<button class="btn btn-sm btn-primary" type="button">"#;
                                    {
                                        *buf += r#"<img class="icon" src="/static/ionicons/funnel-outline.svg">&nbsp;フィルター"#;
                                    }
                                    *buf += r#"</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row pt-2">"#;
                        {
                            *buf += r#"<div class="col-md-9"></div>"#;
                            *buf += r#"<div class="col-md-3">"#;
                            {
                                *buf += r#"<nav>"#;
                                {
                                    *buf += r#"<ul class="pagination pagination-sm">"#;
                                    {
                                        *buf += r#"<li class="page-item disabled">"#;
                                        {
                                            *buf += r#"<a class="page-link" href="" tabindex="-1" aria-label="前へ">"#;
                                            {
                                                *buf +=
                                                    r#"<span aria-hidden="true">&laquo;</span>"#;
                                                *buf +=
                                                    r#"<span class="visually-hidden">前へ</span>"#;
                                            }
                                            *buf += r#"</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="page-item">"#;
                                        {
                                            *buf += r#"<a class="page-link active" href="">"#;
                                            {
                                                *buf += r#"1"#;
                                                *buf += r#"<span class="visually-hidden">(現ページ)</span>"#;
                                            }
                                            *buf += r#"</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="page-item">"#;
                                        {
                                            *buf += r#"<a class="page-link" href="">2</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="page-item">"#;
                                        {
                                            *buf += r#"<a class="page-link" href="">3</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="page-item">"#;
                                        {
                                            *buf += r#"<a class="page-link" href="" aria-label="次へ">"#;
                                            {
                                                *buf +=
                                                    r#"<span aria-hidden="true">&raquo;</span>"#;
                                                *buf +=
                                                    r#"<span class="visually-hidden">次へ</span>"#;
                                            }
                                            *buf += r#"</a>"#;
                                        }
                                        *buf += r#"</li>"#;
                                    }
                                    *buf += r#"</ul>"#;
                                }
                                *buf += r#"</nav>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<div class="table-responsive">"#;
                                {
                                    *buf += r#"<table class="table table-sm table-hover">"#;
                                    {
                                        *buf += r#"<thead>"#;
                                        {
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<th scope="col">ID / チケット名</th>"#;
                                                *buf += r#"<th scope="col">親チケット</th>"#;
                                                *buf += r#"<th scope="col">開始日</th>"#;
                                                *buf += r#"<th scope="col">終了日</th>"#;
                                                *buf += r#"<th class="text-right" scope="col">進捗率</th>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                        }
                                        *buf += r#"</thead>"#;

                                        *buf += r#"<tbody>"#;
                                        {
                                            *buf += r#"<tr>"#;
                                            {
                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += r#"<a href="">"#;
                                                    *buf += r#"BN1"#;
                                                    *buf += r#"</a>&nbsp;"#;
                                                    *buf += r#"実行委員会"#;
                                                }
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>"#;
                                                {
                                                    *buf += r#"<a href="">"#;
                                                    *buf += r#"BN5"#;
                                                    *buf += r#"</a>&nbsp;"#;
                                                    *buf += r#"文化祭出し物"#;
                                                }
                                                *buf += r#"</td>"#;
                                                *buf += r#"<td>2024/09/02</td>"#;
                                                *buf += r#"<td> </td>"#;
                                                *buf += r#"<td class="text-right">10%</td>"#;
                                            }
                                            *buf += r#"</tr>"#;
                                        }
                                        *buf += r#"</tbody>"#;
                                    }
                                    *buf += r#"</table>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col">"#;
                            {
                                *buf += r#"<a href="step02.html" title="チケットを作成">"#;
                                {
                                    *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg">"#;
                                }
                                *buf += r#"</a>"#;
                            }
                            *buf += r#"</div>"#;
                        }
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
