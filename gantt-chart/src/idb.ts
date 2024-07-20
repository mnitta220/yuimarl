import Dexie from "dexie";

const DB_NAME = "yuimarldb1";

export class AppDatabase extends Dexie {
  projects: Dexie.Table<IProject, string>;
  trees: Dexie.Table<ITree, string>;

  constructor() {
    super(DB_NAME);

    this.version(1).stores({
      projects: "id",
      trees: "id",
    });

    this.projects = this.table("projects");
    this.trees = this.table("trees");
  }
}

export interface IProject {
  id: string; // プロジェクトID
  showDone: boolean; // 完了済みを表示
  delayRed: boolean; // 進捗遅れを赤く表示
}

export class Project implements IProject {
  id: string;
  showDone: boolean;
  delayRed: boolean;

  constructor(id: string, showDone: boolean, delayRed: boolean) {
    this.id = id;
    this.showDone = showDone;
    this.delayRed = delayRed;
  }
}

export interface ITree {
  id: string; // チケットID
  open: boolean; // true:開いている, false:閉じている
  color: string; // 背景色
}

export class Tree implements ITree {
  id: string;
  open: boolean;
  color: string;

  constructor(id: string, open: boolean, color: string) {
    this.id = id;
    this.open = open;
    this.color = color;
  }
}
