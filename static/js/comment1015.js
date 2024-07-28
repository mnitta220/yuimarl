let commentModal = new bootstrap.Modal("#commentModal");
let deleteCommentModal = new bootstrap.Modal("#deleteCommentModal");

function editComment(idx) {
    $("#commentModalLabel").html('コメントを編集');
    $('#btnAdd').addClass('d-none');
    $('#btnUpdate').removeClass('d-none');
    let cid = '#cid' + idx;
    let mid = '#mkd' + idx;
    if ($(cid) && $(mid)) {
        $("#comment_id").val($(cid).val());
        $('#markdown').val($(mid).val());
        let html = marked.parse($(mid).val());
        $('#preview1').html(html);
        commentModal.show();
    }
}

function deleteComment(idx) {
    let cid = '#cid' + idx;
    if ($(cid)) {
        $("#comment_id").val($(cid).val());
        deleteCommentModal.show();
    }
}

$('#markdown').keyup(function () {
    let html = marked.parse($(this).val());
    $('#preview1').html(html);
});

function clickComment() {
    $("#comment_id").val('');
    $("#commentModalLabel").html('コメントを追加');
    $('#btnUpdate').addClass('d-none');
    $('#btnAdd').removeClass('d-none');
    $('#markdown').val('');
    $('#preview1').html('');
    commentModal.show();
}

$(document).ready(function () {
    for (i = 0; i < 100; i++) {
        let mid = '#mkd' + i;
        if ($(mid)) {
            let html = marked.parse($(mid).val());
            $('#pre' + i).html(html);
        } else {
            break;
        }
    }
});

$('#btnAdd').on('click', function () {
    $("#action").val('Create');
    $('#comment').val($('#markdown').val());
    $('#post_comment').submit();
});

$('#btnUpdate').on('click', function () {
    $("#action").val('Update');
    $('#comment').val($('#markdown').val());
    $('#post_comment').submit();
});

$('#btnDelete').on('click', function () {
    $("#action").val('Delete');
    $('#post_comment').submit();
});
