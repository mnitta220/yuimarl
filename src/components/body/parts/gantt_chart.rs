use crate::{components::Component, Props};

pub struct GanttChart {
    pub can_update: bool,
}

impl Component for GanttChart {
    fn write(&self, props: &Props, buf: &mut String) {
        if let Some(p) = &props.project {
            *buf += r#"<form action="/project_note" method="POST">"#;
            {
                // ガントチャート
                *buf += r#"<div class="row py-2">"#;
                {
                    *buf += r#"<div class="col flexbox" id="flexbox">"#;
                    {
                        *buf += r#"<div class="ganttbase" id="ganttbase">"#;
                        {
                            *buf += r#"<div class="ganttframe" id="ganttframe"></div>"#;
                            *buf += r#"<canvas class="scrollH" id="scrollh"></canvas>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="scrollV">"#;
                        {
                            *buf += r#"<div class="sv1"></div>"#;
                            *buf += r#"<canvas class="sv2" id="scrollv"></canvas>"#;
                            *buf += r#"<div class="sv3"></div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;

                if self.can_update {
                    *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                    {
                        *buf += r#"<div class="col">"#;
                        {
                            *buf += r#"<button class="btn btn-primary" type="submit">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/save-outline.svg">&nbsp;更新"#;
                            }
                            *buf += r#"</button>&nbsp;&nbsp;"#;

                            *buf += r#"<a class="btn btn-primary" href="/project?id="#;
                            *buf += &p.id;
                            *buf += r#"&tab=map" role="button">"#;
                            {
                                *buf += r#"<img class="icon" src="/static/ionicons/refresh-outline.svg">&nbsp;再読み込み"#;
                            }
                            *buf += r#"</a>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"<input type="hidden" id="startdate" value="2024-04-08">"#;
                *buf += r#"<input type="hidden" id="enddate" value="2024-08-31">"#;
                *buf += r#"<input type="hidden" id="holidays" value="2024-07-15,2024-08-12">"#;
                *buf += r#"<input type="hidden" id="tickets""#;
                *buf += r#"value="[{&quot;id&quot;:&quot;YU1&quot;,&quot;idDisp&quot;:&quot;YU1&quot;,&quot;name&quot;:&quot;Yuimarl開発&quot;,&quot;start&quot;:&quot;2024-04-07T15:00:00.000Z&quot;,&quot;end&quot;:null,&quot;progress&quot;:0,&quot;open&quot;:true,&quot;children&quot;:[{&quot;id&quot;:&quot;YU2&quot;,&quot;idDisp&quot;:&quot;YU2&quot;,&quot;name&quot;:&quot;技術調査&quot;,&quot;start&quot;:&quot;2024-04-07T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-04-16T15:00:00.000Z&quot;,&quot;progress&quot;:100,&quot;open&quot;:false,&quot;children&quot;:[{&quot;id&quot;:&quot;YU3&quot;,&quot;idDisp&quot;:&quot;YU3&quot;,&quot;name&quot;:&quot;Firestore&quot;,&quot;start&quot;:&quot;2024-04-07T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-06-12T15:00:00.000Z&quot;,&quot;progress&quot;:100,&quot;open&quot;:true,&quot;children&quot;:[]}]},{&quot;id&quot;:&quot;YU8&quot;,&quot;idDisp&quot;:&quot;YU8&quot;,&quot;name&quot;:&quot;version 1.0.0&quot;,&quot;start&quot;:&quot;2024-04-17T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-06-12T15:00:00.000Z&quot;,&quot;progress&quot;:100,&quot;open&quot;:false,&quot;children&quot;:[{&quot;id&quot;:&quot;YU8&quot;,&quot;idDisp&quot;:&quot;YU8&quot;,&quot;name&quot;:&quot;version 1.0.0&quot;,&quot;start&quot;:&quot;2024-04-17T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-06-12T15:00:00.000Z&quot;,&quot;progress&quot;:100,&quot;open&quot;:true,&quot;children&quot;:[]}]},{&quot;id&quot;:&quot;YU62&quot;,&quot;idDisp&quot;:&quot;YU62&quot;,&quot;name&quot;:&quot;version 1.0.12&quot;,&quot;start&quot;:&quot;2024-06-20T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-06-22T15:00:00.000Z&quot;,&quot;progress&quot;:100,&quot;open&quot;:false,&quot;children&quot;:[{&quot;id&quot;:&quot;YU8&quot;,&quot;idDisp&quot;:&quot;YU8&quot;,&quot;name&quot;:&quot;version 1.0.0&quot;,&quot;start&quot;:&quot;2024-04-17T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-06-12T15:00:00.000Z&quot;,&quot;progress&quot;:100,&quot;open&quot;:true,&quot;children&quot;:[]}]},{&quot;id&quot;:&quot;YU71&quot;,&quot;idDisp&quot;:&quot;YU71&quot;,&quot;name&quot;:&quot;version 1.0.13&quot;,&quot;start&quot;:&quot;2024-06-23T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-07-29T15:00:00.000Z&quot;,&quot;progress&quot;:10,&quot;open&quot;:true,&quot;children&quot;:[{&quot;id&quot;:&quot;YU60&quot;,&quot;idDisp&quot;:&quot;YU60&quot;,&quot;name&quot;:&quot;ガントチャート&quot;,&quot;start&quot;:&quot;2024-06-23T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-07-29T15:00:00.000Z&quot;,&quot;progress&quot;:10,&quot;open&quot;:true,&quot;children&quot;:[{&quot;id&quot;:&quot;YU72&quot;,&quot;idDisp&quot;:&quot;YU72&quot;,&quot;name&quot;:&quot;画面プロトタイプ作成&quot;,&quot;start&quot;:&quot;2024-06-23T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-07-11T15:00:00.000Z&quot;,&quot;progress&quot;:80,&quot;open&quot;:true,&quot;children&quot;:[]},{&quot;id&quot;:&quot;YU73&quot;,&quot;idDisp&quot;:&quot;YU73&quot;,&quot;name&quot;:&quot;実装・テスト&quot;,&quot;start&quot;:&quot;2024-07-15T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-07-25T15:00:00.000Z&quot;,&quot;progress&quot;:0,&quot;open&quot;:true,&quot;children&quot;:[]},{&quot;id&quot;:&quot;YU74&quot;,&quot;idDisp&quot;:&quot;YU74&quot;,&quot;name&quot;:&quot;ユーザーガイド更新&quot;,&quot;start&quot;:&quot;2024-07-28T15:00:00.000Z&quot;,&quot;end&quot;:&quot;2024-07-29T15:00:00.000Z&quot;,&quot;progress&quot;:0,&quot;open&quot;:true,&quot;children&quot;:[]}]}]},{&quot;id&quot;:&quot;YU4&quot;,&quot;idDisp&quot;:&quot;YU4&quot;,&quot;name&quot;:&quot;バックログ&quot;,&quot;start&quot;:null,&quot;end&quot;:null,&quot;progress&quot;:0,&quot;open&quot;:true,&quot;children&quot;:[{&quot;id&quot;:&quot;YU46&quot;,&quot;idDisp&quot;:&quot;YU46&quot;,&quot;name&quot;:&quot;ページング改善&quot;,&quot;start&quot;:null,&quot;end&quot;:null,&quot;progress&quot;:0,&quot;open&quot;:true,&quot;children&quot;:[]},{&quot;id&quot;:&quot;YU45&quot;,&quot;idDisp&quot;:&quot;YU45&quot;,&quot;name&quot;:&quot;オーナー変更&quot;,&quot;start&quot;:null,&quot;end&quot;:null,&quot;progress&quot;:0,&quot;open&quot;:true,&quot;children&quot;:[]}]}]}]">"#;
            }
            *buf += r#"</form>"#;

            // チケットダイアログ
            *buf += r#"<div class="modal fade" id="ticketModal" tabindex="-1" aria-labelledby="ticketModalLabel" aria-hidden="true">"#;
            {
                *buf += r#"<div class="modal-dialog modal-xl">"#;
                {
                    *buf += r#"<div class="modal-content">"#;
                    {
                        *buf += r#"<div class="modal-header">"#;
                        {
                            *buf += r#"<h1 class="modal-title fs-5" id="ticketModalLabel">チケット</h1>"#;
                            *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="modal-body">"#;
                        {
                            // プロジェクト / チケットID
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="ticket-id">プロジェクト / チケットID</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">文化祭実行プロジェクト / BN8</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // チケット名
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="name">チケット名</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control" id="name" type="text" maxlength="40" value="たこやき模擬店" required>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 内容
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="message">内容</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<textarea class="form-control" id="message" rows="3" name="message">"#;
                                    *buf += r#"たこやき模擬店を出店するために、やるべきことを検討する。"#;
                                    *buf += r#"</textarea>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 担当者
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="member">担当者</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-floating">"#;
                                    {
                                        *buf += r#"<table class="table table-hover">"#;
                                        {
                                            *buf += r#"<thead>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf +=
                                                        r#"<th scope="col">メールアドレス</th>"#;
                                                    *buf += r#"<th scope="col">名前</th>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</thead>"#;
                                            *buf += r#"<tbody>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>taro.yamada@mail.com</td>"#;
                                                    *buf += r#"<td>山田太郎</td>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>masami.iwaki@mail.com</td>"#;
                                                    *buf += r#"<td>岩鬼正美</td>"#;
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

                            // 開始日 / 終了日
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<legend class="col-form-label col-md-3 bg-light mb-1">開始日 / 終了日</legend>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="row">"#;
                                    {
                                        *buf += r#"<div class="col-sm-6 mb-2">"#;
                                        {
                                            *buf += r#"<label class="form-label" for="startdate">開始日</label>"#;
                                            *buf += r#"<input class="form-control" id="startdate" type="date" value="2024-09-10">"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="col-sm-6 mb-1">"#;
                                        {
                                            *buf += r#"<label class="form-label" for="enddate">終了日</label>"#;
                                            *buf += r#"<input class="form-control" id="enddate" type="date">"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 進捗率
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<legend class="col-form-label col-md-3 bg-light mb-1">優先度</legend>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority1" name="priority" type="radio" value="priority1">"#;
                                        *buf += r#"<label class="form-check-label" for="priority1">最優先</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority2" name="priority" type="radio" value="priority2" checked="checked">"#;
                                        *buf += r#"<label class="form-check-label" for="priority2">高</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority3" name="priority" type="radio" value="priority3">"#;
                                        *buf += r#"<label class="form-check-label" for="priority3">中</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority4" name="priority" type="radio" value="priority4">"#;
                                        *buf += r#"<label class="form-check-label" for="priority4">低</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority0" name="priority" type="radio" value="priority0">"#;
                                        *buf += r#"<label class="form-check-label" for="priority0">未設定</label>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 親チケット
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">親チケット</label>"#;
                                *buf += r#"<div class="col-md-9">"#;
                                {
                                    *buf += r#"BN5"#;
                                    *buf += r#"&nbsp;:&nbsp;"#;
                                    *buf += r#"文化祭出し物"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 子チケット
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">子チケット</label>"#;
                                *buf += r#"<div class="col-md-9 pb-md-2">"#;
                                {
                                    *buf += r#"<p class="mb-0">"#;
                                    {
                                        *buf += r#"BN13"#;
                                        *buf += r#"&nbsp;:&nbsp;"#;
                                        *buf += r#"備品準備"#;
                                    }
                                    *buf += r#"</p>"#;
                                    *buf += r#"<p class="mb-0">"#;
                                    {
                                        *buf += r#"BN14"#;
                                        *buf += r#"&nbsp;:&nbsp;"#;
                                        *buf += r#"食材購入"#;
                                    }
                                    *buf += r#"</p>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="modal-footer">"#;
                        {
                            *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                            *buf += r#"<button class="btn btn-primary" type="button">"#;
                            {
                                *buf +=
                                    r#"<img class="icon" src="/static/ionicons/exit-outline.svg">"#;
                                *buf += r#"&nbsp;チケット画面に遷移"#;
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
