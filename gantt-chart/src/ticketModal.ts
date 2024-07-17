import * as bootstrap from "bootstrap";
import { TicketModalResult } from "./common";

export default class TicketModal {
  private id = "ticketModal";
  private modal: bootstrap.Modal | null = null;

  constructor() {
    const ticketModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (ticketModal) {
      this.modal = new bootstrap.Modal(ticketModal);
    }
  }

  show(data: TicketModalResult) {
    if (this.modal && data.ticket) {
      const ticket_id = document.querySelector("#modal_ticket_id");
      if (ticket_id) {
        ticket_id.textContent = data.ticket.id_disp ?? "";
      }
      const ticket_name =
        document.querySelector<HTMLInputElement>("#modal_ticket_name");
      if (ticket_name) {
        ticket_name.value = data.ticket.name ?? "";
      }
      /*document.querySelector("#modal_ticket_id")!.textContent =
        data.ticket.id_disp ?? "";
      document.querySelector("#modal_ticket_name")!.innerHTML =
        data.ticket.name ?? "";
      document.querySelector<HTMLInputElement>("#modal_ticket_name")!.value =
        data.ticket.name ?? "";
      */
      this.modal.show();
    }
  }

  hide() {
    if (this.modal) {
      this.modal.hide();
    }
  }
}
