import * as bootstrap from "bootstrap";
import TicketInfo, { IMember } from "./ticketInfo";

interface IMemberResult {
  result: string;
  members: IMember[];
  message: string;
}

export default class AddChargeModal {
  private id = "addChargeModal";
  private modal: bootstrap.Modal | null = null;

  constructor(private info: TicketInfo) {
    const memberModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (memberModal) {
      this.modal = new bootstrap.Modal(memberModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const icnAddCharge =
      document.querySelector<HTMLButtonElement>(`#icnAddCharge`);
    if (icnAddCharge) {
      icnAddCharge.addEventListener("click", () => {
        this.clickAddCharge();
      });
    }

    const btnMemberAdd =
      document.querySelector<HTMLButtonElement>(`#btnMemberAdd`);
    if (btnMemberAdd) {
      btnMemberAdd.addEventListener("click", () => {
        this.addCharge();
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

  private clickAddCharge() {
    const projectId =
      document.querySelector<HTMLInputElement>(`input#project_id`);
    if (projectId && projectId.value) {
      const buf = `project_id=${projectId.value}`;

      fetch("/api/projectMember", {
        method: "POST",
        headers: {
          Accept: "*/*",
          "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
        },
        mode: "cors",
        body: encodeURI(buf),
      })
        .then((response) => response.json())
        .then((data: IMemberResult) => {
          this.projectMemberResult(data);
        })
        .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
    }
  }

  private projectMemberResult(res: IMemberResult) {
    let buf = "";
    if (res.result === "OK") {
      buf += `<table class="table table-hover">`;
      {
        buf += `<thead>`;
        {
          buf += `<tr>`;
          {
            buf += `<th scope="col">選択</th>`;
            buf += `<th scope="col">ロール</th>`;
            buf += `<th scope="col">メールアドレス</th>`;
            buf += `<th scope="col">名前</th>`;
            buf += `<th scope="col"></th>`;
          }
          buf += `</tr>`;
        }
        buf += `</thead>`;
        buf += `<tbody>`;
        for (let i in res.members) {
          buf += `<tr>`;
          {
            buf += `<td>`;
            {
              buf += `<input class="form-check-input" type="checkbox" id="check${i}"`;
              for (let j in this.info.members) {
                if (this.info.members[j].uid == res.members[i].uid) {
                  buf += ` checked`;
                  break;
                }
              }
              buf += `>`;
            }
            buf += `</td>`;
            buf += `<td>`;
            buf += this.info.roleToString(res.members[i].role);
            buf += `</td>`;
            buf += `<td>${res.members[i].email}</td>`;
            buf += `<td>${res.members[i].name}</td>`;
            buf += `<td>`;
            {
              buf += `<input type="hidden" id="uid${i}" value="${res.members[i].uid}">`;
              buf += `<input type="hidden" id="email${i}" value="${res.members[i].email}">`;
              buf += `<input type="hidden" id="name${i}" value="${res.members[i].name}">`;
            }
            buf += "</td>";
          }
          buf += "</tr>";
        }
        buf += "</tbody>";
      }
      buf += "</table>";
    } else {
      buf = res.message;
    }

    const searched = document.querySelector<HTMLDivElement>(`#searched`);
    if (searched) {
      searched.innerHTML = buf;
      this.show();
    }
  }

  private addCharge() {
    for (let i = 0; i < 1000; i++) {
      const check = document.querySelector<HTMLInputElement>(`#check${i}`);
      if (check) {
        if (check.checked) {
          let member = {
            uid: "",
            email: "",
            name: "",
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
          let idx = this.info?.members.findIndex((x) => x.uid == member.uid);
          if (idx < 0) {
            this.info.members.push(member);
          } else {
            this.info.members[idx] = member;
          }
        } else {
          const uid = document.querySelector<HTMLInputElement>(`#uid${i}`);
          if (uid) {
            let idx = this.info.members.findIndex((x) => x.uid == uid.value);
            if (idx >= 0) {
              this.info.members.splice(idx, 1);
            }
          }
        }
      } else {
        break;
      }
    }

    this.info.setChargeTable();
    this.hide();
  }
}
