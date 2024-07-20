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
                *buf += r#"<div class="row p-2">"#;
                {
                    *buf += r#"<div class="col flexbox2" id="flexbox">"#;
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

                // 完了済みを表示
                *buf += r#"<div class="row pt-2">"#;
                {
                    *buf += r#"<div class="col">"#;
                    {
                        *buf += r#"<div class="form-check form-switch">"#;
                        {
                            *buf += r#"<input class="form-check-input" id="showdone" type="checkbox" role="switch">"#;
                            *buf += r#"<label class="form-check-label" for="showdone">完了済みを表示</label>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;

                // 進捗遅れを赤く表示
                *buf += r#"<div class="row py-2">"#;
                {
                    *buf += r#"<div class="col">"#;
                    {
                        *buf += r#"<div class="form-check form-switch">"#;
                        {
                            *buf += r#"<input class="form-check-input" id="delayred" type="checkbox" role="switch">"#;
                            *buf += r#"<label class="form-check-label" for="delayred">進捗遅れを赤く表示</label>"#;
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

                *buf += r#"<input type="hidden" id="projectId" value=""#;
                if let Some(p) = &props.project {
                    *buf += &p.id;
                }
                *buf += r#"">"#;
                *buf += r#"<input type="hidden" id="startdate" value=""#;
                if let Some(s) = &props.gantt_start {
                    *buf += &s;
                }
                *buf += r#"">"#;
                *buf += r#"<input type="hidden" id="enddate" value=""#;
                if let Some(e) = &props.gantt_end {
                    *buf += &e;
                }
                *buf += r#"">"#;

                // 日本の祝日
                *buf += r#"<input type="hidden" id="holidays" value=""#;
                if let Some(p) = &props.project {
                    if let Some(h) = p.holiday_jp {
                        if h {
                            *buf += &crate::model::horiday::get_holiday(
                                &props.gantt_start,
                                &props.gantt_end,
                            );
                        }
                    }
                }
                *buf += r#"">"#;

                *buf += r#"<input type="hidden" id="tickets" value=""#;
                if let Ok(g) = serde_json::to_string(&props.gantt_tickets) {
                    super::super::super::escape_html(&g, buf)
                }
                *buf += r#"">"#;
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
                            // チケットID
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="ticket-id">チケットID</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1" id="modal_ticket_id"></div>"#;
                            }
                            *buf += r#"</div>"#;

                            // チケット名
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="ticket_name">チケット名</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control" id="ticket_name" type="text" value="">"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 内容
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="description">内容</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<textarea class="form-control" id="description" rows="3" name="description">"#;
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
                                    *buf += r#"<div class="form-floating" id="members"></div>"#;
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
                                        *buf +=
                                            r#"<div class="col-sm-6 mb-2" id="start_date"></div>"#;
                                        *buf +=
                                            r#"<div class="col-sm-6 mb-2" id="end_date"></div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 進捗率
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<legend class="col-form-label col-md-3 bg-light mb-1">進捗率</legend>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="row">"#;
                                    {
                                        *buf += r#"<div class="col-sm-6 mb-2">"#;
                                        {
                                            *buf += r#"<table>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>"#;
                                                    {
                                                        *buf += r#"<input class="form-control" id="progress" name="progress" type="number" min="0" max="100" value="">"#;
                                                    }
                                                    *buf += r#"</td>"#;
                                                    *buf += r#"<td>&nbsp;%</td>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</table>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="col-sm-6 mb-1">"#;
                                        {
                                            *buf += r#"<div class="form-check">"#;
                                            {
                                                *buf += r#"<input class="form-check-input" id="finished" type="checkbox">"#;
                                                *buf += r#"<label class="form-check-label" for="finished">完了済</label>"#;
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

                            // 優先度
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<legend class="col-form-label col-md-3 bg-light mb-1">優先度</legend>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority1" name="priority" type="radio" value="4">"#;
                                        *buf += r#"<label class="form-check-label" for="priority1">最優先</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority2" name="priority" type="radio" value="3">"#;
                                        *buf += r#"<label class="form-check-label" for="priority2">高</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority3" name="priority" type="radio" value="2">"#;
                                        *buf += r#"<label class="form-check-label" for="priority3">中</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority4" name="priority" type="radio" value="1">"#;
                                        *buf += r#"<label class="form-check-label" for="priority4">低</label>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="form-check form-check-inline">"#;
                                    {
                                        *buf += r#"<input class="form-check-input" id="priority0" name="priority" type="radio" value="0">"#;
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
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="parent">親チケット</label>"#;
                                *buf += r#"<div class="col-md-9" id="parent"></div>"#;
                            }
                            *buf += r#"</div>"#;

                            // 子チケット
                            *buf += r#"<div class="row pt-1 pb-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="category">子チケット</label>"#;
                                *buf += r#"<div class="col-md-9 pb-md-2" id="children"></div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="modal-footer">"#;
                        {
                            *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                            *buf += r#"<button class="btn btn-primary" id="btnRedirectTicket" type="button">"#;
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
