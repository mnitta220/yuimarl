import { SCROLL_BAR_WIDTH } from "./common";
import GanttFrame from "./ganttFrame";

// カラムスクロールバー
export default class ColumnScroll {
  id = "colscr";
  pos = 0;

  constructor(private frame: GanttFrame) {}

  // スクロールバーを構築する
  build(frag: DocumentFragment) {
    const bar = document.createElement("div");
    bar.id = this.id;
    bar.className = "scroll-corner";
    bar.style.top = `${this.frame.height - SCROLL_BAR_WIDTH}px`;
    bar.style.left = `${this.pos}px`;
    bar.style.width = `${this.frame.calendarLeft}px`;
    bar.style.height = `${SCROLL_BAR_WIDTH}px`;
    frag.append(bar);
  }

  scrollH(x: number) {
    this.pos = -x;
    const colscr = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (colscr) {
      colscr.style.left = `${this.pos}px`;
    }
  }
}
