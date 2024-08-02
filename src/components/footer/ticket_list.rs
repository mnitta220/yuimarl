pub fn ticket_list_footer(buf: &mut String) {
    *buf += r#"<input class="invisible" type="button" id="btnAddCharge" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnRemoveCharge" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnPageCharge" value="">"#;

    *buf += r#"<script>
        function clickSelectCharge() {
          const btnAddCharge = document.querySelector(`#btnAddCharge`);
          if (btnAddCharge) {
            btnAddCharge.click();
          }
        }

        function clickRemoveCharge() {
          const btnRemoveCharge = document.querySelector(`#btnRemoveCharge`);
          if (btnRemoveCharge) {
            btnRemoveCharge.click();
          }
        }
  
        function pageChange(idx) {
          const page = document.querySelector(`#page`);
          if (page) {
            page.value = `${idx}`;
          }
          const btnPageCharge = document.querySelector(`#btnPageCharge`);
          if (btnPageCharge) {
            btnPageCharge.click();
          }
        }
        </script>"#;
}
