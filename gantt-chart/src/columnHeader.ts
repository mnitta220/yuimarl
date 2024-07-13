import { HEADER_HEIGHT, HEADER_LABEL_Y } from "./common";
import GanttFrame from "./ganttFrame";

// カラムヘッダー
export default class ColumnHeader {
  id = "colhead";
  pos = 0;

  constructor(private frame: GanttFrame) {}

  // カラムヘッダーを構築する
  build(frag: DocumentFragment) {
    const hd = document.createElement("div");
    hd.id = this.id;
    hd.className = "header";
    hd.style.top = `0px`;
    hd.style.left = `${this.pos}px`;
    hd.style.width = `${this.frame.calendarLeft}px`;
    hd.style.height = `${HEADER_HEIGHT}px`;
    frag.append(hd);

    let line = document.createElement("div");
    line.className = "line";
    line.style.top = `${HEADER_HEIGHT}px`;
    line.style.left = "0px";
    line.style.width = `${this.frame.calendarLeft}px`;
    line.style.height = "1px";
    frag.append(line);

    let x = 0;
    for (let col of this.frame.cols) {
      // カラムラベル
      let label = document.createElement("div");
      label.className = "label";
      label.style.top = `${HEADER_LABEL_Y}px`;
      label.style.left = `${x}px`;
      label.style.width = `${col.width}px`;
      label.textContent = col.name;
      hd.append(label);
      x += col.width;

      // カラム区切り線
      line = document.createElement("div");
      line.className = "line";
      line.style.top = "0px";
      line.style.left = `${x}px`;
      line.style.width = "1px";
      line.style.height = `${HEADER_HEIGHT}px`;
      hd.append(line);
    }

    // カレンダー境界線
    x += 2;
    line = document.createElement("div");
    line.className = "line";
    line.style.top = "0px";
    line.style.left = `${x}px`;
    line.style.width = "1px";
    line.style.height = `${HEADER_HEIGHT}px`;
    hd.append(line);
  }

  scrollH(x: number) {
    this.pos = -x;
    const colhead = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (colhead) {
      colhead.style.left = `${this.pos}px`;
    }
  }
}
