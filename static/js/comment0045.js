var comments = [];

function editComment(idx) {
    console.log('editComment: ' + idx);
    $('#view' + idx).addClass('d-none');
    $('#edit' + idx).removeClass('d-none');
    //$('#newComment').addClass('d-none');
    $("#btnAddComment").attr({ 'disabled': 'disabled' });
    $("#comment").attr({ 'disabled': 'disabled' });
}

function cancelComment(idx) {
    console.log('cancelComment: ' + idx);
    $('#edit' + idx).addClass('d-none');
    $('#view' + idx).removeClass('d-none');
    //$('#newComment').removeClass('d-none');
    $("#btnAddComment").removeAttr('disabled');
    $("#comment").removeAttr('disabled');
}
