import * as bootstrap from "bootstrap";
import TicketList from "./ticketList";

interface IMember {
  id: string; // チケットメンバーID
  uid: string; // メンバーのユーザーID
  role: number | null; // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
  seq: number | null; // 表示順
  name: string | null; // メンバーの名前
  email: string | null; // メンバーのメールアドレス
}

interface IMemberResult {
  result: string;
  members: IMember[];
  message: string;
}

export default class ChargeModal {
  private id = "chargeModal";
  private modal: bootstrap.Modal | null = null;
  members: IMember[] = [];

  constructor(private info: TicketList) {
    const memberModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (memberModal) {
      this.modal = new bootstrap.Modal(memberModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const btnAddCharge =
      document.querySelector<HTMLButtonElement>(`#btnAddCharge`);
    if (btnAddCharge) {
      btnAddCharge.addEventListener("click", () => {
        this.clickSelectCharge();
      });
    }

    const btnSelectCharge =
      document.querySelector<HTMLButtonElement>(`#btnSelectCharge`);
    if (btnSelectCharge) {
      btnSelectCharge.addEventListener("click", () => {
        this.selectCharge();
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

  private clickSelectCharge() {
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
      this.members = res.members;
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
        for (let i in this.members) {
          buf += `<tr>`;
          {
            buf += `<td>`;
            {
              buf += `<input class="form-check-input" type="radio" id="charge${i}" name="charge" value="${i}"`;
              if (Number(i) == 0) {
                buf += " checked";
              }
              buf += `>`;
            }
            buf += `</td>`;
            buf += `<td>`;
            buf += this.info.roleToString(this.members[i].role);
            buf += `</td>`;
            buf += `<td>${this.members[i].email}</td>`;
            buf += `<td>${this.members[i].name}</td>`;
            buf += `<td>`;
            {
              buf += `<input type="hidden" id="uid${i}" value="${this.members[i].uid}">`;
              buf += `<input type="hidden" id="email${i}" value="${this.members[i].email}">`;
              buf += `<input type="hidden" id="name${i}" value="${this.members[i].name}">`;
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

  private selectCharge() {
    const check = document.querySelector<HTMLInputElement>(
      `input[name="charge"]:checked`
    );
    if (check) {
      const idx = Number(check.value);
      const member = this.members[idx];
      const sel = document.querySelector<HTMLElement>(`#charge-sel`);
      if (sel)
        sel.innerHTML = `<small>[${member.email}]</small> ${member.name}`;
      const uid = document.querySelector<HTMLInputElement>(`#chargeuid`);
      if (uid) uid.value = member.uid;
      const mail = document.querySelector<HTMLInputElement>(`#chargemail`);
      if (mail) mail.value = member.email ?? "";
      const name = document.querySelector<HTMLInputElement>(`#chargename`);
      if (name) name.value = member.name ?? "";
      const charge2 = document.querySelector<HTMLElement>(`#charge2`);
      if (charge2) charge2.classList.remove("d-none");
      const charge1 = document.querySelector<HTMLElement>(`#charge1`);
      if (charge1) charge1.classList.add("d-none");
      this.hide();
    }
  }
}
