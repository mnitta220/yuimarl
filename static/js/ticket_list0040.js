function pageChange(idx) {
    $("#page").val(idx);
    $('#form_filter').submit();
}

$('#btnFilter').on('click', function () {
    $("#page").val(1);
    $('#form_filter').submit();
});
