import Dexie from "dexie";

const DB_NAME = "yuimarldb1";

export class AppDatabase extends Dexie {
  trees: Dexie.Table<ITree, string>;

  constructor() {
    super(DB_NAME);

    this.version(1).stores({
      trees: "id",
    });

    this.trees = this.table("trees");
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
