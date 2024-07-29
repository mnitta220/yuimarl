import AddChargeModal from "./addChargeModal";
import UpdateMemberModal from "./updateMemberModal";

export interface IMember {
  id: string; // チケットメンバーID
  //ticket_id: string; // チケットID
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
  updateMemberModal = new UpdateMemberModal(this);

  constructor() {
    this.handler();
  }

  // イベントハンドラを登録する
  private handler() {
    /*
    const icnAddCharge =
      document.querySelector<HTMLButtonElement>(`#icnAddCharge`);
    if (icnAddCharge) {
      icnAddCharge.addEventListener("click", () => {
        console.log("icnAddCharge click");
        this.addChargeModal.show();
      });
    }
    */

    this.addChargeModal.handler();
    /*
    const btnProjectDel =
      document.querySelector<HTMLButtonElement>(`#btnProjectDel`);
    if (btnProjectDel) {
      btnProjectDel.addEventListener("click", () => {
        this.deleteProject();
      });
    }

    const btnWithdraw =
      document.querySelector<HTMLButtonElement>(`#btnWithdraw`);
    if (btnWithdraw) {
      btnWithdraw.addEventListener("click", () => {
        this.withdraw();
      });
    }

    this.updateMemberModal.handler();
    */
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
    //this.memberHandler();
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
            buf += `<a href="javascript:removeCharge(${i})">`;
            buf += `<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除"></a>`;
            if (Number(i) != 0) {
              buf += `&nbsp;<a href="javascript:chargeSeqUp(${i})">`;
              buf += `<img class="icon" src="/static/ionicons/arrow-up-outline.svg" title="上に移動"></a>`;
            }
            if (Number(i) + 1 != this.members.length) {
              buf += `&nbsp;<a href="javascript:chargeSeqDown(${i})">`;
              buf += `<img class="icon" src="/static/ionicons/arrow-down-outline.svg" title="下に移動"></a>`;
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

  /*
  private memberHandler() {
    for (let i in this.members) {
      let icnSetMem = document.querySelector<HTMLImageElement>(
        `#icnSetMem${i}`
      );
      if (icnSetMem) {
        icnSetMem.addEventListener("click", () => {
          this.updateMember(Number(i));
        });
      }

      let icnRemoveMem = document.querySelector<HTMLImageElement>(
        `#icnRemoveMem${i}`
      );
      if (icnRemoveMem) {
        icnRemoveMem.addEventListener("click", () => {
          this.removeMember(Number(i));
        });
      }
    }
  }

  private roleToString(role: number | null) {
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
    return "Unknown";
  }

  setMemberList() {
    const ms = document.querySelector<HTMLInputElement>(`#members`);
    if (ms) {
      ms.value = JSON.stringify(this.members);
    }
    let buf = "";
    for (let i in this.members) {
      buf += `<tr>`;
      {
        buf += `<td>${this.roleToString(this.members[i].role)}</td>`;
        buf += `<td>${this.members[i].email}</td>`;
        buf += `<td>${this.members[i].name}</td>`;
        buf += `<td>`;
        if (Number(i) > 0) {
          buf += `<img class="icon" style="cursor:pointer" id="icnSetMem${i}" src="/static/ionicons/settings-outline.svg" title="設定">&nbsp;`;
          buf += `<img class="icon" style="cursor:pointer" id="icnRemoveMem${i}" src="/static/ionicons/remove-circle-outline.svg" title="削除">`;
        }
        buf += `</td>`;
      }
      buf += `</tr>`;
    }

    const tbody = document.querySelector<HTMLElement>(`#members-tbody`);
    if (tbody) {
      tbody.innerHTML = buf;
    }

    this.memberHandler();

    const limit = document.querySelector<HTMLInputElement>(`#member_limit`);
    if (limit) {
      let l = Number(limit.value);
      if (this.members.length < l) {
        const divAddMember =
          document.querySelector<HTMLInputElement>(`#divAddMember`);
        if (divAddMember) {
          divAddMember.classList.remove("d-none");
        }
      } else {
        const divAddMember =
          document.querySelector<HTMLInputElement>(`#divAddMember`);
        if (divAddMember) {
          divAddMember.classList.add("d-none");
        }
      }
    }
  }

  updateMember(i: number) {
    let buf = `<table class="table table-hover">`;
    {
      buf += `<thead>`;
      {
        buf += `<tr>`;
        {
          buf += `<th scope="col">メールアドレス</th>`;
          buf += `<th scope="col">名前</th>`;
          buf += `<th scope="col">ロール</th>`;
        }
        buf += `</tr>`;
      }
      buf += `</thead>`;
      buf += `<tbody>`;
      {
        buf += `<tr>`;
        {
          buf += `<td>${this.members[i].email}</td>`;
          buf += `<td>${this.members[i].name}</td>`;
          buf += `<td>`;
          {
            buf += `<select class="form-select" id="updateMemberRole" name="updateMemberRole">`;
            {
              buf += `<option value="2"`;
              if (this.members[i].role == 2) {
                buf += ` selected`;
              }
              buf += `>管理者</option>`;
              buf += `<option value="3"`;
              if (this.members[i].role == 3) {
                buf += ` selected`;
              }
              buf += `>メンバー</option>`;
              buf += `<option value="4"`;
              if (this.members[i].role == 4) {
                buf += ` selected`;
              }
              buf += `>閲覧者</option>`;
            }
            buf += `</select>`;
          }
          buf += `</td>`;
        }
        buf += `</tr>`;
      }
      buf += `</tbody>`;
    }
    buf += `</table>`;
    buf += `<input type="hidden" id="updateMemberIdx" value="${i}">`;

    const memberTbl = document.querySelector<HTMLDivElement>(`div#memberTbl`);
    if (memberTbl) {
      memberTbl.innerHTML = buf;
    }

    this.updateMemberModal.show();
  }

  private removeMember(i: number) {
    this.members.splice(i, 1);
    this.setMemberList();
  }

  private deleteProject() {
    const action = document.querySelector<HTMLInputElement>(`#action`);
    if (action) {
      action.value = "Delete";
      const form = document.querySelector<HTMLFormElement>(`#post_project`);
      if (form) {
        form.submit();
      }
    }
  }

  private withdraw() {
    const action = document.querySelector<HTMLInputElement>(`#action`);
    if (action) {
      action.value = "Withdraw";
      const form = document.querySelector<HTMLFormElement>(`#post_project`);
      if (form) {
        form.submit();
      }
    }
  }
  */
}
