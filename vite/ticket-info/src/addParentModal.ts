import * as bootstrap from "bootstrap";
import TicketInfo from "./ticketInfo";

interface ITicket {
  id: string; // チケットID
  id_disp: string; // チケットID
  name: string | null; // 名前
}

interface ITicketResult {
  result: boolean;
  ticket: ITicket | null;
  message: string;
}

export default class AddParentModal {
  private id = "addParentModal";
  private modal: bootstrap.Modal | null = null;

  constructor(private info: TicketInfo) {
    const parentModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (parentModal) {
      this.modal = new bootstrap.Modal(parentModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const btnAddParentInvisible = document.querySelector<HTMLButtonElement>(
      `#btnAddParentInvisible`
    );
    if (btnAddParentInvisible) {
      btnAddParentInvisible.addEventListener("click", () => {
        this.show();
      });
    }

    const btnSearchParent =
      document.querySelector<HTMLButtonElement>(`#search-parent`);
    if (btnSearchParent) {
      btnSearchParent.addEventListener("click", () => {
        this.searchParent();
      });
    }

    const btnAddParent =
      document.querySelector<HTMLButtonElement>(`#btnAddParent`);
    if (btnAddParent) {
      btnAddParent.addEventListener("click", () => {
        this.addParent();
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

  private searchParent() {
    const searchId =
      document.querySelector<HTMLInputElement>(`input#search_id`);
    if (searchId) {
      let id = searchId.value.trim();

      if (id.length == 0) {
        let buf = '<p class="text-danger">チケットID を入力してください</p>';
        buf += '<input type="hidden" id="searchedParentId" value="">';
        buf += '<input type="hidden" id="searchedParentIdDisp" value="">';
        buf += '<input type="hidden" id="searchedParentName" value="">';
        const btnAddParent =
          document.querySelector<HTMLButtonElement>(`#btnAddParent`);
        if (btnAddParent) {
          btnAddParent.setAttribute("disabled", "disabled");
        }
        const searchedParent =
          document.querySelector<HTMLElement>(`#searchedParent`);
        if (searchedParent) {
          searchedParent.innerHTML = buf;
        }
        return;
      }

      let buf = `project_id=`;
      const projectId = document.querySelector<HTMLInputElement>(`#project_id`);
      if (projectId) {
        buf += projectId.value;
      }
      buf += `&id_disp=${id}`;

      fetch("/api/ticketByIdDisp", {
        method: "POST",
        headers: {
          Accept: "*/*",
          "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
        },
        mode: "cors",
        body: encodeURI(buf),
      })
        .then((response) => response.json())
        .then((data: ITicketResult) => {
          this.ticketSearchResult(data);
        })
        .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
    }
  }

  private ticketSearchResult(res: ITicketResult) {
    let buf = "";
    if (res.result && res.ticket) {
      const ticketId = document.querySelector<HTMLInputElement>(`#ticket_id`);
      if (ticketId) {
        if (res.ticket.id == ticketId.value) {
          buf +=
            '<p class="text-danger">自身を親チケットにすることはできません</p>';
          buf += '<input type="hidden" id="searchedParentId" value="">';
          buf += '<input type="hidden" id="searchedParentIdDisp" value="">';
          buf += '<input type="hidden" id="searchedParentName" value="">';
          const btnAddParent =
            document.querySelector<HTMLButtonElement>(`#btnAddParent`);
          if (btnAddParent) {
            btnAddParent.setAttribute("disabled", "disabled");
          }
          const searchedParent =
            document.querySelector<HTMLElement>(`#searchedParent`);
          if (searchedParent) {
            searchedParent.innerHTML = buf;
          }
          return;
        }
      }

      buf +=
        `<p><b>${res.ticket.id_disp}</b> : ` +
        this.info.escapeHtml(res.ticket.name ?? "") +
        `</p>`;
      buf += `<input type="hidden" id="searchedParentId" value="`;
      buf += res.ticket.id + `">`;
      buf += `<input type="hidden" id="searchedParentIdDisp" value="`;
      buf += res.ticket.id_disp + `">`;
      buf += `<input type="hidden" id="searchedParentName" value="`;
      buf += res.ticket.name + `">`;
      const btnAddParent =
        document.querySelector<HTMLButtonElement>(`#btnAddParent`);
      if (btnAddParent) {
        btnAddParent.removeAttribute("disabled");
      }
    } else {
      buf += `<p class="text-danger">`;
      buf += res.message;
      buf += `</p>`;
      buf += `<input type="hidden" id="searchedParentId" value="">`;
      buf += `<input type="hidden" id="searchedParentIdDisp" value="">`;
      buf += `<input type="hidden" id="searchedParentName" value="">`;
      const btnAddParent =
        document.querySelector<HTMLButtonElement>(`#btnAddParent`);
      if (btnAddParent) {
        btnAddParent.setAttribute("disabled", "disabled");
      }
    }

    const searchedParent =
      document.querySelector<HTMLElement>(`#searchedParent`);
    if (searchedParent) {
      searchedParent.innerHTML = buf;
    }
  }

  private addParent() {
    const parentId = document.querySelector<HTMLInputElement>(
      `input#searchedParentId`
    );
    const parentIdDisp = document.querySelector<HTMLInputElement>(
      `input#searchedParentIdDisp`
    );
    const parentName = document.querySelector<HTMLInputElement>(
      `input#searchedParentName`
    );

    let buf = `<a href="/ticket?id=`;
    if (parentId) {
      buf += parentId.value;
    }
    buf += `">`;
    if (parentIdDisp) {
      buf += parentIdDisp.value;
    }
    buf += `</a>&nbsp;:&nbsp;`;
    if (parentName) {
      buf += this.info.escapeHtml(parentName.value);
    }
    buf += `&nbsp;`;
    buf += `<a href="javascript:clickRemoveParent();">`;
    buf += `<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">`;
    buf += `</a>`;
    buf += `<input type="hidden" id="parent_id" name="parent_id" value="`;
    if (parentId) {
      buf += parentId.value;
    }
    buf += `">`;
    buf += `<input type="hidden" id="parent_id_disp" name="parent_id_disp" value="`;
    if (parentIdDisp) {
      buf += parentIdDisp.value;
    }
    buf += `">`;
    buf += `<input type="hidden" id="parent_name" name="parent_name" value="`;
    if (parentName) {
      buf += this.info.escapeHtml(parentName.value);
    }
    buf += `">`;

    const parentTicket = document.querySelector<HTMLElement>(`#parentTicket`);
    if (parentTicket) {
      parentTicket.innerHTML = buf;
    }

    this.hide();
  }
}
