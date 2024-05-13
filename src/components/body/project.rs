use super::super::Component;
use super::parts::{footer::Footer, nav::Nav};
use crate::Props;

pub struct ProjectBody {
    pub nav: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl ProjectBody {
    pub fn new() -> Self {
        ProjectBody {
            nav: Box::new(Nav {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for ProjectBody {
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
                        *buf += r#"<h3 class="mb-3">プロジェクトを作成</h3>"#;
                        *buf += r#"<form action="/project/add" method="POST">"#;
                        {
                            // プロジェクト名
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="name">"#;
                                *buf += r#"プロジェクト名</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control"#;
                                    if let Some(v) = &props.project_validation {
                                        if let Some(_) = &v.project_name {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="project_name" name="project_name" type="text" maxlength="40" value=""#;
                                    if let Some(p) = &props.project {
                                        if let Some(n) = &p.project_name {
                                            *buf += n;
                                        }
                                    }
                                    *buf += r#"" required>"#;

                                    if let Some(v) = &props.project_validation {
                                        if let Some(e) = &v.project_name {
                                            *buf += r#"<div class="invalid-feedback">"#;
                                            *buf += e;
                                            *buf += r#"</div>"#;
                                        }
                                    }
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // チケットID接頭辞
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="prefix">"#;
                                *buf += r#"チケットID接頭辞</label>"#;
                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input class="form-control" id="prefix" name="prefix" type="text" maxlength="10" value=""#;
                                    if let Some(p) = &props.project {
                                        if let Some(p) = &p.prefix {
                                            *buf += p;
                                        }
                                    }
                                    *buf += r#"" required>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // メンバー
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label class="col-md-3 col-form-label bg-light mb-1" for="member">"#;
                                *buf += r#"メンバー</label>"#;
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
                                                    *buf += r#"<th scope="col">ロール</th>"#;
                                                    *buf +=
                                                        r#"<th scope="col">メールアドレス</th>"#;
                                                    *buf += r#"<th scope="col">名前</th>"#;
                                                    *buf += r#"<th scope="col"></th>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</thead>"#;

                                            *buf += r#"<tbody id="members-tbody">"#;
                                            {
                                                let mut i = 0;
                                                for member in &props.members {
                                                    *buf += r#"<tr>"#;
                                                    {
                                                        *buf += r#"<td>"#;
                                                        *buf += &member.role_to_string();
                                                        *buf += r#"</td><td>"#;
                                                        if let Some(e) = &member.email {
                                                            *buf += e;
                                                        }
                                                        *buf += r#"</td><td>"#;
                                                        if let Some(n) = &member.name {
                                                            *buf += n;
                                                        }
                                                        *buf += r#"</td><td>"#;
                                                        if i > 0 {
                                                            *buf += r#"<a href="javascript:alert('hello');">"#;
                                                            {
                                                                *buf += r#"<img class="icon" src="/static/ionicons/settings-outline.svg" title="設定">"#;
                                                            }
                                                            *buf += r#"</a>&nbsp;"#;
                                                            *buf += r#"<a href="javascript:alert('hello');">"#;
                                                            {
                                                                *buf += r#"<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">"#;
                                                            }
                                                            *buf += r#"</a>"#;
                                                        }
                                                        *buf += r#"</td>"#;
                                                    }
                                                    *buf += r#"</tr>"#;
                                                    i += 1;
                                                }
                                            }
                                            *buf += r#"</tbody>"#;
                                        }
                                        *buf += r#"</table>"#;
                                        *buf += r#"<a href="javascript:clickAddMember();">"#;
                                        {
                                            *buf += r#"<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="メンバーを追加">"#;
                                        }
                                        *buf += r#"</a>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                                *buf +=
                                    r#"<input type="hidden" id="members" name="members" value=""#;
                                if let Ok(r) = serde_json::to_string(&props.members) {
                                    super::super::escape_html(&r, buf);
                                }
                                *buf += r#"">"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                            {
                                *buf += r#"<div class="col-md-9">"#;
                                {
                                    *buf += r#"<button class="btn btn-primary" type="submit">"#;
                                    {
                                        *buf += r#"<img class="icon" src="/static/ionicons/create-outline.svg">&nbsp;作成"#;
                                    }
                                    *buf += r#"</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;

                        *buf += r#"<div class="modal fade" id="addMemberModal" tabindex="-1" aria-labelledby="addMemberModalLabel" "#;
                        *buf += r#"aria-hidden="true">"#;
                        {
                            *buf += r#"<div class="modal-dialog modal-lg">"#;
                            {
                                *buf += r#"<div class="modal-content">"#;
                                {
                                    *buf += r#"<div class="modal-header">"#;
                                    {
                                        *buf += r#"<h1 class="modal-title fs-5" id="addMemberModalLabel">メンバーを検索</h1>"#;
                                        *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="modal-body">"#;
                                    {
                                        *buf += r#"<div class="row">"#;
                                        {
                                            *buf += r#"<label class="col-md-3 col-form-label mb-1" for="email">メールアドレス</label>"#;
                                            *buf += r#"<div class="col-md-6 mb-1">"#;
                                            {
                                                *buf += r#"<input class="form-control" id="email" type="text" maxlength="50">"#;
                                            }
                                            *buf += r#"</div>"#;
                                            *buf += r#"<div class="col-md-3 mb-1">"#;
                                            {
                                                *buf += r#"<button class="btn btn-info" id="search-email" type="button">検索</button>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="row">"#;
                                        {
                                            *buf += r#"<label class="col-md-3 col-form-label mb-1" for="member-name">名前</label>"#;
                                            *buf += r#"<div class="col-md-6 mb-1">"#;
                                            {
                                                *buf += r#"<input class="form-control" id="member-name" type="text" maxlength="50">"#;
                                            }
                                            *buf += r#"</div>"#;
                                            *buf += r#"<div class="col-md-3 mb-1">"#;
                                            {
                                                *buf += r#"<button class="btn btn-info" type="button">検索</button>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="row py-3">"#;
                                        {
                                            *buf += r#"<div class="col">"#;
                                            {
                                                *buf += r#"<div id="searched"></div>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="modal-footer">"#;
                                    {
                                        *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                                        *buf += r#"<button class="btn btn-primary" id="btnAddMember" type="button" disabled>メンバーに追加</button>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                                *buf += r#"<input type="hidden" id="add_members" name="add_members" value="">"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="modal fade" id="updateMemberModal" tabindex="-1" aria-labelledby="updateMemberModalLabel" "#;
                        *buf += r#"aria-hidden="true">"#;
                        {
                            *buf += r#"<div class="modal-dialog modal-lg">"#;
                            {
                                *buf += r#"<div class="modal-content">"#;
                                {
                                    *buf += r#"<div class="modal-header">"#;
                                    {
                                        *buf += r#"<h1 class="modal-title fs-5" id="addMemberModalLabel">メンバーを更新</h1>"#;
                                        *buf += r#"<button class="btn-close" type="button" data-bs-dismiss="modal" aria-label="キャンセル"></button>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="modal-body">"#;
                                    {
                                        *buf += r#"<div class="row py-3">"#;
                                        {
                                            *buf += r#"<div class="col">"#;
                                            {
                                                *buf += r#"<div id="updateMember"></div>"#;
                                            }
                                            *buf += r#"</div>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="modal-footer">"#;
                                    {
                                        *buf += r#"<button class="btn btn-secondary" type="button" data-bs-dismiss="modal">キャンセル</button>"#;
                                        *buf += r#"<button class="btn btn-primary" id="btnUpdateMember" type="button">メンバーを更新</button>"#;
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
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
            *buf += r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
            *buf += r#"<script src="/static/js/project0012.js"></script>"#;
        }
        *buf += r#"</body>"#;
    }
}
