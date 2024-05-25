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
                    buf += 'id="check' + i + '"';
                    for (var j in members) {
                        if (members[j].uid == ret.members[i].uid) {
                            buf += ' checked';
                            break;
                        }
                    }
                    buf += '></td><td>';
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
                    buf += '<input type="hidden" id="email' + i + '" value="';
                    buf += ret.members[i].email + '">';
                    buf += '<input type="hidden" id="name' + i + '" value="';
                    buf += ret.members[i].name + '">';
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

$('#btnMemberAdd').on('click', function () {
    for (i = 0; i < 1000; i++) {
        if ($('#check' + i)) {
            if ($('#check' + i).prop('checked')) {
                var member = {
                    uid: $('#uid' + i).val(),
                    email: $('#email' + i).val(),
                    name: $('#name' + i).val(),
                };
                var idx = members.findIndex(x => x.uid == member.uid);
                if (idx < 0) {
                    members.push(member);
                } else {
                    members[idx] = member;
                }
            } else {
                var idx = members.findIndex(x => x.uid == $('#uid' + i).val());
                if (idx >= 0) {
                    members.splice(idx, 1);
                }
            }
        } else {
            break;
        }
    }
    setChargeTable();
    inChargeModal.hide();
});

function removeCharge(idx) {
    members.splice(idx, 1);
    setChargeTable();
}

function chargeSeqUp(idx) {
    var i = Number(idx) - 1;
    var j = Number(idx);
    [members[i], members[j]] = [members[j], members[i]];
    setChargeTable();
}

function chargeSeqDown(idx) {
    var i = Number(idx);
    var j = Number(idx) + 1;
    [members[i], members[j]] = [members[j], members[i]];
    setChargeTable();
}

function setChargeTable() {
    var buf = '';
    if (members.length > 0) {
        buf += '<table class="table table-hover">';
        buf += '<thead><tr>';
        buf += '<th scope="col">メールアドレス</th>';
        buf += '<th scope="col">名前</th>';
        buf += '<th scope="col"></th>';
        buf += '</tr>';
        buf += '</thead>';
        buf += '<tbody>';
        for (var i in members) {
            buf += '<tr><td>';
            buf += members[i].email;
            buf += '</td><td>';
            buf += members[i].name;
            buf += '</td><td>';
            buf += '<a href="javascript:removeCharge(' + i + ')">';
            buf += '<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除"></a>&nbsp;';
            if (i != 0) {
                buf += '<a href="javascript:chargeSeqUp(' + i + ')">';
                buf += '<img class="icon" src="/static/ionicons/arrow-up-outline.svg" title="上に移動"></a>&nbsp;';
            }
            if ((Number(i) + 1) != members.length) {
                buf += '<a href="javascript:chargeSeqDown(' + i + ')">';
                buf += '<img class="icon" src="/static/ionicons/arrow-down-outline.svg" title="下に移動"></a>';
            }
            buf += '</td></tr>';
        }
        buf += '</tbody></table>';
    }
    $("div#updateCharge").html(buf);
}

function clickDeliverables() {
    new bootstrap.Modal("#deliverablesModal").show();
}
