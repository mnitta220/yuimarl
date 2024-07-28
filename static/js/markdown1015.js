$('#markdown').keyup(function () {
    let html = marked.parse($(this).val());
    $('#preview1').html(html);
    $('#preview2').html(html);
});

$('#edit').change(function () {
    if ($(this).prop('checked')) {
        $('#note1').removeClass('d-none');
        $('#note2').addClass('d-none');
    } else {
        $('#note2').removeClass('d-none');
        $('#note1').addClass('d-none');
    }
});

$(document).ready(function () {
    let html = marked.parse($('#markdown').val());
    $('#preview1').html(html);
    $('#preview2').html(html);
});

