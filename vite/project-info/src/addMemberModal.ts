import * as bootstrap from "bootstrap";
import ProjectInfo, { IMember } from "./projectInfo";

interface IUserSub {
  uid: string;
  email: string;
  name: string;
}

interface IUserResult {
  result: string;
  users: IUserSub[];
  message: string;
}

export default class AddMemberModal {
  private id = "addMemberModal";
  private modal: bootstrap.Modal | null = null;

  constructor(private info: ProjectInfo) {
    const memberModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (memberModal) {
      this.modal = new bootstrap.Modal(memberModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const searchEmail =
      document.querySelector<HTMLButtonElement>(`#search-email`);
    if (searchEmail) {
      searchEmail.addEventListener("click", () => {
        this.searchEmail();
      });
    }

    const searchName =
      document.querySelector<HTMLButtonElement>(`#search-name`);
    if (searchName) {
      searchName.addEventListener("click", () => {
        this.searchName();
      });
    }

    const btnAddMember =
      document.querySelector<HTMLButtonElement>(`#btnAddMember`);
    if (btnAddMember) {
      btnAddMember.addEventListener("click", () => {
        this.addMember();
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

  private searchEmail() {
    const addMembers = document.querySelector<HTMLInputElement>(`#add_members`);
    if (addMembers) {
      addMembers.value = "";
    }

    const inputEmail = document.querySelector<HTMLInputElement>(`input#email`);
    if (inputEmail && inputEmail.value) {
      const buf = `email=${inputEmail.value}`;

      fetch("/api/userByEmail", {
        method: "POST",
        headers: {
          Accept: "*/*",
          "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
        },
        mode: "cors",
        body: encodeURI(buf),
      })
        .then((response) => response.json())
        .then((data: IUserResult) => {
          this.memberSearchResult(data);
        })
        .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
    }
  }

  private searchName() {
    const addMembers = document.querySelector<HTMLInputElement>(`#add_members`);
    if (addMembers) {
      addMembers.value = "";
    }

    const inputName =
      document.querySelector<HTMLInputElement>(`input#member-name`);
    if (inputName && inputName.value) {
      const buf = `name=${inputName.value}`;

      fetch("/api/userByName", {
        method: "POST",
        headers: {
          Accept: "*/*",
          "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
        },
        mode: "cors",
        body: encodeURI(buf),
      })
        .then((response) => response.json())
        .then((data: IUserResult) => {
          this.memberSearchResult(data);
        })
        .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
    }
  }

  private memberSearchResult(res: IUserResult) {
    let buf = "";
    if (res.result === "OK") {
      if (res.users.length > 0) {
        buf = `<table class="table table-hover">`;
        {
          buf += `<thead>`;
          {
            buf += `<tr>`;
            {
              buf += `<th scope="col">選択</th>`;
              buf += `<th scope="col">メールアドレス</th>`;
              buf += `<th scope="col">名前</th>`;
              buf += `<th scope="col">ロール</th>`;
            }
            buf += `</tr>`;
          }
          buf += `</thead>`;
          buf += `<tbody>`;
          for (let i in res.users) {
            buf += `<tr>`;
            {
              buf += `<td>`;
              {
                buf += `<input class="form-check-input" type="checkbox" id="check${i}" checked>`;
              }
              buf += `</td>`;
              buf += `<td>${res.users[i].email}</td>`;
              buf += `<td>${res.users[i].name}</td>`;
              buf += `<td>`;
              {
                buf += `<select class="form-select" id="role${i}" name="role${i}">`;
                {
                  buf += `<option value="2">管理者</option>`;
                  buf += `<option value="3">メンバー</option>`;
                  buf += `<option value="4">閲覧者</option>`;
                }
                buf += `</select>`;
                buf += `<input type="hidden" id="uid${i}" value="${res.users[i].uid}">`;
                buf += `<input type="hidden" id="name${i}" value="${res.users[i].name}">`;
                buf += `<input type="hidden" id="email${i}" value="${res.users[i].email}">`;
              }
              buf += `</td>`;
            }
            buf += `</tr>`;
          }
          buf += `</tbody>`;
        }
        buf += `</table>`;
        const btnAddMember =
          document.querySelector<HTMLButtonElement>(`#btnAddMember`);
        if (btnAddMember) {
          btnAddMember.removeAttribute("disabled");
        }
      } else {
        buf =
          '<div class="col"><p class="text-danger">該当するユーザーが登録されていません。</p></div>';
        const btnAddMember =
          document.querySelector<HTMLButtonElement>(`#btnAddMember`);
        if (btnAddMember) {
          btnAddMember.setAttribute("disabled", "disabled");
        }
      }
    } else {
      buf = res.message;
    }

    const searched = document.querySelector<HTMLDivElement>(`#searched`);
    if (searched) {
      searched.innerHTML = buf;
    }
  }

  private addMember() {
    for (let i = 0; i < 10; i++) {
      const check = document.querySelector<HTMLInputElement>(`#check${i}`);
      if (check) {
        if (!check.checked) {
          continue;
        }
        let member = {
          uid: "",
          email: "",
          name: "",
          role: 0,
        } as IMember;
        const uid = document.querySelector<HTMLInputElement>(`#uid${i}`);
        if (uid) {
          member.uid = uid.value;
        }
        const email = document.querySelector<HTMLInputElement>(`#email${i}`);
        if (email) {
          member.email = email.value;
        }
        const name = document.querySelector<HTMLInputElement>(`#name${i}`);
        if (name) {
          member.name = name.value;
        }
        const role = document.querySelector<HTMLInputElement>(`#role${i}`);
        if (role) {
          member.role = Number(role.value);
        }
        let idx = this.info?.members.findIndex((x) => x.uid == member.uid);
        if (idx == -1) {
          this.info.members.push(member);
        }
      } else {
        break;
      }
    }

    document.querySelector<HTMLInputElement>(`input#email`)!.value = "";
    document.querySelector<HTMLInputElement>(`input#member-name`)!.value = "";
    document.querySelector<HTMLDivElement>(`#searched`)!.innerHTML = "";

    this.hide();
    this.info.setMemberList();
  }
}
