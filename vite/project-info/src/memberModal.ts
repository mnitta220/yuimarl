import * as bootstrap from "bootstrap";
//import { TicketModalResult } from "./common";

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

export default class MemberModal {
  private id = "addMemberModal";
  private modal: bootstrap.Modal | null = null;
  //private ticketId: string | null = null;

  constructor() {
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
      // 更新ボタンが押された
      searchEmail.addEventListener("click", () => {
        //console.log("searchEmail");
        this.searchEmail();
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
    console.log("searchEmail");
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
        .then((response) => {
          return response.json();
        })
        .then((data: IUserResult) => {
          console.log(JSON.stringify(data));
          this.memberSearchResult(data);
          /*
          if (data.result) {
            const projectid =
              document.querySelector<HTMLInputElement>(`#projectId`);
            if (projectid) {
              window.location.href = `/project?id=${projectid.value}&tab=gantt`;
            }
          } else {
            window.alert(`エラーが発生しました。: ${data.message}`);
          }
          */
        })
        .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
    }
  }

  private memberSearchResult(ret: IUserResult) {
    let buf = "";
    if (ret.result === "OK") {
      if (ret.users.length > 0) {
        buf += `<table class="table table-hover">`;
        {
          buf += `<thead>`;
          {
            buf += `<tr>`;
            buf += `<th scope="col">選択</th>`;
            buf += `<th scope="col">メールアドレス</th>`;
            buf += `<th scope="col">名前</th>`;
            buf += `<th scope="col">ロール</th>`;
            buf += `</tr>`;
          }
          buf += `</thead>`;
          buf += `<tbody>`;
          for (let i in ret.users) {
            buf += `<tr>`;
            {
              buf += `<td>`;
              {
                buf += `<input class="form-check-input" type="checkbox" id="check${i}" checked>`;
              }
              buf += `</td>`;
              buf += `<td>${ret.users[i].email}</td>`;
              buf += `<td>${ret.users[i].name}</td>`;
              buf += `<td>`;
              {
                buf += `<select class="form-select" id="role${i}" name="role${i}">`;
                {
                  buf += `<option value="2">管理者</option>`;
                  buf += `<option value="3">メンバー</option>`;
                  buf += `<option value="4">閲覧者</option>`;
                }
                buf += `</select>`;
                buf += `<input type="hidden" id="uid${i}" value="${ret.users[i].uid}">`;
                buf += `<input type="hidden" id="name${i}" value="${ret.users[i].name}">`;
                buf += `<input type="hidden" id="email${i}" value="${ret.users[i].email}">`;
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
        buf +=
          '<div class="col"><p class="text-danger">該当するユーザーが登録されていません。</p></div>';
        const btnAddMember =
          document.querySelector<HTMLButtonElement>(`#btnAddMember`);
        if (btnAddMember) {
          btnAddMember.setAttribute("disabled", "disabled");
        }
      }
    } else {
      buf += ret.message;
    }

    const searched = document.querySelector<HTMLDivElement>(`#searched`);
    if (searched) {
      searched.innerHTML = buf;
    }
  }
}
