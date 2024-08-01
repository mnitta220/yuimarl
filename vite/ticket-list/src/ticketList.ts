import ChargeModal from "./chargeModal";

export default class TicketList {
  chargeModal = new ChargeModal(this);

  constructor() {
    this.handler();
  }

  // イベントハンドラを登録する
  private handler() {
    const btnRemoveCharge =
      document.querySelector<HTMLButtonElement>(`#btnRemoveCharge`);
    if (btnRemoveCharge) {
      btnRemoveCharge.addEventListener("click", () => {
        this.removeCharge();
      });
    }

    const btnFilter = document.querySelector<HTMLButtonElement>(`#btnFilter`);
    if (btnFilter) {
      btnFilter.addEventListener("click", () => {
        const page = document.querySelector<HTMLInputElement>(`#page`);
        if (page) {
          page.value = "1";
        }

        const form = document.querySelector<HTMLFormElement>(`#form_filter`);
        if (form) {
          form.submit();
        }
      });
    }

    const btnPageCharge =
      document.querySelector<HTMLButtonElement>(`#btnPageCharge`);
    if (btnPageCharge) {
      btnPageCharge.addEventListener("click", () => {
        const form = document.querySelector<HTMLFormElement>(`#form_filter`);
        if (form) {
          form.submit();
        }
      });
    }

    this.chargeModal.handler();
  }

  private removeCharge() {
    const uid = document.querySelector<HTMLInputElement>(`#chargeuid`);
    if (uid) uid.value = "";
    const mail = document.querySelector<HTMLInputElement>(`#chargemail`);
    if (mail) mail.value = "";
    const name = document.querySelector<HTMLInputElement>(`#chargename`);
    if (name) name.value = "";
    const charge1 = document.querySelector<HTMLElement>(`#charge1`);
    if (charge1) charge1.classList.remove("d-none");
    const charge2 = document.querySelector<HTMLElement>(`#charge2`);
    if (charge2) charge2.classList.add("d-none");
  }

  roleToString(role: number | null): string {
    if (!role) return "";
    switch (role) {
      case 1:
        return "オーナー";
      case 2:
        return "管理者";
      case 3:
        return "メンバー";
      case 4:
        return "閲覧者";
    }
    return "";
  }
}
