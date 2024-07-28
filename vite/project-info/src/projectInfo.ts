interface IMember {
  id: string; // プロジェクトID
  uid: string; // メンバーのユーザーID
  role: number | null; // ロール 1:オーナー, 2:管理者, 3:メンバー, 4:閲覧者
  name: string | null; // メンバーの名前
  email: string | null; // メンバーのメールアドレス
}

export default class ProjectInfo {
  members: IMember[] = [];

  load() {
    this.members = [];
    const ms = document.querySelector<HTMLInputElement>(`#members`);
    if (ms?.value) {
      this.members = JSON.parse(ms.value);
    }
  }
}
