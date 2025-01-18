import AddChargeModal from "./addChargeModal";
import AddParentModal from "./addParentModal";
import DeliverablesModal from "./deliverablesModal";

export interface IMember {
  id: string; // チケットメンバーID
  uid: string; // メンバーのユーザーID
  role: number | null; // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
  seq: number | null; // 表示順
  name: string | null; // メンバーの名前
  email: string | null; // メンバーのメールアドレス
}

export interface IDeliverable {
  name: string;
  path: string;
}

export default class TicketInfo {
  members: IMember[] = [];
  deliverables: IDeliverable[] = [];
  addChargeModal = new AddChargeModal(this);
  addParentModal = new AddParentModal(this);
  deliverablesModal = new DeliverablesModal(this);

  constructor() {
    this.handler();
  }

  // イベントハンドラを登録する
  private handler() {
    const btnRemoveParent =
      document.querySelector<HTMLButtonElement>(`#btnRemoveParent`);
    if (btnRemoveParent) {
      btnRemoveParent.addEventListener("click", () => {
        this.removeParent();
      });
    }

    const btnRemoveCharge =
      document.querySelector<HTMLButtonElement>(`#btnRemoveCharge`);
    if (btnRemoveCharge) {
      btnRemoveCharge.addEventListener("click", () => {
        const selectedIndex =
          document.querySelector<HTMLInputElement>(`#selectedIndex`);
        if (selectedIndex) {
          this.removeCharge(Number(selectedIndex.value));
        }
      });
    }

    console.log(`***up4`);

    const btnUpCharge =
      document.querySelector<HTMLButtonElement>(`#btnUpCharge`);
    if (btnUpCharge) {
      console.log(`***up5`);
      btnUpCharge.addEventListener("click", () => {
        console.log(`***up6`);
        const selectedIndex =
          document.querySelector<HTMLInputElement>(`#selectedIndex`);
        if (selectedIndex) {
          console.log(`***up7`);
          this.chargeSeqUp(Number(selectedIndex.value));
        }
      });
    }

    const btnDownCharge =
      document.querySelector<HTMLButtonElement>(`#btnDownCharge`);
    if (btnDownCharge) {
      btnDownCharge.addEventListener("click", () => {
        const selectedIndex =
          document.querySelector<HTMLInputElement>(`#selectedIndex`);
        if (selectedIndex) {
          this.chargeSeqDown(Number(selectedIndex.value));
        }
      });
    }

    const finished = document.querySelector<HTMLInputElement>(`#finished`);
    if (finished) {
      finished.addEventListener("change", () => {
        if (finished.checked) {
          const progress =
            document.querySelector<HTMLInputElement>(`#progress`);
          if (progress) {
            progress.value = "100";
          }
        }
      });
    }

    const btnTicketDel =
      document.querySelector<HTMLButtonElement>(`#btnTicketDel`);
    if (btnTicketDel) {
      btnTicketDel.addEventListener("click", () => {
        const action = document.querySelector<HTMLInputElement>(`#action`);
        if (action) {
          action.value = "Delete";
          const form = document.querySelector<HTMLFormElement>(`#post_ticket`);
          if (form) {
            form.submit();
          }
        }
      });
    }

    const btnRemoveDeliverable = document.querySelector<HTMLButtonElement>(
      `#btnRemoveDeliverable`
    );
    if (btnRemoveDeliverable) {
      btnRemoveDeliverable.addEventListener("click", () => {
        this.removeDeliverable();
      });
    }

    const btnSelectColor =
      document.querySelector<HTMLButtonElement>(`#btnSelectColor`);
    if (btnSelectColor) {
      btnSelectColor.addEventListener("click", () => {
        this.selectColor();
      });
    }

    const btnSave = document.querySelector<HTMLButtonElement>(`#btnSave`);
    if (btnSave) {
      btnSave.addEventListener("click", () => {
        this.save();
      });
    }

    this.addChargeModal.handler();
    this.addParentModal.handler();
    this.deliverablesModal.handler();
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

    const toastMessage =
      document.querySelector<HTMLInputElement>(`#toast_message`);
    if (toastMessage?.value) {
      // トーストを表示する
      setTimeout(function () {
        const toast = document.getElementById("toast");
        if (toast) {
          toast.innerHTML = toastMessage.value;
          toast.style.visibility = "visible";
          setTimeout(function () {
            toast.style.visibility = "hidden";
            toastMessage.value = "";
          }, 1500);
        }
      }, 100);
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
            {
              buf += `<a href="javascript:clickRemoveCharge(${i});">`;
              buf += `<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">`;
              buf += `</a>`;

              if (Number(i) != 0) {
                buf += `<a href="javascript:clickUpCharge(${i});">`;
                buf += `<img class="icon" src="/static/ionicons/arrow-up-outline.svg" title="上に移動">`;
                buf += `</a>`;
              }
              if (Number(i) + 1 != this.members.length) {
                buf += `<a href="javascript:clickDownCharge(${i});">`;
                buf += `<img class="icon" src="/static/ionicons/arrow-down-outline.svg" title="下に移動">`;
                buf += `</a>`;
              }
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
  }

  removeParent() {
    let buf = '<p class="my-1">';
    buf += `<a href="javascript:clickAddParentInvisible();">`;
    buf +=
      '<img class="icon3" src="/static/ionicons/add-circle-outline.svg" title="親チケットを追加">';
    buf += "</a>";
    buf += '<input type="hidden" id="parent" name="parent" value="">';
    buf += "</p>";
    const parentTicket = document.querySelector<HTMLElement>(`#parentTicket`);
    if (parentTicket) {
      parentTicket.innerHTML = buf;
    }
  }

  removeDeliverable() {
    const selectedIndex =
      document.querySelector<HTMLInputElement>(`#selectedIndex`);
    if (selectedIndex) {
      this.deliverables.splice(Number(selectedIndex.value), 1);
      this.setDeliverablesTable();
    }
  }

  setDeliverablesTable() {
    let buf = "";
    if (this.deliverables.length > 0) {
      buf += `<table class="table table-hover">`;
      {
        buf += `<thead>`;
        {
          buf += `<tr>`;
          {
            buf += `<th scope="col">成果物名</th>`;
            buf += `<th scope="col">ファイルパス / URL</th>`;
            buf += `<th scope="col"></th>`;
          }
          buf += `</tr>`;
        }
        buf += `</thead>`;
        buf += `<tbody>`;
        for (let i in this.deliverables) {
          buf += `<tr>`;
          {
            buf += `<td>`;
            buf += this.escapeHtml(this.deliverables[i].name);
            buf += `</td>`;
            buf += `<td>`;
            if (
              this.deliverables[i].path.startsWith("http://") ||
              this.deliverables[i].path.startsWith("https://")
            ) {
              buf += `<a href="${this.deliverables[i].path}" target="_blank">`;
              buf += this.deliverables[i].path;
              buf += `</a>`;
            } else {
              buf += this.escapeHtml(this.deliverables[i].path);
            }
            buf += `</td>`;
            buf += `<td>`;
            buf += `<a href="javascript:removeDeliverable(${i})">`;
            buf += `<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除"></a>`;
            buf += `</td>`;
          }
          buf += `</tr>`;
        }
        buf += `</tbody>`;
      }
      buf += `</table>`;
    }

    const dt = document.querySelector<HTMLElement>(`div#deliverablesTbl`);
    if (dt) dt.innerHTML = buf;
  }

  selectColor() {
    const selectedColor =
      document.querySelector<HTMLInputElement>(`#selectedColor`);

    if (selectedColor && selectedColor.value) {
      const ticketId = document.querySelector<HTMLInputElement>(`#ticket_id`);
      if (ticketId && ticketId.value) {
        const buf = `ticket_id=${ticketId.value}&color=${selectedColor.value}`;

        fetch("/api/ticketColor", {
          method: "POST",
          headers: {
            Accept: "*/*",
            "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
          },
          mode: "cors",
          body: encodeURI(buf),
        })
          .then((response) => response.json())
          .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
      }

      const color = document.querySelector<HTMLInputElement>(`#color`);
      if (color) {
        const current = document.querySelector<HTMLElement>(
          `#img-${color.value}`
        );
        if (current) current.classList.add("d-none");
        color.value = selectedColor.value;
      }
      const newone = document.querySelector<HTMLElement>(
        `#img-${selectedColor.value}`
      );
      if (newone) newone.classList.remove("d-none");
    }
  }

  save() {
    const members = document.querySelector<HTMLInputElement>(`#members`);
    if (members) {
      members.value = JSON.stringify(this.members);
    }
    const deliverables =
      document.querySelector<HTMLInputElement>(`#deliverables`);
    if (deliverables) {
      deliverables.value = JSON.stringify(this.deliverables);
    }
    const form = document.querySelector<HTMLFormElement>(`#post_ticket`);
    if (form) {
      form.submit();
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
