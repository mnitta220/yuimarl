import {
  SCROLL_BAR_WIDTH,
  CALENDAR_MIN,
  DAY_MILISEC,
  DAY_WIDTH,
  Column,
  Ticket,
} from "./common";
import ColumnHeader from "./columnHeader";
import ColumnBody from "./columnBody";
import ColumnScroll from "./columnScroll";
import CalendarHeader from "./calendarHeader";
import CalendarBody from "./calendarBody";
import CalendarScroll from "./calendarScroll";
import ScrollH from "./scrollH";
import ScrollV from "./scrollV";

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
  sch = new ScrollH(this);
  scv = new ScrollV(this);
  colW = 0;
  calendarLeft = 0;
  calendarStart = new Date(2024, 3, 8);
  calendarEnd = new Date(2024, 7, 31);
  calendarTotalWidth = 0;
  ticketsTotalHeight = 0;
  ticketsFrameHeight = 0;
  schThreshold = 0; // 横スクロールバーを表示するしきい値
  posX = 0;
  cols: Column[] = [];
  holidays: Date[] = []; // 日本の祝日
  tickets: Ticket[] = [];

  constructor() {
    this.holidays = [new Date(2024, 6, 15), new Date(2024, 7, 12)];
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
    this.tickets = [
      {
        id: "YU1",
        idDisp: "YU1",
        name: "Yuimarl開発",
        start: new Date(2024, 3, 8),
        end: null,
        progress: 0,
        open: true,
        children: [
          {
            id: "YU2",
            idDisp: "YU2",
            name: "技術調査",
            start: new Date(2024, 3, 8),
            end: new Date(2024, 3, 17),
            progress: 100,
            open: false,
            children: [
              {
                id: "YU3",
                idDisp: "YU3",
                name: "Firestore",
                start: new Date(2024, 3, 8),
                end: new Date(2024, 5, 13),
                progress: 100,
                open: true,
                children: [],
              },
            ],
          },
          {
            id: "YU8",
            idDisp: "YU8",
            name: "version 1.0.0",
            start: new Date(2024, 3, 18),
            end: new Date(2024, 5, 13),
            progress: 100,
            open: false,
            children: [
              {
                id: "YU8",
                idDisp: "YU8",
                name: "version 1.0.0",
                start: new Date(2024, 3, 18),
                end: new Date(2024, 5, 13),
                progress: 100,
                open: true,
                children: [],
              },
            ],
          },
          {
            id: "YU62",
            idDisp: "YU62",
            name: "version 1.0.12",
            start: new Date(2024, 5, 21),
            end: new Date(2024, 5, 23),
            progress: 100,
            open: false,
            children: [
              {
                id: "YU8",
                idDisp: "YU8",
                name: "version 1.0.0",
                start: new Date(2024, 3, 18),
                end: new Date(2024, 5, 13),
                progress: 100,
                open: true,
                children: [],
              },
            ],
          },
          {
            id: "YU71",
            idDisp: "YU71",
            name: "version 1.0.13",
            start: new Date(2024, 5, 24),
            end: new Date(2024, 6, 30),
            progress: 10,
            open: true,
            children: [
              {
                id: "YU60",
                idDisp: "YU60",
                name: "ガントチャート",
                start: new Date(2024, 5, 24),
                end: new Date(2024, 6, 30),
                progress: 10,
                open: true,
                children: [
                  {
                    id: "YU72",
                    idDisp: "YU72",
                    name: "画面プロトタイプ作成",
                    start: new Date(2024, 5, 24),
                    end: new Date(2024, 6, 12),
                    progress: 80,
                    open: true,
                    children: [],
                  },
                  {
                    id: "YU73",
                    idDisp: "YU73",
                    name: "実装・テスト",
                    start: new Date(2024, 6, 16),
                    end: new Date(2024, 6, 26),
                    progress: 0,
                    open: true,
                    children: [],
                  },
                  {
                    id: "YU74",
                    idDisp: "YU74",
                    name: "ユーザーガイド更新",
                    start: new Date(2024, 6, 29),
                    end: new Date(2024, 6, 30),
                    progress: 0,
                    open: true,
                    children: [],
                  },
                ],
              },
            ],
          },
          {
            id: "YU4",
            idDisp: "YU4",
            name: "バックログ",
            start: null,
            end: null,
            progress: 0,
            open: true,
            children: [
              {
                id: "YU46",
                idDisp: "YU46",
                name: "ページング改善",
                start: null,
                end: null,
                progress: 0,
                open: true,
                children: [],
              },
              {
                id: "YU45",
                idDisp: "YU45",
                name: "オーナー変更",
                start: null,
                end: null,
                progress: 0,
                open: true,
                children: [],
              },
            ],
          },
        ],
      },
    ];
    this.build();
    this.handler();
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
    const diff =
      (this.calendarEnd.getTime() - this.calendarStart.getTime()) / DAY_MILISEC;
    this.calendarTotalWidth = (diff + 1) * DAY_WIDTH;

    frame.append(frag);
  }

  // イベントハンドラを登録する
  handler() {
    this.columnBody.handler();
    this.calendarScroll.handler();
    this.sch.handler();
    this.scv.handler();
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
}
