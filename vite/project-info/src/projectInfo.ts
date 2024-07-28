import MemberModal from "./memberModal";

export interface IMember {
  id: string; // プロジェクトID
  uid: string; // メンバーのユーザーID
  role: number | null; // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
  name: string | null; // メンバーの名前
  email: string | null; // メンバーのメールアドレス
}

export default class ProjectInfo {
  members: IMember[] = [];
  memberModal = new MemberModal(this);

  constructor() {
    this.handler();
  }

  // イベントハンドラを登録する
  private handler() {
    const icnAddMember =
      document.querySelector<HTMLButtonElement>(`#icnAddMember`);
    if (icnAddMember) {
      // メンバーを追加アイコンが押された
      icnAddMember.addEventListener("click", () => {
        //console.log("icnAddMember");
        this.memberModal.show();
      });
    }
    this.memberModal.handler();
  }

  load() {
    this.members = [];
    const ms = document.querySelector<HTMLInputElement>(`#members`);
    if (ms?.value) {
      this.members = JSON.parse(ms.value);
    }
  }

  roleToString(role: number | null) {
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
          buf += `<a href="javascript:updateMember(${i})">`;
          buf += `<img class="icon" src="/static/ionicons/settings-outline.svg" title="設定">`;
          buf += `</a>&nbsp;<a href="javascript:removeMember(${i})">`;
          buf += `<img class="icon" src="/static/ionicons/remove-circle-outline.svg" title="削除">`;
          buf += `</a>`;
        }
        buf += `</td>`;
      }
      buf += `</tr>`;
    }

    const tbody = document.querySelector<HTMLElement>(`#members-tbody`);
    if (tbody) {
      tbody.innerHTML = buf;
    }

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
}
