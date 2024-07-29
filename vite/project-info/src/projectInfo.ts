import AddMemberModal from "./addMemberModal";
import UpdateMemberModal from "./updateMemberModal";

export interface IMember {
  id: string; // プロジェクトID
  uid: string; // メンバーのユーザーID
  role: number | null; // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
  name: string | null; // メンバーの名前
  email: string | null; // メンバーのメールアドレス
}

export default class ProjectInfo {
  members: IMember[] = [];
  addMemberModal = new AddMemberModal(this);
  updateMemberModal = new UpdateMemberModal(this);

  constructor() {
    this.handler();
  }

  // イベントハンドラを登録する
  private handler() {
    const icnAddMember =
      document.querySelector<HTMLButtonElement>(`#icnAddMember`);
    if (icnAddMember) {
      icnAddMember.addEventListener("click", () => {
        this.addMemberModal.show();
      });
    }

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

    this.addMemberModal.handler();
    this.updateMemberModal.handler();
  }

  load() {
    this.members = [];
    const ms = document.querySelector<HTMLInputElement>(`#members`);
    if (ms?.value) {
      this.members = JSON.parse(ms.value);
    }
    this.memberHandler();
  }

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
}
