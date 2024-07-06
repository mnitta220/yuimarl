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
const HEADER_LABEL_Y = 42;
const LINE_HEIGHT = 21;
const HEADER_HEIGHT = 63;
const DAY_WIDTH = 22;
const CALENDAR_MIN = DAY_WIDTH * 10;
const DAY_MILISEC = 1000 * 60 * 60 * 24;

// カラムヘッダー
class ColumnHeader {
  id = "colhead";
  pos = 0;

  // カラムヘッダーを構築する
  build(frag: DocumentFragment) {
    const hd = document.createElement("div");
    hd.id = this.id;
    hd.className = "header";
    hd.style.top = `0px`;
    hd.style.left = `${this.pos}px`;
    hd.style.width = `${frame.calendarLeft}px`;
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

// カラムボディ
class ColumnBody {
  id = "colbody";
  pos = 0;

  // カラムボディを構築する
  build(frag: DocumentFragment) {
    const body = document.createElement("div");
    const height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    body.id = this.id;
    body.className = "gantt-body";
    body.style.top = `${HEADER_HEIGHT}px`;
    body.style.left = `${this.pos}px`;
    body.style.width = `${frame.calendarLeft}px`;
    body.style.height = `${height}px`;
    frag.append(body);

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
      body.append(line);
    }

    // カレンダー境界線
    x += 2;
    const line = document.createElement("div");
    line.className = "line";
    line.style.top = "0px";
    line.style.left = `${x}px`;
    line.style.width = "1px";
    line.style.height = `${height}px`;
    body.append(line);
  }

  scrollH(x: number) {
    this.pos = -x;
    const colbody = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (colbody) {
      colbody.style.left = `${this.pos}px`;
    }
  }
}

// カラムスクロールバー
class ColumnScroll {
  id = "colscr";
  pos = 0;

  // スクロールバーを構築する
  build(frag: DocumentFragment) {
    const bar = document.createElement("div");
    bar.id = this.id;
    bar.className = "scroll-corner";
    bar.style.top = `${frame.height - SCROLL_BAR_WIDTH}px`;
    bar.style.left = `${this.pos}px`;
    bar.style.width = `${frame.calendarLeft}px`;
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

// カレンダーヘッダー
class CalendarHeader {
  id = "calhead";
  width = 0;
  pos = 0;
  dtpos = 0;
  dtStart = 0;
  dtEnd = 0;

  // カレンダーヘッダーを構築する
  build(frag: DocumentFragment) {
    const cv = document.createElement("canvas");
    this.width = frame.width - frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    cv.id = this.id;
    cv.className = "header";
    cv.style.top = `0px`;
    cv.style.left = `${this.pos + frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${HEADER_HEIGHT + 1}px`;
    frag.append(cv);
    this.dtStart = frame.calendarStart.getTime();
    this.dtEnd = frame.calendarEnd.getTime();
  }

  // 描画する
  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cnvs) {
      const height = cnvs.offsetHeight;
      cnvs.width = this.width;
      cnvs.height = height;
      const dtTop = LINE_HEIGHT + LINE_HEIGHT;
      const ctx = cnvs.getContext("2d");
      const font = "9.5pt sans-serif";
      if (ctx) {
        ctx.save();
        // カレンダーヘッダー横線
        ctx.fillStyle = "#82a4c1";
        ctx.fillRect(0, LINE_HEIGHT, this.width, 1);
        ctx.fill();
        ctx.fillRect(0, LINE_HEIGHT + LINE_HEIGHT, this.width, 1);
        ctx.fill();
        ctx.fillRect(0, HEADER_HEIGHT, this.width, 1);
        ctx.fill();

        let dt = new Date(this.dtStart);
        let x = 0;
        let first = true;
        ctx.font = font;
        ctx.textBaseline = "bottom";
        ctx.textAlign = "left";

        while (dt.getTime() <= this.dtEnd) {
          if (x < this.dtpos - DAY_WIDTH) {
            x += DAY_WIDTH;
            dt.setDate(dt.getDate() + 1);
            first = false;
            continue;
          }
          if (x > this.width + this.dtpos) break;

          const date = dt.getDate();
          if (date == 1 || (first && date < 25)) {
            ctx.fillStyle = "#000";
            ctx.fillText(
              `${dt.getFullYear()}/${dt.getMonth() + 1}`,
              x + 3 - this.dtpos,
              LINE_HEIGHT + LINE_HEIGHT - 3
            );
            ctx.fillStyle = "#bdcede";
            ctx.fillRect(x - this.dtpos, LINE_HEIGHT, 1, LINE_HEIGHT);
            ctx.fill();
          }
          first = false;
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
          ctx.fillText(`${date}`, x + 3 - this.dtpos, HEADER_HEIGHT - 3);
          x += DAY_WIDTH;
          dt.setTime(dt.getTime() + DAY_MILISEC);
          // 日付区切り線
          ctx.fillStyle = "#bdcede";
          ctx.fillRect(x - this.dtpos, dtTop, 1, LINE_HEIGHT);
          ctx.fill();
        }

        ctx.restore();
      }
    }
  }

  scrollH(x: number) {
    this.pos = -x;
    const calhead = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (calhead) {
      calhead.style.left = `${this.pos + frame.calendarLeft}px`;
    }
  }

  scroll(x: number) {
    this.dtpos =
      (x * frame.calendarTotalWidth) /
      (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
    //console.log(`scroll ${x} ${this.dtpos}`);
    this.draw();
    frame.calendarBody.dtpos = this.dtpos;
    frame.calendarBody.draw();
  }
}

// カレンダーボディ
class CalendarBody {
  id = "calbody";
  width = 0;
  pos = 0;
  dtpos = 0;
  dtStart = 0;
  dtEnd = 0;

  // カレンダーボディを構築する
  build(frag: DocumentFragment) {
    const cv = document.createElement("canvas");
    this.width = frame.width - frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    cv.id = this.id;
    cv.className = "gantt-body";
    cv.style.top = `${HEADER_HEIGHT}px`;
    cv.style.left = `${this.pos + frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${height}px`;
    frag.append(cv);
    this.dtStart = frame.calendarStart.getTime();
    this.dtEnd = frame.calendarEnd.getTime();
  }

  // 描画する
  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cnvs) {
      const width = cnvs.offsetWidth;
      const height = cnvs.offsetHeight;
      cnvs.width = width;
      cnvs.height = height;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();
        ctx.fillStyle = "#bdcede";
        let dt = new Date(this.dtStart);
        let x = 0;
        while (dt.getTime() <= this.dtEnd) {
          if (x < this.dtpos - DAY_WIDTH) {
            x += DAY_WIDTH;
            dt.setDate(dt.getDate() + 1);
            continue;
          }
          if (x > this.width + this.dtpos) break;

          dt.setDate(dt.getDate() + 1);
          x += DAY_WIDTH;
          if (x > width + this.dtpos) break;
          // 日付区切り線
          ctx.fillRect(x - this.dtpos, 0, 1, height);
          ctx.fill();
        }

        ctx.restore();
      }
    }
  }

  scrollH(x: number) {
    this.pos = -x;
    const calbody = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (calbody) {
      calbody.style.left = `${this.pos + frame.calendarLeft}px`;
    }
  }
}

// カレンダースクロールバー
class CalendarScroll {
  id = "calscroll";
  width = 0;
  pos = 0;
  barWidth = 0;
  height = SCROLL_BAR_WIDTH;
  moving = false;
  startX = 0;
  barpos = 0;
  startPos = 0;

  // スクロールバーを構築する
  build(frag: DocumentFragment) {
    this.width = frame.width - frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const cv = document.createElement("canvas");
    cv.id = this.id;
    cv.className = "scroll-bar";
    cv.style.top = `${frame.height - SCROLL_BAR_WIDTH}px`;
    cv.style.left = `${this.pos + frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${this.height}px`;
    frag.append(cv);
  }

  // イベントハンドラを登録する
  handler() {
    const calscroll = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (calscroll) {
      calscroll.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseDownCalScroll(e.layerX);
      });
      calscroll.addEventListener("mousemove", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseMoveCalScroll(e.layerX);
      });
      calscroll.addEventListener("mouseup", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpCalScroll();
      });
      calscroll.addEventListener("mouseleave", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpCalScroll();
      });
    }
  }

  // 描画する
  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cnvs) {
      cnvs.width = this.width;
      cnvs.height = this.height;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();

        //ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
        ctx.lineJoin = "miter";
        ctx.fillStyle = "#505050";
        // 左ボタン
        ctx.beginPath();
        ctx.moveTo(5, 8);
        ctx.lineTo(9, 12);
        ctx.lineTo(9, 4);
        ctx.closePath();
        ctx.fill();
        // 右ボタン
        ctx.beginPath();
        ctx.moveTo(this.width - 10, 4);
        ctx.lineTo(this.width - 10, 12);
        ctx.lineTo(this.width - 6, 8);
        ctx.closePath();
        ctx.fill();
        //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
        ctx.fillStyle = "#c1c1c1";
        // バー
        this.barWidth =
          ((this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH) * this.width) /
          frame.calendarTotalWidth;
        ctx.fillRect(
          SCROLL_BAR_WIDTH + this.barpos,
          2,
          this.barWidth < 4 ? 4 : this.barWidth,
          13
        );

        ctx.restore();
      }
    }
  }

  scrollH(x: number) {
    this.pos = -x;
    const calscroll = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (calscroll) {
      calscroll.style.left = `${this.pos + frame.calendarLeft}px`;
    }
  }

  mouseDownCalScroll(x: number) {
    //debugger;
    if (
      SCROLL_BAR_WIDTH + this.barpos < x &&
      x < SCROLL_BAR_WIDTH + this.barpos + this.barWidth
    ) {
      this.moving = true;
      this.startX = x - SCROLL_BAR_WIDTH;
      this.startPos = this.barpos;
      return;
    }

    if (x < SCROLL_BAR_WIDTH) {
      // 左ボタン
      this.barpos -= SCROLL_BAR_WIDTH;
    } else if (x < SCROLL_BAR_WIDTH + this.barpos) {
      // バーの左側
      this.barpos -= this.barWidth;
    } else if (this.width - SCROLL_BAR_WIDTH < x) {
      // 右ボタン
      this.barpos += SCROLL_BAR_WIDTH;
    } else {
      // バーの右側
      this.barpos += this.barWidth;
    }

    if (this.barpos < 0) this.barpos = 0;
    if (
      this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth <
      this.barpos
    ) {
      this.barpos =
        this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth;
    }

    frame.calendarHeader.scroll(this.barpos);
    this.draw();
    this.moving = false;
  }

  mouseMoveCalScroll(x: number) {
    if (!this.moving) return;
    this.barpos = x - SCROLL_BAR_WIDTH - this.startX + this.startPos;
    if (this.barpos < 0) this.barpos = 0;
    const w = this.width - this.barWidth - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH;
    if (this.barpos > w) {
      this.barpos = w;
    }
    frame.calendarHeader.scroll(this.barpos);
    this.draw();
  }

  mouseUpCalScroll() {
    this.moving = false;
  }
}

// 横スクロールバー
class ScrollH {
  id = "scrollh";
  width = 0;
  barWidth = 0;
  height = SCROLL_BAR_WIDTH;
  moving = false;
  startX = 0;
  pos = 0;
  startPos = 0;

  // イベントハンドラを登録する
  handler() {
    const scrollh = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (scrollh) {
      scrollh.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseDownScrollH(e.pageX - scrollh.offsetLeft);
      });
      scrollh.addEventListener("mousemove", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseMoveScrollH(e.pageX - scrollh.offsetLeft);
      });
      scrollh.addEventListener("mouseup", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollH();
      });
      scrollh.addEventListener("mouseleave", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollH();
      });
    }
  }

  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cnvs) {
      this.width = cnvs.offsetWidth;
      //const height = cnvs.offsetHeight;
      cnvs.width = this.width;
      cnvs.height = this.height;
      if (this.width > frame.schThreshold) return;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();

        //ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
        ctx.lineJoin = "miter";
        ctx.fillStyle = "#505050";
        // 左ボタン
        ctx.beginPath();
        ctx.moveTo(5, 8);
        ctx.lineTo(9, 12);
        ctx.lineTo(9, 4);
        ctx.closePath();
        ctx.fill();
        // 右ボタン
        ctx.beginPath();
        ctx.moveTo(this.width - 10, 4);
        ctx.lineTo(this.width - 10, 12);
        ctx.lineTo(this.width - 6, 8);
        ctx.closePath();
        ctx.fill();
        //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
        ctx.fillStyle = "#c1c1c1";
        // バー
        //ctx.fillRect(16, 2, this.width - 32, 13);
        this.barWidth =
          ((this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH) * this.width) /
          frame.schThreshold;
        ctx.fillRect(
          SCROLL_BAR_WIDTH + this.pos,
          2,
          this.barWidth < 4 ? 4 : this.barWidth,
          13
        );

        ctx.restore();
      }
    }
  }

  mouseDownScrollH(x: number) {
    //console.log(`mouseDownScrollH: ${x} ${this.pos} ${this.barWidth}`);
    if (
      SCROLL_BAR_WIDTH + this.pos < x &&
      x < SCROLL_BAR_WIDTH + this.pos + this.barWidth
    ) {
      this.moving = true;
      this.startX = x - SCROLL_BAR_WIDTH;
      this.startPos = this.pos;
      return;
    }

    if (x < SCROLL_BAR_WIDTH) {
      // 左ボタン
      this.pos -= SCROLL_BAR_WIDTH;
    } else if (x < SCROLL_BAR_WIDTH + this.pos) {
      // バーの左側
      this.pos -= this.barWidth;
    } else if (this.width - SCROLL_BAR_WIDTH < x) {
      // 右ボタン
      this.pos += SCROLL_BAR_WIDTH;
    } else {
      // バーの右側
      this.pos += this.barWidth;
    }
    if (this.pos < 0) this.pos = 0;
    if (
      this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth <
      this.pos
    ) {
      this.pos =
        this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth;
    }
    frame.scrollH(this.pos);
    this.draw();
    this.moving = false;
  }

  mouseMoveScrollH(x: number) {
    if (!this.moving) return;
    this.pos = x - SCROLL_BAR_WIDTH - this.startX + this.startPos;
    if (this.pos < 0) this.pos = 0;
    const w = this.width - this.barWidth - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH;
    if (this.pos > w) {
      this.pos = w;
    }
    frame.scrollH(this.pos);
    this.draw();
  }

  mouseUpScrollH() {
    this.moving = false;
  }
}

// 縦スクロールバー
class ScrollV {
  id = "scrollv";

  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
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

// ガントチャート全体フレーム
class GanttFrame {
  id = "ganttframe";
  width = 0;
  height = 0;
  columnHeader = new ColumnHeader();
  columnBody = new ColumnBody();
  columnScroll = new ColumnScroll();
  calendarHeader = new CalendarHeader();
  calendarBody = new CalendarBody();
  calendarScroll = new CalendarScroll();
  sch = new ScrollH();
  scv = new ScrollV();
  colW = 0;
  calendarLeft = 0;
  calendarStart = new Date(2024, 5, 24);
  calendarEnd = new Date(2024, 7, 31);
  calendarTotalWidth = 0;
  schThreshold = 0; // 横スクロールバーを表示するしきい値
  posX = 0;

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
    this.calendarScroll.handler();
    this.sch.handler();
  }

  // ガントチャートを表示する
  draw() {
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

  mouseDownScrollH(x: number) {
    this.sch.mouseDownScrollH(x);
  }

  mouseMoveScrollH(x: number) {
    this.sch.mouseMoveScrollH(x);
  }

  mouseUpScrollH() {
    this.sch.mouseUpScrollH();
  }
}

const frame = new GanttFrame();
frame.build();
frame.handler();
frame.draw();
