pub fn ticket_comment_footer(buf: &mut String) {
    *buf += r#"<input class="invisible" type="button" id="btnEditComment" value="">"#;

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
            </script>"#;
}
