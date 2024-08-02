pub fn project_info_footer(buf: &mut String) {
    *buf += r#"<input class="invisible" type="button" id="btnSetMember" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnRemoveMember" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnSetMember" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnRemoveMember" value="">"#;

    *buf += r#"<script>
function clickSetMember(idx) {
  const btnSetMember = document.querySelector(`#btnSetMember`);
  if (btnSetMember) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnSetMember.click();
    }
  }
}

function clickRemoveMember(idx) {
  const btnRemoveMember = document.querySelector(`#btnRemoveMember`);
  if (btnRemoveMember) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnRemoveMember.click();
    }
  }
}
</script>"#;
}
