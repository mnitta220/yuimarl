import * as bootstrap from "bootstrap";

export default class MemoModal {
  private id = "memoModal";
  private modal: bootstrap.Modal | null = null;

  constructor() {
    const memoModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (memoModal) {
      this.modal = new bootstrap.Modal(memoModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const icnEditMemo =
      document.querySelector<HTMLButtonElement>(`#icnEditMemo`);
    if (icnEditMemo) {
      icnEditMemo.addEventListener("click", () => {
        this.show();
      });
    }

    const btnUpdate = document.querySelector<HTMLButtonElement>(`#btnUpdate`);
    if (btnUpdate) {
      btnUpdate.addEventListener("click", () => {
        this.update();
      });
    }
  }

  show() {
    if (this.modal) {
      this.modal.show();
    }
  }

  hide() {
    if (this.modal) {
      this.modal.hide();
    }
  }

  update() {
    const form = document.querySelector<HTMLFormElement>(`#post_memo`);
    if (form) {
      form.submit();
    }
  }
}
