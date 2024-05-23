var members = [];
var inChargeModal = new bootstrap.Modal("#inChargeModal");

$(document).ready(function () {
    members = JSON.parse($("#members").val());
});

function clickAddCharge() {
    $.ajax({
        type: "POST",
        url: "/api/projectMember",
        data: {
            project_id: $("input#project_id").val()
        },
        success: function (result) {
            //console.log(result);
            var ret = JSON.parse(result);
            var buf = '';
            if (ret.result == "OK") {
                buf += '<table class="table table-hover">';
                buf += '<thead>';
                buf += '<tr>';
                buf += '<th scope="col">選択</th>';
                buf += '<th scope="col">ロール</th>';
                buf += '<th scope="col">メールアドレス</th>';
                buf += '<th scope="col">名前</th>';
                buf += '<th scope="col"></th>';
                buf += '</tr>';
                buf += '</thead>';
                buf += '<tbody>';
                for (var i in ret.members) {
                    buf += '<tr><td>';
                    buf += '<input class="form-check-input" type="checkbox" ';
                    buf += 'id="check' + i + '" checked></td><td>';
                    switch (ret.members[i].role) {
                        case 1:
                            buf += 'オーナー';
                            break;
                        case 2:
                            buf += '管理者';
                            break;
                        case 3:
                            buf += 'メンバー';
                            break;
                        case 4:
                            buf += '閲覧者';
                            break;
                    }
                    buf += '</td><td>';
                    buf += ret.members[i].email + '</td><td>';
                    buf += ret.members[i].name + '</td><td>';
                    buf += '<input type="hidden" id="uid' + i + '" value="';
                    buf += ret.members[i].uid + '">';
                    buf += '</td></tr>';
                }
                buf += '</tbody>';
                buf += '</table>';
            } else {
                buf += ret.message;
            }
            $("div#searched").html(buf);
            inChargeModal.show();
        }
    });
}

function clickDeliverables() {
    new bootstrap.Modal("#deliverablesModal").show();
}
