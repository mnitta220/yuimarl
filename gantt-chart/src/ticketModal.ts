import * as bootstrap from "bootstrap";
import { TicketModalResult } from "./common";

export default class TicketModal {
  private id = "ticketModal";
  private modal: bootstrap.Modal | null = null;
  private ticketId: string | null = null;

  constructor() {
    const ticketModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (ticketModal) {
      this.modal = new bootstrap.Modal(ticketModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const btnRedirectTicket =
      document.querySelector<HTMLButtonElement>(`#btnRedirectTicket`);
    if (btnRedirectTicket) {
      btnRedirectTicket.addEventListener("click", () => {
        if (this.modal) {
          this.modal.hide();
          // チケット画面に遷移する
          window.location.href = `/ticket?id=${this.ticketId}`;
        }
      });
    }
  }

  show(data: TicketModalResult) {
    if (this.modal && data.ticket) {
      this.ticketId = data.ticket.id;

      const ticket_id = document.querySelector("#modal_ticket_id");
      if (ticket_id) {
        ticket_id.textContent = data.ticket.id_disp ?? "";
      }

      const ticket_name =
        document.querySelector<HTMLInputElement>("#ticket_name");
      if (ticket_name) {
        ticket_name.value = data.ticket.name ?? "";
      }

      const description = document.querySelector("#description");
      if (description) {
        description.innerHTML = data.ticket.description ?? "";
      }

      if (data.members && data.members.length > 0) {
        const members = document.querySelector("#members");
        if (members) {
          let buf = `<table class="table table-hover">`;
          buf += `<thead>`;
          buf += `<tr>`;
          buf += `<th scope="col">メールアドレス</th>`;
          buf += `<th scope="col">名前</th>`;
          buf += `</tr>`;
          buf += `</thead>`;
          buf += `<tbody>`;
          for (const m of data.members) {
            buf += `<tr>`;
            buf += `<td>${m.email}</td>`;
            buf += `<td>${m.name}</td>`;
            buf += `</tr>`;
          }
          buf += `</tbody>`;
          buf += `</table>`;
          members.innerHTML = buf;
        }
      }

      const startdate = document.querySelector("#start_date");
      if (startdate) {
        let buf = `<label class="form-label" for="startdate">開始日</label>`;
        buf += `<input class="form-control" id="startdate" type="date" value="${
          data.ticket.start_date ?? ""
        }">`;
        startdate.innerHTML = buf;
      }

      const enddate = document.querySelector("#end_date");
      if (enddate) {
        let buf = `<label class="form-label" for="enddate">終了日</label>`;
        buf += `<input class="form-control" id="enddate" type="date" value="${
          data.ticket.end_date ?? ""
        }">`;
        enddate.innerHTML = buf;
      }

      const progress = document.querySelector<HTMLInputElement>("#progress");
      if (progress) {
        progress.value = `${data.ticket.progress}`;
      }

      const finished = document.querySelector<HTMLInputElement>("#finished");
      if (finished) {
        finished.checked = data.ticket.progress === 100;
      }

      const priority = document.querySelector<HTMLInputElement>(
        `#priority${data.ticket.priority}`
      );
      if (priority) {
        priority.checked = true;
      }

      if (data.parent) {
        const parent = document.querySelector("#parent");
        if (parent) {
          parent.innerHTML = `${data.parent.id_disp}&nbsp;&nbsp;${data.parent.name}`;
        }
      }

      if (data.children) {
        const children = document.querySelector("#children");
        if (children) {
          let buf = "";
          for (const c of data.children) {
            buf += `<p class="mb-0">${c.id_disp}&nbsp;&nbsp;${c.name}</p>`;
          }
          children.innerHTML = buf;
        }
      }

      this.modal.show();
    }
  }

  hide() {
    if (this.modal) {
      this.modal.hide();
    }
  }
}
