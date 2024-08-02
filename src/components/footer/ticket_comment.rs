pub fn ticket_comment_footer(buf: &mut String) {
    *buf += r#"<input class="invisible" type="button" id="btnEditComment" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnDeleteComment" value="">"#;
    *buf += r#"<input type="hidden" id="selectedIndex" name="selectedIndex" value="">"#;

    *buf += r#"<script>
function editComment(idx) {
  const btnEditComment = document.querySelector(`#btnEditComment`);
  if (btnEditComment) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnEditComment.click();
    }
  }
}

function deleteComment(idx) {
  const btnDeleteComment = document.querySelector(`#btnDeleteComment`);
  if (btnDeleteComment) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnDeleteComment.click();
    }
  }
}
</script>"#;
}
