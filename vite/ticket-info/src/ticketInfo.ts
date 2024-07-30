import AddChargeModal from "./addChargeModal";
import AddParentModal from "./addParentModal";

export interface IMember {
  id: string; // チケットメンバーID
  uid: string; // メンバーのユーザーID
  role: number | null; // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
  seq: number | null; // 表示順
  name: string | null; // メンバーの名前
  email: string | null; // メンバーのメールアドレス
}

export interface IDeliverable {
  name: string | null;
  path: string | null;
}

export default class TicketInfo {
  members: IMember[] = [];
  deliverables: IDeliverable[] = [];
  addChargeModal = new AddChargeModal(this);
  addParentModal = new AddParentModal(this);

  constructor() {
    this.handler();
  }

  // イベントハンドラを登録する
  private handler() {
    const icnRemoveParent =
      document.querySelector<HTMLButtonElement>(`#icnRemoveParent`);
    if (icnRemoveParent) {
      icnRemoveParent.addEventListener("click", () => {
        this.removeParent();
      });
    }

    this.addChargeModal.handler();
    this.addParentModal.handler();
  }

  load() {
    this.members = [];
    const ms = document.querySelector<HTMLInputElement>(`#members`);
    if (ms?.value) {
      this.members = JSON.parse(ms.value);
    }

    this.deliverables = [];
    const ds = document.querySelector<HTMLInputElement>(`#deliverables`);
    if (ds?.value) {
      this.deliverables = JSON.parse(ds.value);
    }
    this.memberHandler();
  }

  private memberHandler() {
    for (let i in this.members) {
      let icnRemoveMem = document.querySelector<HTMLImageElement>(
        `#icnRemoveMem${i}`
      );
      if (icnRemoveMem) {
        icnRemoveMem.addEventListener("click", () => {
          this.removeCharge(Number(i));
        });
      }

      let icnUpMem = document.querySelector<HTMLImageElement>(`#icnUpMem${i}`);
      if (icnUpMem) {
        icnUpMem.addEventListener("click", () => {
          this.chargeSeqUp(Number(i));
        });
      }

      let icnDownMem = document.querySelector<HTMLImageElement>(
        `#icnDownMem${i}`
      );
      if (icnDownMem) {
        icnDownMem.addEventListener("click", () => {
          this.chargeSeqDown(Number(i));
        });
      }
    }
  }

  private removeCharge(i: number) {
    this.members.splice(i, 1);
    this.setChargeTable();
  }

  private chargeSeqUp(idx: number) {
    let j = idx;
    let i = j - 1;
    [this.members[i], this.members[j]] = [this.members[j], this.members[i]];
    this.setChargeTable();
  }

  private chargeSeqDown(idx: number) {
    let i = idx;
    let j = i + 1;
    [this.members[i], this.members[j]] = [this.members[j], this.members[i]];
    this.setChargeTable();
  }

  setChargeTable() {
    let buf = "";
    let exist = false;
    if (this.members.length > 0) {
      buf += `<table class="table table-hover">`;
      {
        buf += `<thead>`;
        {
          buf += `<tr>`;
          {
            buf += `<th scope="col">メールアドレス</th>`;
            buf += `<th scope="col">名前</th>`;
            buf += `<th scope="col"></th>`;
          }
          buf += `</tr>`;
        }
        buf += `</thead>`;
        buf += `<tbody>`;
        for (let i in this.members) {
          buf += `<tr>`;
          {
            buf += `<td>${this.members[i].email}</td>`;
            buf += `<td>${this.members[i].name}</td>`;
            buf += `<td>`;
            buf += `<img class="icon" style="cursor:pointer" id="icnRemoveMem${i}" src="/static/ionicons/remove-circle-outline.svg" title="削除">`;
            if (Number(i) != 0) {
              buf += `&nbsp;<img class="icon" style="cursor:pointer" id="icnUpMem${i}" src="/static/ionicons/arrow-up-outline.svg" title="上に移動">`;
            }
            if (Number(i) + 1 != this.members.length) {
              buf += `&nbsp;<img class="icon" style="cursor:pointer" id="icnDownMem${i}" src="/static/ionicons/arrow-down-outline.svg" title="下に移動">`;
            }
            buf += `</td>`;
          }
          buf += `</tr>`;

          const su = document.querySelector<HTMLInputElement>(`#session_uid`);
          if (su?.value) {
            if (this.members[i].uid == su.value) {
              exist = true;
            }
          }
        }
        buf += `</tbody>`;
      }
      buf += `</table>`;
    }

    const bc = document.querySelector<HTMLInputElement>(`#back_color`);
    if (bc) {
      if (exist) {
        bc.classList.remove("d-none");
      } else {
        bc.classList.add("d-none");
      }
    }

    const ct = document.querySelector<HTMLElement>(`div#chargeTbl`);
    if (ct) ct.innerHTML = buf;

    this.memberHandler();
  }

  removeParent() {
    let buf = '<p class="my-1">';
    buf +=
      '<img class="icon3" style="cursor:pointer" id="icnAddParent" src="/static/ionicons/add-circle-outline.svg" title="親チケットを追加">';
    buf += "</p>";
    buf += '<input type="hidden" id="parent" name="parent" value="">';
    const parentTicket = document.querySelector<HTMLElement>(`#parentTicket`);
    if (parentTicket) {
      parentTicket.innerHTML = buf;
    }

    const icnAddParent =
      document.querySelector<HTMLButtonElement>(`#icnAddParent`);
    if (icnAddParent) {
      icnAddParent.addEventListener("click", () => {
        this.addParentModal.show();
      });
    }
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

  escapeHtml(str: string): string {
    return str.replace(/[&'`"<>]/g, (match) => {
      switch (match) {
        case "&":
          return "&amp;";
        case "'":
          return "&#x27;";
        case "`":
          return "&#x60;";
        case '"':
          return "&quot;";
        case "<":
          return "&lt;";
        case ">":
          return "&gt;";
        default:
          return match;
      }
    });
  }
}
