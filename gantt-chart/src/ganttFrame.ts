import dayjs from "dayjs";
import {
  SCROLL_BAR_WIDTH,
  CALENDAR_MIN,
  DAY_WIDTH,
  Column,
  GanttTicket,
} from "./common";
import ColumnHeader from "./columnHeader";
import ColumnBody from "./columnBody";
import ColumnScroll from "./columnScroll";
import CalendarHeader from "./calendarHeader";
import CalendarBody from "./calendarBody";
import CalendarScroll from "./calendarScroll";
import ScrollH from "./scrollH";
import ScrollV from "./scrollV";
import TicketModal from "./ticketModal";
import { AppDatabase, ITree, Tree } from "./idb";

// ガントチャート全体フレーム
export default class GanttFrame {
  id = "ganttframe";
  width = 0;
  height = 0;
  columnHeader = new ColumnHeader(this);
  columnBody = new ColumnBody(this);
  columnScroll = new ColumnScroll(this);
  calendarHeader = new CalendarHeader(this);
  calendarBody = new CalendarBody(this);
  calendarScroll = new CalendarScroll(this);
  ticketModal = new TicketModal();
  sch = new ScrollH(this);
  scv = new ScrollV(this);
  colW = 0;
  calendarLeft = 0;
  calendarStart = dayjs();
  calendarEnd = dayjs();
  calendarTotalWidth = 0;
  ticketsTotalHeight = 0;
  ticketsFrameHeight = 0;
  schThreshold = 0; // 横スクロールバーを表示するしきい値
  posX = 0;
  cols: Column[] = [];
  holidays: dayjs.Dayjs[] = []; // 日本の祝日
  tickets: GanttTicket[] = [];
  treeList: ITree[] = [];
  private _idb?: AppDatabase;

  constructor() {
    this._idb = new AppDatabase();
    const startdate = document.querySelector<HTMLInputElement>(`#startdate`);
    const enddate = document.querySelector<HTMLInputElement>(`#enddate`);
    this.calendarStart = dayjs(`${startdate?.value ?? ""}`);
    this.calendarEnd = dayjs(`${enddate?.value ?? ""}`);

    const holidays = document.querySelector<HTMLInputElement>(`#holidays`);
    if (holidays?.value) {
      this.holidays = holidays.value
        .split(",")
        .map((v) => dayjs(v))
        .filter((v) => v.isValid());
    }

    const ts = document.querySelector<HTMLInputElement>(`#tickets`);
    if (ts?.value) {
      this.tickets = JSON.parse(ts.value);
    }

    this.cols = [
      {
        name: "ID",
        width: 54,
      },
      {
        name: "チケット名",
        width: 320,
      },
      {
        name: "開始日",
        width: 67,
      },
      {
        name: "終了日",
        width: 67,
      },
      {
        name: "進捗",
        width: 36,
      },
    ];

    this.build();
    this.handler();
  }

  async readTreeList() {
    try {
      this.treeList = [];
      if (!this._idb) {
        this._idb = new AppDatabase();
      }
      await this._idb.trees.each((t: Tree) => {
        this.setTreeOpen(t.id, t.open, this.tickets);
        this.treeList.push(t);
      });
    } catch (e) {
      throw Error(`${e}`);
    }
  }

  setTreeOpen(id: string, open: boolean, tickets: GanttTicket[]): boolean {
    for (const t of tickets) {
      if (t.id === id) {
        t.open = open;
        return true;
      }
      const ret = this.setTreeOpen(id, open, t.children);
      if (ret) {
        return true;
      }
    }

    return false;
  }

  // ガントチャートを構築する
  build() {
    const frame = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (!frame) {
      console.error("Failed to get #ganttframe!");
      return;
    }
    this.width = frame.offsetWidth;
    this.height = frame.offsetHeight;
    this.colW = this.columnWidth();
    this.calendarLeft = this.colW + 1;
    this.schThreshold = this.calendarLeft + CALENDAR_MIN; // フレームの幅がこれよりも小さければ横スクロールバーを表示する
    const frag = document.createDocumentFragment();
    this.columnBody.build(frag);
    this.columnHeader.build(frag);
    this.columnScroll.build(frag);
    this.calendarBody.build(frag);
    this.calendarHeader.build(frag);
    this.calendarScroll.build(frag);

    // カレンダーの開始日と終了日の差分から全体の幅を求める
    const diff = this.calendarEnd.diff(this.calendarStart, "day");
    this.calendarTotalWidth = (diff + 1) * DAY_WIDTH;

    frame.append(frag);
  }

  // イベントハンドラを登録する
  handler() {
    this.columnBody.handler();
    this.calendarScroll.handler();
    this.sch.handler();
    this.scv.handler();
    this.ticketModal.handler();
  }

  resize() {
    const flexbox = document.querySelector<HTMLDivElement>(`#flexbox`);
    if (!flexbox) {
      console.error("Failed to get #flexbox!");
      return;
    }
    const ganttbase = document.querySelector<HTMLDivElement>(`#ganttbase`);
    if (!ganttbase) {
      console.error("Failed to get #ganttbase!");
      return;
    }
    ganttbase.style.width = `${flexbox.offsetWidth - SCROLL_BAR_WIDTH}px`;
    const frame = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (!frame) {
      console.error("Failed to get #ganttframe!");
      return;
    }
    this.width = frame.offsetWidth;
    this.height = frame.offsetHeight;
    this.colW = this.columnWidth();
    this.calendarLeft = this.colW + 1;
    this.schThreshold = this.calendarLeft + CALENDAR_MIN;
    this.sch.pos = 0;
    this.calendarScroll.pos = 0;
    this.calendarBody.resize();
    this.calendarHeader.resize();
    this.calendarScroll.resize();
    this.scrollH(0);
  }

  // ガントチャートを表示する
  draw() {
    this.columnBody.draw();
    this.calendarHeader.draw();
    this.calendarBody.draw();
    this.calendarScroll.draw();
    this.sch.draw();
    this.scv.draw();
  }

  scrollH(x: number) {
    this.posX =
      (x * this.schThreshold) /
      (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
    this.columnBody.scrollH(this.posX);
    this.columnHeader.scrollH(this.posX);
    this.columnScroll.scrollH(this.posX);
    this.calendarHeader.scrollH(this.posX);
    this.calendarBody.scrollH(this.posX);
    this.calendarScroll.scrollH(this.posX);
  }

  scrollV(y: number) {
    this.columnBody.scrollV(y);
    this.calendarBody.scrollV(y);
  }

  columnWidth(): number {
    let w = 2;
    for (let col of this.cols) {
      w += col.width;
    }
    return w;
  }

  mouseDownCalScroll(x: number) {
    this.calendarScroll.mouseDownCalScroll(x);
  }

  mouseMoveCalScroll(x: number) {
    this.calendarScroll.mouseMoveCalScroll(x);
  }

  mouseUpCalScroll() {
    this.calendarScroll.mouseUpCalScroll();
  }

  mouseDownScrollH(x: number) {
    this.sch.mouseDownScrollH(x);
  }

  mouseMoveScrollH(x: number) {
    this.sch.mouseMoveScrollH(x);
  }

  mouseUpScrollH() {
    this.sch.mouseUpScrollH();
  }

  mouseDownScrollV(y: number) {
    this.scv.mouseDownScrollV(y);
  }

  mouseMoveScrollV(y: number) {
    this.scv.mouseMoveScrollV(y);
  }

  mouseUpScrollV() {
    this.scv.mouseUpScrollV();
  }

  async nodeOpenClose(id: string, open: boolean) {
    try {
      let tree = new Tree(id, open, "");
      await this._idb?.trees.put(tree);
      await this.readTreeList();
    } catch (e) {
      throw Error(`${e}`);
    }
  }
}
