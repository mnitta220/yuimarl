import * as bootstrap from "bootstrap";
import ProjectInfo from "./projectInfo";

export default class UpdateMemberModal {
  private id = "updateMemberModal";
  private modal: bootstrap.Modal | null = null;

  constructor(private info: ProjectInfo) {
    this.info = info;
    const memberModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (memberModal) {
      this.modal = new bootstrap.Modal(memberModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const btnUpdateMember =
      document.querySelector<HTMLButtonElement>(`#btnUpdateMember`);
    if (btnUpdateMember) {
      btnUpdateMember.addEventListener("click", () => {
        this.updateMember();
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

  private updateMember() {
    const updateMemberIdx =
      document.querySelector<HTMLInputElement>(`#updateMemberIdx`);
    if (updateMemberIdx) {
      let idx = Number(updateMemberIdx.value);
      const updateMemberRole =
        document.querySelector<HTMLInputElement>(`#updateMemberRole`);
      if (updateMemberRole) {
        this.info.members[idx].role = Number(updateMemberRole.value);
        const memberTbl =
          document.querySelector<HTMLDivElement>(`div#memberTbl`);
        if (memberTbl) {
          memberTbl.innerHTML = "";
        }
      }
      this.hide();
      this.info.setMemberList();
    }
  }
}
