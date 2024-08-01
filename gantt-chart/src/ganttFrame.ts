import dayjs from "dayjs";
import {
  SCROLL_BAR_WIDTH,
  CALENDAR_MIN,
  LINE_HEIGHT,
  DAY_WIDTH,
  DAY_MILISEC,
  Column,
  GanttTicket,
  GanttRow,
  TICKET_HEIGHT,
  GanttSaveResult,
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
import { AppDatabase, Project, ITree, Tree } from "./idb";

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
  ganttRow = new GanttRow();
  sch = new ScrollH(this);
  scv = new ScrollV(this);
  useIteration = false;
  iterationStart: dayjs.Dayjs | null = null;
  iterationNo = 1;
  iterationUnit = "w1";
  headerHeight = 0;
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
  lines: GanttTicket[] = [];
  treeList: ITree[] = [];
  projectId: string | null = null;
  showDone = false;
  delayRed = false;
  startY = 0;
  startX = 0;
  diffX = 0;
  diffY = 0;
  clicked = false;
  moving = false;
  movingTicket: GanttTicket | null = null;
  movingLine = -1;
  hoverLine = -1;
  dropPos = "";
  private _idb?: AppDatabase;

  constructor() {
    this._idb = new AppDatabase();

    const projectid = document.querySelector<HTMLInputElement>(`#projectId`);
    if (projectid) {
      this.projectId = projectid.value ?? null;
    }

    const startdate = document.querySelector<HTMLInputElement>(`#startdate`);
    if (startdate) {
      this.calendarStart = dayjs(`${startdate.value ?? ""}`);
    }

    const enddate = document.querySelector<HTMLInputElement>(`#enddate`);
    if (enddate) {
      this.calendarEnd = dayjs(`${enddate.value ?? ""}`);
    }

    const useIteration =
      document.querySelector<HTMLInputElement>(`#useIteration`);
    if (useIteration) {
      this.useIteration = useIteration.value === "true" ? true : false;
    }
    this.headerHeight =
      (this.useIteration ? LINE_HEIGHT * 3 : LINE_HEIGHT * 2) + 1;
    const sv1 = document.querySelector<HTMLDivElement>(`#sv1`);
    if (sv1) {
      sv1.style.height = `${this.useIteration ? 64 : 43}px`;
    }

    const iterationStart =
      document.querySelector<HTMLInputElement>(`#iterationStart`);
    if (iterationStart) {
      this.iterationStart = dayjs(iterationStart.value);
    }

    const iterationNo =
      document.querySelector<HTMLInputElement>(`#iterationNo`);
    if (iterationNo) {
      this.iterationNo = Number(iterationNo.value);
    }

    const iterationUnit =
      document.querySelector<HTMLInputElement>(`#iterationUnit`);
    if (iterationUnit) {
      this.iterationUnit = iterationUnit.value;
    }

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
        width: 56,
      },
      {
        name: "チケット名",
        width: 320,
      },
      {
        name: "開始日",
        width: 72,
      },
      {
        name: "終了日",
        width: 72,
      },
      {
        name: "進捗",
        width: 36,
      },
    ];

    this.build();
    this.handler();
  }

  resetLines(cursor: string = "") {
    this.lines = [];
    this.setPos(this.tickets, "", 0, -1, cursor);
    this.ticketsTotalHeight = this.lines.length * TICKET_HEIGHT;
  }

  getNow(): dayjs.Dayjs {
    return dayjs();
    // デバッグ用の日時を設定
    //return dayjs("2024/07/17 15:00:00.000+09:00");
  }

  initDtPos() {
    let dtpos = Math.floor(
      (this.getNow().diff(this.calendarStart) / DAY_MILISEC) * DAY_WIDTH -
        this.calendarHeader.width / 2
    );
    let barpos = Math.floor(
      (dtpos *
        (this.calendarHeader.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH)) /
        this.calendarTotalWidth
    );
    this.calendarHeader.dtpos = dtpos;
    this.calendarBody.dtpos = dtpos;
    this.calendarScroll.barpos = barpos;
  }

  private setPos(
    ts: GanttTicket[],
    pos: string,
    level: number,
    line: number,
    cursor: string
  ): number {
    let i = 0;
    let l = line;
    for (const t of ts) {
      if (t.progress === 100 && this.showDone === false) {
        continue;
      }

      t.pos = pos.length === 0 ? `${i}` : `${pos},${i}`;
      i++;
      l++;
      t.level = level;
      t.line = l;
      t.y1 = l * TICKET_HEIGHT;
      t.y2 = (l + 1) * TICKET_HEIGHT;
      this.lines.push(t);

      if (t.id === cursor) {
        this.ganttRow.y1 = t.y1;
        this.ganttRow.y2 = t.y2;
      } else if (!t.moving && t.open && t.children.length > 0) {
        l = this.setPos(t.children, t.pos, level + 1, l, cursor);
      }
    }
    return l;
  }

  async readProject() {
    try {
      if (!this.projectId) {
        return;
      }
      if (!this._idb) {
        this._idb = new AppDatabase();
      }
      let project = await this._idb.projects.get(this.projectId);
      if (project) {
        this.showDone = project.showDone;
        this.delayRed = project.delayRed;
        const showdone = document.querySelector<HTMLInputElement>(`#showdone`);
        if (showdone) {
          showdone.checked = this.showDone;
        }
        const delayred = document.querySelector<HTMLInputElement>(`#delayred`);
        if (delayred) {
          delayred.checked = this.delayRed;
        }
      }
    } catch (e) {
      throw Error(`${e}`);
    }
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

    const btnSave = document.querySelector<HTMLButtonElement>(`#btnSave`);
    if (btnSave) {
      // 更新ボタンが押された
      btnSave.addEventListener("click", () => {
        this.save();
      });
    }
  }

  save() {
    const loading = document.querySelector<HTMLDivElement>(`#loading`);
    if (loading) {
      loading.classList.remove("d-none");
    }

    const buf = `project_id=${
      this.projectId ?? ""
    }&tickets=${encodeURIComponent(JSON.stringify(this.tickets))}`;

    fetch("/api/ganttUpdate", {
      method: "POST",
      headers: {
        Accept: "*/*",
        "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
      },
      mode: "cors",
      body: encodeURI(buf),
    })
      .then((response) => {
        if (loading) {
          loading.classList.add("d-none");
        }
        return response.json();
      })
      .then((data: GanttSaveResult) => {
        if (data.result) {
          const projectid =
            document.querySelector<HTMLInputElement>(`#projectId`);
          if (projectid) {
            window.location.href = `/project?id=${projectid.value}&tab=gantt`;
          }
        } else {
          window.alert(`エラーが発生しました。: ${data.message}`);
        }
      })
      .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));

    /*
    $.ajax({
      type: "POST",
      url: "/api/ganttUpdate",
      data: {
        project_id: this.projectId ?? "",
        tickets: JSON.stringify(this.tickets),
      },
      success: function (result) {
        $("#loading").addClass("d-none");
        var ret = JSON.parse(result);
        if (ret.result) {
          const projectid =
            document.querySelector<HTMLInputElement>(`#projectId`);
          if (projectid) {
            //window.location.href = `/project?id=${projectid.value}&tab=gantt`;
          }
        } else {
          window.alert(`エラーが発生しました。: ${ret.message}`);
        }
      },
      error: function (result) {
        $("#loading").addClass("d-none");
        console.error(result);
        window.alert(`エラーが発生しました。: ${result}`);
      },
    });
    */

    /*let data = new FormData();
    //data.append("project_id", this.projectId ?? "");
    data.append("email", "aaa");
    //data.append("tickets", JSON.stringify(this.tickets));
    //data.append("tickets", "bbb");
    fetch("/api/userByEmail", {
      method: "POST",
      body: data,
    })
      .then((response) => response.json())
      .then((data: GanttSaveResult) => {
        console.log(JSON.stringify(data));
        if (data.result) {
          //this.frame.ticketModal.show(data);
        } else {
          window.alert(`エラーが発生しました。: ${data.message}`);
        }
      })
      .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
      */
    /*
    data.append("project_id", "aaa");
    //data.append("tickets", JSON.stringify(this.tickets));
    data.append("tickets", "bbb");
    fetch("/api/ganttUpdate", {
      method: "POST",
      body: data,
    })
      .then((response) => response.json())
      .then((data: GanttSaveResult) => {
        console.log(JSON.stringify(data));
        if (data.result) {
          //this.frame.ticketModal.show(data);
        } else {
          window.alert(`エラーが発生しました。: ${data.message}`);
        }
      })
      .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
      */
    /*
    let data = {
      project_id: this.projectId ?? "",
      tickets: "hello", // JSON.stringify(this.tickets),
    };
    fetch("/api/ganttUpdate", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      mode: "no-cors",
      body: JSON.stringify(data),
    })
      .then((response) => response.json())
      .then((data: GanttSaveResult) => {
        console.log(JSON.stringify(data));
        if (data.result) {
          //this.frame.ticketModal.show(data);
        } else {
          window.alert(`エラーが発生しました。: ${data.message}`);
        }
      })
      .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
      */

    /*
    const tickets = document.querySelector<HTMLInputElement>(`#tickets`);
    if (tickets) {
      tickets.value = JSON.stringify(this.tickets);
      const ganttform = document.querySelector<HTMLFormElement>(`#ganttform`);
      if (ganttform) {
        ganttform.submit();
      }
    }
    */
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
    this.initDtPos();
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
    this.posX = Math.floor(
      (x * this.schThreshold) /
        (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH)
    );
    this.columnBody.scrollH(this.posX);
    this.columnHeader.scrollH(this.posX);
    this.columnScroll.scrollH(this.posX);
    this.calendarHeader.scrollH(this.posX);
    this.calendarBody.scrollH(this.posX);
    this.calendarScroll.scrollH(this.posX);
  }

  scrollV(y: number) {
    let y2 = Math.floor(y);
    this.columnBody.scrollV(y2);
    this.calendarBody.scrollV(y2);
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
      this.resetLines();
      this.draw();
    } catch (e) {
      throw Error(`${e}`);
    }
  }

  async setShowDone(show: boolean) {
    this.showDone = show;
    try {
      if (!this.projectId) return;
      this.resetLines();
      this.ganttRow.y1 = -1;
      this.ganttRow.y2 = -1;
      let project = new Project(this.projectId, show, this.delayRed);
      await this._idb?.projects.put(project);
      this.scv.pos = 0;
      this.scrollV(0);
      this.draw();
    } catch (e) {
      throw Error(`${e}`);
    }
  }

  async setDelayRed(red: boolean) {
    this.delayRed = red;
    try {
      if (!this.projectId) return;
      let project = new Project(this.projectId, this.showDone, red);
      await this._idb?.projects.put(project);
      this.draw();
    } catch (e) {
      throw Error(`${e}`);
    }
  }
}
