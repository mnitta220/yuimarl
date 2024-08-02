pub fn ticket_info_footer(buf: &mut String) {
    *buf += r#"<input class="invisible" type="button" id="btnAddParentInvisible" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnRemoveParent" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnRemoveCharge" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnUpCharge" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnDownCharge" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnRemoveDeliverable" value="">"#;
    *buf += r#"<input class="invisible" type="button" id="btnSelectColor" value="">"#;
    *buf += r#"<input type="hidden" name="selectedIndex" id="selectedIndex" value="">"#;
    *buf += r#"<input type="hidden" name="selectedColor" id="selectedColor" value="light">"#;

    *buf += r#"<script>
function clickRemoveCharge(idx) {
  const btnRemoveCharge = document.querySelector(`#btnRemoveCharge`);
  if (btnRemoveCharge) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnRemoveCharge.click();
    }
  }
}

function clickUpCharge(idx) {
  const btnUpCharge = document.querySelector(`#btnUpCharge`);
  if (btnUpCharge) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnUpCharge.click();
    }
  }
}

function clickDownCharge(idx) {
  const btnDownCharge = document.querySelector(`#btnDownCharge`);
  if (btnDownCharge) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnDownCharge.click();
    }
  }
}

function clickAddParentInvisible() {
  const btnAddParentInvisible = document.querySelector(`#btnAddParentInvisible`);
  if (btnAddParentInvisible) {
    btnAddParentInvisible.click();
  }
}

function clickRemoveParent() {
  const btnRemoveParent = document.querySelector(`#btnRemoveParent`);
  if (btnRemoveParent) {
    btnRemoveParent.click();
  }
}

function removeDeliverable(idx) {
  const btnRemoveDeliverable = document.querySelector(`#btnRemoveDeliverable`);
  if (btnRemoveDeliverable) {
    const selectedIndex = document.querySelector(`#selectedIndex`);
    if (selectedIndex) {
      selectedIndex.value = `${idx}`;
      btnRemoveDeliverable.click();
    }
  }
}

function clickColor(color) {
  const btnSelectColor = document.querySelector(`#btnSelectColor`);
  if (btnSelectColor) {
    const selectedColor = document.querySelector(`#selectedColor`);
    if (selectedColor) {
      selectedColor.value = `${color}`;
      btnSelectColor.click();
    }
  }
}
</script>"#;
}
