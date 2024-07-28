import MemberModal from "./memberModal";

interface IMember {
  id: string; // プロジェクトID
  uid: string; // メンバーのユーザーID
  role: number | null; // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
  name: string | null; // メンバーの名前
  email: string | null; // メンバーのメールアドレス
}

export default class ProjectInfo {
  members: IMember[] = [];
  memberModal = new MemberModal();

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
}
