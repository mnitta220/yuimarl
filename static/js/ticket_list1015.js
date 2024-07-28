let inChargeModal = new bootstrap.Modal("#inChargeModal");
let members = [];

function pageChange(idx) {
    $("#page").val(idx);
    $('#form_filter').submit();
}

$('#btnFilter').on('click', function () {
    $("#page").val(1);
    $('#form_filter').submit();
});

function clickSelectCharge() {
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
                members = ret.members;
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
                for (let i in members) {
                    buf += '<tr><td>';
                    buf += '<input class="form-check-input" type="radio" ';
                    buf += 'id="charge' + i + '" name="charge" value="';
                    buf += i + '"';
                    if (i == 0) {
                        buf += ' checked';
                    }
                    buf += '></td><td>';
                    switch (members[i].role) {
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
                    buf += members[i].email + '</td><td>';
                    buf += members[i].name + '</td><td>';
                    buf += '<input type="hidden" id="uid' + i + '" value="';
                    buf += members[i].uid + '">';
                    buf += '<input type="hidden" id="email' + i + '" value="';
                    buf += members[i].email + '">';
                    buf += '<input type="hidden" id="name' + i + '" value="';
                    buf += members[i].name + '">';
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

$('#btnSelectMember').on('click', function () {
    let idx = Number($('input[name="charge"]:checked').val());
    let member = members[idx];
    $('#charge-sel').html('<small>[' + member.email + ']</small> ' + member.name);
    $('#chargeuid').val(member.uid);
    $('#chargemail').val(member.email);
    $('#chargename').val(member.name);
    $('#charge2').removeClass('d-none');
    $('#charge1').addClass('d-none');
    inChargeModal.hide();
});

function clickRemoveCharge() {
    $('#chargeuid').val('');
    $('#chargemail').val('');
    $('#chargename').val('');
    $('#charge1').removeClass('d-none');
    $('#charge2').addClass('d-none');
}