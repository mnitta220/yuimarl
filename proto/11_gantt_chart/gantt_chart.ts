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

  // カラムヘッダーを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    const hd = document.createElement("div");
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
    //let calendarX = x;
    line = document.createElement("div");
    line.className = "line";
    line.style.top = "0px";
    line.style.left = `${x}px`;
    line.style.width = "1px";
    line.style.height = `${HEADER_HEIGHT}px`;
    frag.append(line);
  }
}

// カラムボディ
class ColumnBody {
  // カラムボディを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    const bar = document.createElement("div");
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
      const line = document.createElement("div");
      line.className = "line";
      line.style.top = "0px";
      line.style.left = `${x}px`;
      line.style.width = "1px";
      line.style.height = `${height}px`;
      bar.append(line);
    }

    // カレンダー境界線
    x += 2;
    const line = document.createElement("div");
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
    const bar = document.createElement("div");
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
    const cv = document.createElement("canvas");
    const width = frame.width - frame.calendarLeft;
    cv.id = "calhead";
    cv.className = "header";
    cv.style.top = `0px`;
    cv.style.left = `${frame.calendarLeft}px`;
    cv.style.width = `${width}px`;
    cv.style.height = `${HEADER_HEIGHT + 1}px`;
    frag.append(cv);
  }

  // 描画する
  draw(frame: GanttFrame) {
    const cnvs = document.querySelector<HTMLCanvasElement>("#calhead");
    if (cnvs) {
      const width = cnvs.offsetWidth;
      const height = cnvs.offsetHeight;
      cnvs.width = width;
      cnvs.height = height;
      const dtTop = LINE_HEIGHT + LINE_HEIGHT;
      const ctx = cnvs.getContext("2d");
      const font2 = "9.5pt sans-serif";
      if (ctx) {
        ctx.save();
        // カレンダーヘッダー横線
        ctx.fillStyle = "#82a4c1";
        ctx.fillRect(0, LINE_HEIGHT, width, 1);
        ctx.fill();
        ctx.fillRect(0, LINE_HEIGHT + LINE_HEIGHT, width, 1);
        ctx.fill();
        ctx.fillRect(0, HEADER_HEIGHT, width, 1);
        ctx.fill();

        let dt = new Date(frame.calendarStart.getTime());
        let x = 0;
        ctx.font = font2;
        ctx.textBaseline = "bottom";
        ctx.textAlign = "left";

        while (dt.getTime() <= frame.calendarEnd.getTime()) {
          switch (dt.getDay()) {
            case 0: // Sunday
              ctx.fillStyle = "#f00";
              break;
            case 6: // Saturday
              ctx.fillStyle = "#00f";
              break;
            default:
              ctx.fillStyle = "#000";
              break;
          }
          ctx.fillText(`${dt.getDate()}`, x + 3, HEADER_HEIGHT - 2);
          x += 22;
          if (x > width) break;
          dt.setDate(dt.getDate() + 1);
          // 日付区切り線
          ctx.fillStyle = "#bdcede";
          ctx.fillRect(x, dtTop, 1, LINE_HEIGHT);
          ctx.fill();
        }

        ctx.restore();
      }
    }
  }
}

// カレンダーボディ
class CalendarBody {
  // カレンダーボディを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    const cv = document.createElement("canvas");
    const height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    cv.id = "calbody";
    cv.className = "gantt-body";
    cv.style.top = `${HEADER_HEIGHT}px`;
    cv.style.left = `${frame.calendarLeft}px`;
    cv.style.width = `${frame.width - frame.calendarLeft}px`;
    cv.style.height = `${height}px`;
    frag.append(cv);
  }

  // 描画する
  draw(frame: GanttFrame) {
    const cnvs = document.querySelector<HTMLCanvasElement>("#calbody");
    if (cnvs) {
      const width = cnvs.offsetWidth;
      const height = cnvs.offsetHeight;
      cnvs.width = width;
      cnvs.height = height;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();
        ctx.fillStyle = "#bdcede";
        let dt = new Date(frame.calendarStart.getTime());
        let x = 0;
        while (dt.getTime() <= frame.calendarEnd.getTime()) {
          dt.setDate(dt.getDate() + 1);
          x += 22;
          if (x > width) break;
          // 日付区切り線
          ctx.fillRect(x, 0, 1, height);
          ctx.fill();
        }

        ctx.restore();
      }
    }
  }
}

// カレンダースクロールバー
class CalendarScroll {
  width = 0;
  totalWidth = 0;
  height = SCROLL_BAR_WIDTH;
  moving = false;
  startX = 0;

  // スクロールバーを表示する
  render(frame: GanttFrame, frag: DocumentFragment) {
    this.width = frame.width - frame.calendarLeft;
    const cv = document.createElement("canvas");
    cv.id = "calscroll";
    cv.className = "scroll-bar";
    cv.style.top = `${frame.height - SCROLL_BAR_WIDTH}px`;
    cv.style.left = `${frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${this.height}px`;
    frag.append(cv);
  }

  // 描画する
  draw(frame: GanttFrame) {
    const cnvs = document.querySelector<HTMLCanvasElement>("#calscroll");
    if (cnvs) {
      cnvs.width = this.width;
      cnvs.height = this.height;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();
        //ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
        ctx.lineJoin = "miter";
        ctx.fillStyle = "#505050";
        ctx.beginPath();
        ctx.moveTo(5, 8);
        ctx.lineTo(9, 12);
        ctx.lineTo(9, 4);
        ctx.closePath();
        ctx.fill();
        ctx.beginPath();
        ctx.moveTo(this.width - 10, 4);
        ctx.lineTo(this.width - 10, 12);
        ctx.lineTo(this.width - 6, 8);
        ctx.closePath();
        ctx.fill();
        //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
        ctx.fillStyle = "#c1c1c1";
        ctx.fillRect(16, 2, this.width - 32, 13);

        ctx.restore();
      }
    }
  }

  mouseDownCalScroll(x: number) {
    this.moving = true;
    this.startX = x;
    console.log(`mouseDownCalScroll: ${x}`);
  }

  mouseMoveCalScroll(x: number) {
    if (!this.moving) return;
    console.log(`mouseMoveCalScroll: ${x}`);
  }

  mouseUpCalScroll() {
    this.moving = false;
    console.log(`mouseUpCalScroll`);
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

  draw() {
    this.calendarHeader.draw(this);
    this.calendarBody.draw(this);
    this.calendarScroll.draw(this);
  }

  columnWidth(): number {
    let w = 2;
    for (let col of cols) {
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
}

// 横スクロールバー
class ScrollH {
  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>("#scrollh");
    if (cnvs) {
      const width = cnvs.offsetWidth;
      const height = cnvs.offsetHeight;
      cnvs.width = width;
      cnvs.height = height;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();
        //ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
        ctx.lineJoin = "miter";
        ctx.fillStyle = "#505050";
        ctx.beginPath();
        ctx.moveTo(5, 8);
        ctx.lineTo(9, 12);
        ctx.lineTo(9, 4);
        ctx.closePath();
        ctx.fill();
        ctx.beginPath();
        ctx.moveTo(width - 10, 4);
        ctx.lineTo(width - 10, 12);
        ctx.lineTo(width - 6, 8);
        ctx.closePath();
        ctx.fill();
        //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
        ctx.fillStyle = "#c1c1c1";
        ctx.fillRect(16, 2, width - 32, 13);
        ctx.restore();
      }
    }
  }
}

// 縦スクロールバー
class ScrollV {
  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>("#scrollv");
    if (cnvs) {
      const width = cnvs.offsetWidth;
      const height = cnvs.offsetHeight;
      cnvs.width = width;
      cnvs.height = height;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();
        //ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
        ctx.lineJoin = "miter";
        ctx.fillStyle = "#505050";
        ctx.beginPath();
        ctx.moveTo(8, 5);
        ctx.lineTo(4, 9);
        ctx.lineTo(12, 9);
        ctx.closePath();
        ctx.fill();
        ctx.beginPath();
        ctx.moveTo(4, height - 10);
        ctx.lineTo(8, height - 6);
        ctx.lineTo(12, height - 10);
        ctx.closePath();
        ctx.fill();
        //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
        ctx.fillStyle = "#c1c1c1";
        ctx.fillRect(2, 16, 13, height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
        ctx.restore();
      }
    }
  }
}

const frame = new GanttFrame();
frame.render();
frame.draw();
const sch = new ScrollH();
sch.draw();
const scv = new ScrollV();
scv.draw();

document
  .querySelector<HTMLCanvasElement>("#calscroll")
  ?.addEventListener("mousedown", function (e) {
    e.preventDefault();
    frame.mouseDownCalScroll(e.layerX);
  });

document
  .querySelector<HTMLCanvasElement>("#calscroll")
  ?.addEventListener("mousemove", function (e) {
    e.preventDefault();
    frame.mouseMoveCalScroll(e.layerX);
  });

document
  .querySelector<HTMLCanvasElement>("#calscroll")
  ?.addEventListener("mouseup", function (e) {
    e.preventDefault();
    frame.mouseUpCalScroll();
  });
