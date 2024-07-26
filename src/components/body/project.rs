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
                                *buf += r#"<label for="name" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"プロジェクト名"#;
                                *buf += r#"</label>"#;

                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input type="text" class="form-control"#;
                                    if let Some(v) = &props.project_validation {
                                        if let Some(_) = &v.project_name {
                                            *buf += r#" is-invalid"#;
                                        }
                                    }
                                    *buf += r#"" id="project_name" name="project_name" maxlength="40" value=""#;
                                    if let Some(p) = &props.project {
                                        *buf += &p.project_name;
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

                            // オーナー
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="name" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"オーナー"#;
                                *buf += r#"</label>"#;

                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<div class="form-floating">"#;
                                    {
                                        *buf += r#"<span>新田雅浩 [masa.nitta@nifty.ne.jp]</span>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // メンバー
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="email" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"メンバー"#;
                                *buf += r#"</label>"#;

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
                                                    *buf += r#"<th scope="col">名前</th>"#;
                                                    *buf +=
                                                        r#"<th scope="col">メールアドレス</th>"#;
                                                    *buf += r#"<th scope="col">ロール</th>"#;
                                                    *buf += r#"<th scope="col"></th>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</thead>"#;
                                            *buf += r#"<tbody>"#;
                                            {
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>新田雅浩</td>"#;
                                                    *buf += r#"<td>masa.nitta@nifty.ne.jp</td>"#;
                                                    *buf += r#"<td>オーナー</td>"#;
                                                    *buf += r#"<td>"#;
                                                    {
                                                        *buf += r#"<a href="" class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"更新</a>&nbsp;<a href="" "#;
                                                        *buf += r#"class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"削除</a>"#;
                                                    }
                                                    *buf += r#"</td>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>新田雅浩</td>"#;
                                                    *buf += r#"<td>masa.nitta@nifty.ne.jp</td>"#;
                                                    *buf += r#"<td>オーナー</td>"#;
                                                    *buf += r#"<td>"#;
                                                    {
                                                        *buf += r#"<a href="" class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"更新</a>&nbsp;<a href="" "#;
                                                        *buf += r#"class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"削除</a>"#;
                                                    }
                                                    *buf += r#"</td>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                                *buf += r#"<tr>"#;
                                                {
                                                    *buf += r#"<td>新田雅浩</td>"#;
                                                    *buf += r#"<td>masa.nitta@nifty.ne.jp</td>"#;
                                                    *buf += r#"<td>オーナー</td>"#;
                                                    *buf += r#"<td>"#;
                                                    {
                                                        *buf += r#"<a href="" class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"更新</a>&nbsp;<a href="" "#;
                                                        *buf += r#"class="badge bg-info text-dark link-underline link-underline-opacity-0">"#;
                                                        *buf += r#"削除</a>"#;
                                                    }
                                                    *buf += r#"</td>"#;
                                                }
                                                *buf += r#"</tr>"#;
                                            }
                                            *buf += r#"</tbody>"#;
                                        }
                                        *buf += r#"</table>"#;

                                        *buf += r#"<button type="button" class="btn btn-primary btn-sm" data-bs-toggle="modal" "#;
                                        *buf += r##"title="メンバーを追加する" data-bs-target="#memberAddModal">"##;
                                        *buf += r#"＋"#;
                                        *buf += r#"</button>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            // チケットID接頭辞
                            *buf += r#"<div class="row py-2">"#;
                            {
                                *buf += r#"<label for="name" class="col-md-3 col-form-label bg-light mb-1">"#;
                                *buf += r#"チケットID接頭辞"#;
                                *buf += r#"</label>"#;

                                *buf += r#"<div class="col-md-9 mb-1">"#;
                                {
                                    *buf += r#"<input type="text" class="form-control" id="prefix" name="prefix" maxlength="8" value=""#;
                                    if let Some(p) = &props.project {
                                        *buf += &p.prefix;
                                    }
                                    *buf += r#"">"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="row py-3 mt-2 bg-light">"#;
                            {
                                *buf += r#"<div class="col-md-9">"#;
                                {
                                    *buf += r#"<button type="submit" class="btn btn-primary">作成する</button>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;

                        *buf += r#"<form id="membarAddForm" action="/member/add" method="POST">"#;
                        {
                            *buf += r#"<div class="modal fade" id="memberAddModal" tabindex="-1" aria-labelledby="memberAddModalLabel" "#;
                            *buf += r#"aria-hidden="true">"#;
                            {
                                *buf += r#"<div class="modal-dialog">"#;
                                {
                                    *buf += r#"<div class="modal-content">"#;
                                    {
                                        *buf += r#"<div class="modal-header">"#;
                                        {
                                            *buf += r#"<h1 class="modal-title fs-5" id="exampleModalLabel">メンバーを検索</h1>"#;
                                            *buf += r#"<button type="button" class="btn-close" data-bs-dismiss="modal" "#;
                                            *buf += r#"aria-label="キャンセル"></button>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="modal-body">"#;
                                        {
                                            *buf += r#"<div class="row">"#;
                                            {
                                                *buf += r#"<label for="email" class="col-md-3 col-form-label mb-1">"#;
                                                {
                                                    *buf += r#"メールアドレス"#;
                                                }
                                                *buf += r#"</label>"#;

                                                *buf += r#"<div class="col-md-6 mb-1">"#;
                                                {
                                                    *buf += r#"<input type="text" class="form-control" id="email" name="email" maxlength="50">"#;
                                                }
                                                *buf += r#"</div>"#;

                                                *buf += r#"<div class="col-md-3 mb-1">"#;
                                                {
                                                    *buf += r#"<button type="button" class="btn btn-info" id="search-email">検索</button>"#;
                                                }
                                                *buf += r#"</div>"#;
                                            }
                                            *buf += r#"</div>"#;

                                            *buf += r#"<div class="row">"#;
                                            {
                                                *buf += r#"<label for="member-name" class="col-md-3 col-form-label mb-1">"#;
                                                {
                                                    *buf += r#"名前"#;
                                                }
                                                *buf += r#"</label>"#;

                                                *buf += r#"<div class="col-md-6 mb-1">"#;
                                                {
                                                    *buf += r#"<input type="text" class="form-control" id="member-name" "#;
                                                    *buf += r#"maxlength="50">"#;
                                                }
                                                *buf += r#"</div>"#;

                                                *buf += r#"<div class="col-md-3 mb-1">"#;
                                                {
                                                    *buf += r#"<button type="button" class="btn btn-info">検索</button>"#;
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
                                            *buf += r#"<button type="button" class="btn btn-secondary" "#;
                                            *buf +=
                                                r#"data-bs-dismiss="modal">キャンセル</button> "#;
                                            *buf += r#"<button id="btnAddMember" type="button" class="btn btn-primary" disabled>メンバーに追加</button>"#;
                                            //*buf += r#"<button id="btnAddMember" type="submit" class="btn btn-primary" disabled>メンバーに追加</button>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                    *buf += r#"<input type="hidden" id="add_members" name="add_members" value="">"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</form>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
            *buf += r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
            *buf += r##"<script>
                $('#search-email').on('click', function () {
                    $("#add_members").val("");
                    $.ajax({
                        type: "POST",
                        url: "/api/userByEmail",
                        data: {
                          email: $("input#email").val()
                        },
                        success: function(result) {
                            var ret = JSON.parse(result);
                            var buf = '';
                            if (ret.result == "OK") {
                                if (ret.users.length > 0) {
                                    buf += '<table class="table table-hover">';
                                    buf += '<thead>';
                                    buf += '<tr>';
                                    buf += '<th scope="col">選択</th>';
                                    buf += '<th scope="col">名前</th>';
                                    buf += '<th scope="col">メールアドレス</th>';
                                    buf += '<th scope="col">ロール</th>';
                                    buf += '</tr>';
                                    buf += '</thead>';
                                    buf += '<tbody>';
                                    for (var i in ret.users) {
                                        buf += '<tr><td>';
                                        buf += '<input class="form-check-input" type="checkbox" ';
                                        buf += 'id="check' + i + '" checked></td><td>';
                                        buf += ret.users[i].name + '</td><td>';
                                        buf += ret.users[i].email + '</td><td>';
                                        buf += '<select class="form-select" id="role' + i;
                                        buf += '" name="role' + i + '">';
                                        buf += '<option value="2">管理者</option>';
                                        buf += '<option value="3">メンバー</option>';
                                        buf += '<option value="4">閲覧者</option>';
                                        buf += '</select>';
                                        buf += '<input type="hidden" id="uid' + i + '" value="';
                                        buf += ret.users[i].uid + '">';
                                        buf += '</td></tr>';
                                    }
                                    buf += '</tbody>';
                                    buf += '</table>';
                                    $("#btnAddMember").removeAttr('disabled');
                                } else {
                                    buf += '<div class="col"><p class="text-danger">該当するユーザーが登録されていません。</p></div>';
                                    $("#btnAddMember").attr({'disabled': 'disabled'});
                                }
                            } else {
                                buf += ret.message;
                            }
                            $("div#searched").html(buf);
                        }
                    });
                });

                $('#btnAddMember').on('click', function () {
                    console.log('***' + $('#role0').children('option:selected').val());
                    console.log('***' + $('#check0').prop('checked'));
                    var buf = '';
                    for (i = 0; i < 10; i++) {
                        if ($('#check' + i)) {
                            if ($('#check' + i).prop('checked')) {
                                if (buf.length > 0) {
                                    buf += ',';
                                }
                                buf += $('#uid' + i).val() + ',';
                                buf += $('#role0').children('option:selected').val();
                            }
                        } else {
                            break;
                        }
                    }
                    console.log('***buf=' + buf);
                    if (buf.length > 0) {
                        $('#add_members').val(buf);
                        $('#membarAddForm').submit();
                    }
                });
            </script>"##;
        }
        *buf += r#"</body>"#;
    }
}
