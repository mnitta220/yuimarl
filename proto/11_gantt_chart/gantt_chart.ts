// カラム
class Column {
  name: string = "";
  width: number = 0;
}

// カラム定義
let cols = [
  {
    name: "ID",
    width: 70,
  },
  {
    name: "チケット名",
    width: 300,
  },
  {
    name: "開始日",
    width: 100,
  },
  {
    name: "終了日",
    width: 100,
  },
  {
    name: "進捗率",
    width: 70,
  },
] as Column[];

const SCROLL_BAR_WIDTH = 16;
const HEADER_LABEL_Y = 46;
const LINE_HEIGHT = 22;
const HEADER_HEIGHT = 66;

// カラムヘッダー
class ColumnHeader {
  x1 = 0;
  //y1 = 0;
  //x2 = 0;
  //y2 = 0;

  // カラムヘッダーを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    //const headerHeight = LINE_HEIGHT * 3;
    let hd = document.createElement("div");
    hd.className = "header";
    hd.style.top = `0px`;
    hd.style.left = `${this.x1}px`;
    hd.style.width = `${frame.colW}px`;
    hd.style.height = `${HEADER_HEIGHT}px`;
    frag.append(hd);

    let line = document.createElement("div");
    line.className = "line";
    line.style.top = `${HEADER_HEIGHT}px`;
    line.style.left = "0px";
    line.style.width = "100%";
    line.style.height = "1px";
    frag.append(line);

    let x = 0;
    for (let col of cols) {
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
    let calendarX = x;
    line = document.createElement("div");
    line.className = "line";
    line.style.top = "0px";
    line.style.left = `${calendarX}px`;
    line.style.width = "1px";
    line.style.height = `${HEADER_HEIGHT}px`;
    frag.append(line);
  }
}

// カラムボディ
class ColumnBody {
  // カラムボディを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    let bar = document.createElement("div");
    const height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    bar.className = "gantt-body";
    bar.style.top = `${HEADER_HEIGHT}px`;
    bar.style.left = `0px`;
    bar.style.width = `${frame.calendarLeft}px`;
    bar.style.height = `${height}px`;
    frag.append(bar);

    let x = 0;
    for (let col of cols) {
      x += col.width;
      // カラム区切り線
      let line = document.createElement("div");
      line.className = "line";
      line.style.top = "0px";
      line.style.left = `${x}px`;
      line.style.width = "1px";
      line.style.height = `${height}px`;
      bar.append(line);
    }

    // カレンダー境界線
    x += 2;
    //let calendarX = x;
    let line = document.createElement("div");
    line.className = "line";
    line.style.top = "0px";
    line.style.left = `${x}px`;
    line.style.width = "1px";
    line.style.height = `${height}px`;
    bar.append(line);
  }
}

// カラムスクロールバー
class ColumnScroll {
  // スクロールバーを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    let bar = document.createElement("div");
    bar.className = "scroll-corner";
    bar.style.top = `${frame.height - SCROLL_BAR_WIDTH}px`;
    bar.style.left = `0px`;
    bar.style.width = `${frame.calendarLeft}px`;
    bar.style.height = `${SCROLL_BAR_WIDTH}px`;
    frag.append(bar);
  }
}

// カレンダーヘッダー
class CalendarHeader {
  x1 = 0;

  // カレンダーヘッダーを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    let hd = document.createElement("div");
    const width = frame.width - frame.calendarLeft;
    hd.className = "header";
    hd.style.top = `0px`;
    hd.style.left = `${frame.calendarLeft}px`;
    hd.style.width = `${width}px`;
    hd.style.height = `${HEADER_HEIGHT}px`;
    frag.append(hd);

    let line = document.createElement("div");
    line.className = "line";
    line.style.top = `${HEADER_HEIGHT}px`;
    line.style.left = `${frame.calendarLeft}px`;
    line.style.width = `${width}px`;
    line.style.height = "1px";
    frag.append(line);

    // カレンダーヘッダー横線1
    line = document.createElement("div");
    line.className = "line";
    line.style.top = `${LINE_HEIGHT}px`;
    line.style.left = `${frame.calendarLeft}px`;
    line.style.width = `${width}px`;
    line.style.height = "1px";
    frag.append(line);

    // カレンダーヘッダー横線2
    line = document.createElement("div");
    line.className = "line";
    line.style.top = `${LINE_HEIGHT + LINE_HEIGHT}px`;
    line.style.left = `${frame.calendarLeft}px`;
    line.style.width = `${width}px`;
    line.style.height = "1px";
    frag.append(line);

    let dt = new Date(frame.calendarStart.getTime()); //this.start;
    let x = 0;
    let dtTop = LINE_HEIGHT + LINE_HEIGHT;
    //console.log(dt.toLocaleDateString());
    while (dt.getTime() <= frame.calendarEnd.getTime()) {
      // カラムラベル
      let label = document.createElement("div");
      label.className = "caldt";
      label.style.top = `${HEADER_LABEL_Y}px`;
      label.style.left = `${x + 3}px`;
      label.textContent = `${dt.getDate()}`;
      hd.append(label);
      dt.setDate(dt.getDate() + 1);
      x += 22;

      // 日付区切り線
      let line = document.createElement("div");
      line.className = "dtline";
      line.style.top = `${dtTop}px`;
      line.style.left = `${x}px`;
      line.style.width = "1px";
      line.style.height = `${LINE_HEIGHT}px`;
      hd.append(line);
    }
  }
}

// カレンダーボディ
class CalendarBody {
  // カレンダーボディを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    let body = document.createElement("div");
    const height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    body.className = "gantt-body";
    body.style.top = `${HEADER_HEIGHT}px`;
    body.style.left = `${frame.calendarLeft}px`;
    body.style.width = `${frame.width - frame.calendarLeft}px`;
    body.style.height = `${height}px`;
    frag.append(body);

    let dt = new Date(frame.calendarStart.getTime());
    let x = 0;
    while (dt.getTime() <= frame.calendarEnd.getTime()) {
      dt.setDate(dt.getDate() + 1);
      x += 22;
      // 日付区切り線
      let line = document.createElement("div");
      line.className = "dtline";
      line.style.top = `0px`;
      line.style.left = `${x}px`;
      line.style.width = "1px";
      line.style.height = `${height}px`;
      body.append(line);
    }
  }
}

// カレンダースクロールバー
class CalendarScroll {
  // スクロールバーを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    let bar = document.createElement("div");
    bar.className = "scroll-bar";
    bar.style.top = `${frame.height - SCROLL_BAR_WIDTH}px`;
    bar.style.left = `${frame.calendarLeft}px`;
    bar.style.width = `${frame.width - frame.calendarLeft}px`;
    bar.style.height = `${SCROLL_BAR_WIDTH}px`;
    frag.append(bar);
  }
}

// ガントチャート全体フレーム
class GanttFrame {
  width = 0;
  height = 0;
  columnHeader = new ColumnHeader();
  columnBody = new ColumnBody();
  columnScroll = new ColumnScroll();
  calendarHeader = new CalendarHeader();
  calendarBody = new CalendarBody();
  calendarScroll = new CalendarScroll();
  colW = 0;
  calendarLeft = 0;
  calendarStart = new Date(2024, 5, 30);
  calendarEnd = new Date(2024, 7, 31);

  // ガントチャートを表示する
  render() {
    const frame = document.querySelector<HTMLDivElement>("#ganttframe");
    if (!frame) {
      console.error("Failed to get #ganttframe!");
      return;
    }
    this.width = frame.offsetWidth;
    this.height = frame.offsetHeight;
    this.colW = this.columnWidth();
    this.calendarLeft = this.colW + 1;
    let frag = document.createDocumentFragment();
    this.columnBody.render(this, frag);
    this.columnHeader.render(this, frag);
    this.columnScroll.render(this, frag);
    this.calendarBody.render(this, frag);
    this.calendarHeader.render(this, frag);
    this.calendarScroll.render(this, frag);

    frame.append(frag);
  }

  columnWidth(): number {
    let w = 2;
    for (let col of cols) {
      w += col.width;
    }
    return w;
  }
}

let f = new GanttFrame();
f.render();
