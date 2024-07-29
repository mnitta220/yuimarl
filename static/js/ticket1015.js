let members = [];
let deliverables = [];
let addChargeModal = new bootstrap.Modal("#addChargeModal");
let addParentModal = new bootstrap.Modal("#addParentModal");
let deliverablesModal = new bootstrap.Modal("#deliverablesModal");

$(document).ready(function () {
    members = JSON.parse($("#members").val());
    deliverables = JSON.parse($("#deliverables").val());
});

function clickAddCharge() {
    $.ajax({
        type: "POST",
        url: "/api/projectMember",
        data: {
            project_id: $("input#project_id").val()
        },
        success: function (result) {
            let ret = JSON.parse(result);
            let buf = '';
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
                for (let i in ret.members) {
                    buf += '<tr><td>';
                    buf += '<input class="form-check-input" type="checkbox" ';
                    buf += `id="check${i}"`;
                    for (let j in members) {
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
                    buf += `<input type="hidden" id="uid${i}" value="${ret.members[i].uid}">`;
                    buf += `<input type="hidden" id="email${i}" value="${ret.members[i].email}">`;
                    buf += `<input type="hidden" id="name${i}" value="${ret.members[i].name}">`;
                    buf += '</td></tr>';
                }
                buf += '</tbody>';
                buf += '</table>';
            } else {
                buf += ret.message;
            }
            $("div#searched").html(buf);
            addChargeModal.show();
        }
    });
}

$('#btnMemberAdd').on('click', function () {
    for (i = 0; i < 1000; i++) {
        if ($('#check' + i)) {
            if ($('#check' + i).prop('checked')) {
                let member = {
                    uid: $(`#uid${i}`).val(),
                    email: $(`#email${i}`).val(),
                    name: $(`#name${i}`).val(),
                };
                let idx = members.findIndex(x => x.uid == member.uid);
                if (idx < 0) {
                    members.push(member);
                } else {
                    members[idx] = member;
                }
            } else {
                let idx = members.findIndex(x => x.uid == $(`#uid${i}`).val());
                if (idx >= 0) {
                    members.splice(idx, 1);
                }
            }
        } else {
            break;
        }
    }
    setChargeTable();
    addChargeModal.hide();
});

$('#btnSave').on('click', function () {
    $("#members").val(JSON.stringify(members));
    $("#deliverables").val(JSON.stringify(deliverables));
    $('#post_ticket').submit();
});

function removeCharge(idx) {
    members.splice(idx, 1);
    setChargeTable();
}

function chargeSeqUp(idx) {
    let j = Number(idx);
    let i = j - 1;
    [members[i], members[j]] = [members[j], members[i]];
    setChargeTable();
}

function chargeSeqDown(idx) {
    let i = Number(idx);
    let j = i + 1;
    [members[i], members[j]] = [members[j], members[i]];
    setChargeTable();
}

function setChargeTable() {
    let buf = '';
    let exist = false;
    if (members.length > 0) {
        buf += '<table class="table table-hover">';
        buf += '<thead><tr>';
        buf += '<th scope="col">メールアドレス</th>';
        buf += '<th scope="col">名前</th>';
        buf += '<th scope="col"></th>';
        buf += '</tr></thead>';
        buf += '<tbody>';
        for (let i in members) {
            buf += '<tr><td>';
            buf += members[i].email;
            buf += '</td><td>';
            buf += members[i].name;
            buf += '</td><td>';
            buf += `<a href="javascript:removeCharge(${i})">`;
            buf += '<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除"></a>';
            if (i != 0) {
                buf += `&nbsp;<a href="javascript:chargeSeqUp(${i})">`;
                buf += '<img class="icon" src="/static/ionicons/arrow-up-outline.svg" title="上に移動"></a>';
            }
            if ((Number(i) + 1) != members.length) {
                buf += `&nbsp;<a href="javascript:chargeSeqDown(${i})">`;
                buf += '<img class="icon" src="/static/ionicons/arrow-down-outline.svg" title="下に移動"></a>';
            }
            buf += '</td></tr>';

            if (members[i].uid == $("#session_uid").val()) {
                exist = true;
            }
        }
        buf += '</tbody></table>';
    }

    if (exist) {
        $('#back_color').removeClass('d-none');
    } else {
        $('#back_color').addClass('d-none');
    }

    $("div#chargeTbl").html(buf);
}

function clickAddParent() {
    addParentModal.show();
}

$('#search-parent').on('click', function () {
    if (`${$("input#search_id").val()}`.trim() == "") {
        let buf = '<p class="text-danger">チケットID を入力してください</p>';
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
            let ret = JSON.parse(result);
            let buf = '';
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

                buf += '<p><b>' + ret.ticket.id_disp + '</b> : ' + escape_html(ret.ticket.name) + '</p>';
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
    let buf = '<a href="/ticket?id=';
    buf += $("input#searchedParentId").val() + '">';
    buf += $("input#searchedParentIdDisp").val();
    buf += '</a>&nbsp;:&nbsp;';
    buf += escape_html($("input#searchedParentName").val());
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
    let buf = '<p class="my-1">';
    buf += '<a href="javascript:clickAddParent();">';
    buf += '<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="親チケットを追加">';
    buf += '</a></p>';
    buf += '<input type="hidden" id="parent" name="parent" value="">';
    $("div#parentTicket").html(buf);
}

function clickDeliverables() {
    $("input#deliverable-name").val("");
    $("input#deliverable-path").val("");
    $('#deliverable-name').removeClass('is-invalid');
    $('#deliverable-feedback').addClass('d-none');
    deliverablesModal.show();
}

$('#btnAddDeliverable').on('click', function () {
    if (`${$("input#deliverable-name").val()}`.trim() == "") {
        $('#deliverable-name').addClass('is-invalid');
        $('#deliverable-feedback').removeClass('d-none');
        return;
    }

    let deliverable = {
        name: $("input#deliverable-name").val(),
        path: $("input#deliverable-path").val(),
    };
    deliverables.push(deliverable);
    setDeliverablesTable();
    deliverablesModal.hide();
});

function removeDeliverable(idx) {
    deliverables.splice(idx, 1);
    setDeliverablesTable();
}

function setDeliverablesTable() {
    let buf = '';
    if (deliverables.length > 0) {
        buf += '<table class="table table-hover">';
        buf += '<thead><tr>';
        buf += '<th scope="col">成果物名</th>';
        buf += '<th scope="col">ファイルパス / URL</th>';
        buf += '<th scope="col"></th>';
        buf += '</tr></thead>';
        buf += '<tbody>';
        for (let i in deliverables) {
            buf += '<tr><td>';
            buf += escape_html(deliverables[i].name);
            buf += '</td><td>';
            if (deliverables[i].path.startsWith('http://') || deliverables[i].path.startsWith('https://')) {
                buf += '<a href="';
                buf += deliverables[i].path;
                buf += '" target="_blank">';
                buf += deliverables[i].path;
                buf += '</a>';
            } else {
                buf += escape_html(deliverables[i].path);
            }
            buf += '</td><td>';
            buf += '<a href="javascript:removeDeliverable(' + i + ')">';
            buf += '<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除"></a>';
            buf += '</td></tr>';
        }
        buf += '</tbody></table>';
    }
    $("div#deliverablesTbl").html(buf);
}

$('#finished').change(function () {
    let r = $(this).prop('checked');
    if (r) {
        $('#progress').val(100);
    }
})

$('#btnTicketDel').on('click', function () {
    $("#action").val('Delete');
    $('#post_ticket').submit();
});

function escape_html(string) {
    if (typeof string !== 'string') {
        return string;
    }
    return string.replace(/[&'`"<>]/g, function (match) {
        return {
            '&': '&amp;',
            "'": '&#x27;',
            '`': '&#x60;',
            '"': '&quot;',
            '<': '&lt;',
            '>': '&gt;',
        }[match]
    });
}

function clickColor(color) {
    $.ajax({
        type: "POST",
        url: "/api/ticketColor",
        data: {
            ticket_id: $("input#ticket_id").val(),
            color: color,
        },
        success: function (result) {
        }
    });

    switch ($("#color").val()) {
        case 'info':
            $('#img-info').addClass('d-none');
            break;
        case 'primary':
            $('#img-primary').addClass('d-none');
            break;
        case 'warning':
            $('#img-warning').addClass('d-none');
            break;
        case 'success':
            $('#img-success').addClass('d-none');
            break;
        case 'danger':
            $('#img-danger').addClass('d-none');
            break;
        case 'secondary':
            $('#img-secondary').addClass('d-none');
            break;
        case 'dark':
            $('#img-dark').addClass('d-none');
            break;
        case 'light':
            $('#img-light').addClass('d-none');
            break;
    }
    $("#color").val(color);
    switch (color) {
        case 'info':
            $('#img-info').removeClass('d-none');
            break;
        case 'primary':
            $('#img-primary').removeClass('d-none');
            break;
        case 'warning':
            $('#img-warning').removeClass('d-none');
            break;
        case 'success':
            $('#img-success').removeClass('d-none');
            break;
        case 'danger':
            $('#img-danger').removeClass('d-none');
            break;
        case 'secondary':
            $('#img-secondary').removeClass('d-none');
            break;
        case 'dark':
            $('#img-dark').removeClass('d-none');
            break;
        case 'light':
            $('#img-light').removeClass('d-none');
            break;
    }
}
