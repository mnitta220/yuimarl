import * as bootstrap from "bootstrap";
import TicketInfo, { IDeliverable } from "./ticketInfo";

export default class DeliverablesModal {
  private id = "deliverablesModal";
  private modal: bootstrap.Modal | null = null;

  constructor(private info: TicketInfo) {
    const deliverablesModal = document.querySelector<HTMLDivElement>(
      `#${this.id}`
    );
    if (deliverablesModal) {
      this.modal = new bootstrap.Modal(deliverablesModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const icnAddDeliverables =
      document.querySelector<HTMLButtonElement>(`#icnAddDeliverables`);
    if (icnAddDeliverables) {
      icnAddDeliverables.addEventListener("click", () => {
        this.clickDeliverables();
      });
    }

    const btnAddDeliverable =
      document.querySelector<HTMLButtonElement>(`#btnAddDeliverable`);
    if (btnAddDeliverable) {
      btnAddDeliverable.addEventListener("click", () => {
        this.addDeliverable();
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

  private clickDeliverables() {
    const name = document.querySelector<HTMLInputElement>(
      `input#deliverable-name`
    );
    if (name) {
      name.value = "";
      name.classList.remove("is-invalid");
    }
    const path = document.querySelector<HTMLInputElement>(
      `input#deliverable-path`
    );
    if (path) {
      path.value = "";
    }
    const feedback = document.querySelector<HTMLDivElement>(
      `#deliverable-feedback`
    );
    if (feedback) {
      feedback.classList.add("d-none");
    }
    this.show();
  }

  private addDeliverable() {
    let name = "";
    const inputName = document.querySelector<HTMLInputElement>(
      `input#deliverable-name`
    );
    if (inputName) {
      name = inputName.value;
      if (name.trim() == "") {
        inputName.classList.add("is-invalid");
        const feedback = document.querySelector<HTMLDivElement>(
          `#deliverable-feedback`
        );
        if (feedback) {
          feedback.classList.remove("d-none");
        }
        return;
      }
    }
    let path = "";
    const inputPath = document.querySelector<HTMLInputElement>(
      `input#deliverable-path`
    );
    if (inputPath) {
      path = inputPath.value;
    }

    let deliverable = {
      name,
      path,
    } as IDeliverable;

    this.info.deliverables.push(deliverable);
    this.info.setDeliverablesTable();

    this.hide();
  }
}
