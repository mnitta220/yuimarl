var members = [];
var inChargeModal = new bootstrap.Modal("#inChargeModal");
var addParentModal = new bootstrap.Modal("#addParentModal");

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

$('#btnSave').on('click', function () {
    $("#members").val(JSON.stringify(members));
    $('#post_ticket').submit();
});

function removeCharge(idx) {
    members.splice(idx, 1);
    setChargeTable();
}

function chargeSeqUp(idx) {
    var j = Number(idx);
    var i = j - 1;
    [members[i], members[j]] = [members[j], members[i]];
    setChargeTable();
}

function chargeSeqDown(idx) {
    var i = Number(idx);
    var j = i + 1;
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
        buf += '</tr></thead>';
        buf += '<tbody>';
        for (var i in members) {
            buf += '<tr><td>';
            buf += members[i].email;
            buf += '</td><td>';
            buf += members[i].name;
            buf += '</td><td>';
            buf += '<a href="javascript:removeCharge(' + i + ')">';
            buf += '<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除"></a>';
            if (i != 0) {
                buf += '&nbsp;<a href="javascript:chargeSeqUp(' + i + ')">';
                buf += '<img class="icon" src="/static/ionicons/arrow-up-outline.svg" title="上に移動"></a>';
            }
            if ((Number(i) + 1) != members.length) {
                buf += '&nbsp;<a href="javascript:chargeSeqDown(' + i + ')">';
                buf += '<img class="icon" src="/static/ionicons/arrow-down-outline.svg" title="下に移動"></a>';
            }
            buf += '</td></tr>';
        }
        buf += '</tbody></table>';
    }
    $("div#chargeTbl").html(buf);
}

function clickAddParent() {
    addParentModal.show();
}

$('#search-parent').on('click', function () {
    if (`${$("input#search_id").val()}`.trim() == "") {
        var buf = '<p class="text-danger">チケットID を入力してください</p>';
        buf += '<input type="hidden" id="searchedParentId" value="">';
        buf += '<input type="hidden" id="searchedParentIdDisp" value="">';
        buf += '<input type="hidden" id="searchedParentName" value="">';
        $("#btnAddParent").attr({ 'disabled': 'disabled' });
        $("div#searchedParent").html(buf);
        return;
    }

    $("#add_members").val("");
    $.ajax({
        type: "POST",
        url: "/api/ticketByIdDisp",
        data: {
            project_id: $("input#project_id").val(),
            id_disp: $("input#search_id").val(),
        },
        success: function (result) {
            var ret = JSON.parse(result);
            var buf = '';
            if (ret.result) {
                if (ret.ticket.id == $("#ticket_id").val()) {
                    buf += '<p class="text-danger">自身を親チケットにすることはできません</p>';
                    buf += '<input type="hidden" id="searchedParentId" value="">';
                    buf += '<input type="hidden" id="searchedParentIdDisp" value="">';
                    buf += '<input type="hidden" id="searchedParentName" value="">';
                    $("#btnAddParent").attr({ 'disabled': 'disabled' });
                    $("div#searchedParent").html(buf);
                    return;
                }

                buf += '<p><b>' + ret.ticket.id_disp + '</b> : ' + ret.ticket.name + '</p>';
                buf += '<input type="hidden" id="searchedParentId" value="';
                buf += ret.ticket.id + '">';
                buf += '<input type="hidden" id="searchedParentIdDisp" value="';
                buf += ret.ticket.id_disp + '">';
                buf += '<input type="hidden" id="searchedParentName" value="';
                buf += ret.ticket.name + '">';
                $("#btnAddParent").removeAttr('disabled');
            } else {
                buf += '<p class="text-danger">';
                buf += ret.message;
                buf += '</p>';
                buf += '<input type="hidden" id="searchedParentId" value="">';
                buf += '<input type="hidden" id="searchedParentIdDisp" value="">';
                buf += '<input type="hidden" id="searchedParentName" value="">';
                $("#btnAddParent").attr({ 'disabled': 'disabled' });
            }
            $("div#searchedParent").html(buf);
        }
    });
});

$('#btnAddParent').on('click', function () {
    var buf = '<a href="">';
    buf += $("input#searchedParentIdDisp").val();
    buf += '</a>&nbsp;:&nbsp;';
    buf += $("input#searchedParentName").val();
    buf += '&nbsp;';
    buf += '<a href="javascript:removeParent();">';
    buf += '<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">';
    buf += '</a>';
    buf += '<input type="hidden" id="parent" name="parent" value="';
    buf += $("input#searchedParentId").val() + '">';
    $("div#parentTicket").html(buf);
    addParentModal.hide();
});

function removeParent() {
    var buf = '<p class="my-1">';
    buf += '<a href="javascript:clickAddParent();">';
    buf += '<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="親チケットを追加">';
    buf += '</a></p>';
    buf += '<input type="hidden" id="parent" name="parent" value="">';
    $("div#parentTicket").html(buf);
}

function clickDeliverables() {
    new bootstrap.Modal("#deliverablesModal").show();
}
