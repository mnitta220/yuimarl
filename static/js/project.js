var members = [];

$(document).ready(function () {
    members = JSON.parse($("#members").val());
});

function clickAddMember() {
    new bootstrap.Modal("#addMemberModal").show();
}

function roleToString(role) {
    switch (role) {
        case 1:
            return 'オーナー';
        case 2:
            return '管理者';
        case 3:
            return 'メンバー';
        case 4:
            return '閲覧者';
    }
    return 'Unknown';
}

$('#search-email').on('click', function () {
    $("#add_members").val("");
    $.ajax({
        type: "POST",
        url: "/api/userByEmail",
        data: {
            email: $("input#email").val()
        },
        success: function (result) {
            console.log('***2 result=' + result);
            var ret = JSON.parse(result);
            var buf = '';
            if (ret.result == "OK") {
                if (ret.users.length > 0) {
                    buf += '<table class="table table-hover">';
                    buf += '<thead>';
                    buf += '<tr>';
                    buf += '<th scope="col">選択</th>';
                    buf += '<th scope="col">メールアドレス</th>';
                    buf += '<th scope="col">名前</th>';
                    buf += '<th scope="col">ロール</th>';
                    buf += '</tr>';
                    buf += '</thead>';
                    buf += '<tbody>';
                    for (var i in ret.users) {
                        buf += '<tr><td>';
                        buf += '<input class="form-check-input" type="checkbox" ';
                        buf += 'id="check' + i + '" checked></td><td>';
                        buf += ret.users[i].email + '</td><td>';
                        buf += ret.users[i].name + '</td><td>';
                        buf += '<select class="form-select" id="role' + i;
                        buf += '" name="role' + i + '">';
                        buf += '<option value="2">管理者</option>';
                        buf += '<option value="3">メンバー</option>';
                        buf += '<option value="4">閲覧者</option>';
                        buf += '</select>';
                        buf += '<input type="hidden" id="uid' + i + '" value="';
                        buf += ret.users[i].uid + '">';
                        buf += '<input type="hidden" id="name' + i + '" value="';
                        buf += ret.users[i].name + '">';
                        buf += '<input type="hidden" id="email' + i + '" value="';
                        buf += ret.users[i].email + '">';
                        buf += '</td></tr>';
                    }
                    buf += '</tbody>';
                    buf += '</table>';
                    $("#btnAddMember").removeAttr('disabled');
                } else {
                    buf += '<div class="col"><p class="text-danger">該当するユーザーが登録されていません。</p></div>';
                    $("#btnAddMember").attr({ 'disabled': 'disabled' });
                }
            } else {
                buf += ret.message;
            }
            $("div#searched").html(buf);
        }
    });
});

$('#btnAddMember').on('click', function () {
    for (i = 0; i < 10; i++) {
        if ($('#check' + i)) {
            if ($('#check' + i).prop('checked')) {
                var member = {
                    uid: $('#uid' + i).val(),
                    email: $('#email' + i).val(),
                    name: $('#name' + i).val(),
                    role: Number($('#role' + i).children('option:selected').val()),
                };
                var idx = members.findIndex(x => x.uid == member.uid);
                if (idx == -1) {
                    members.push(member);
                }
            }
        } else {
            break;
        }
    }

    $("#email").val("");
    $("#member-name").val("");
    $("#members").val('{"members":' + JSON.stringify(members) + '}');
    $("div#searched").html("");
    new bootstrap.Modal("#addMemberModal").hide();
    var buf = '';
    for (var i in members) {
        buf += '<tr><td>';
        buf += roleToString(members[i].role);
        buf += '</td><td>';
        buf += members[i].email;
        buf += '</td><td>';
        buf += members[i].name;
        buf += '</td><td></td></tr>';
    }

    $("#members-tbody").html(buf);
});
