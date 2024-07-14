import * as bootstrap from "bootstrap";
//import GanttFrame from "./ganttFrame";

export default class TicketModal {
  private id = "ticketModal";
  private modal: bootstrap.Modal | null = null;

  constructor() {
    const ticketModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (ticketModal) {
      this.modal = new bootstrap.Modal(ticketModal);
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
}
