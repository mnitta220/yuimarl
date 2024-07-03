//import * as dayjs from "dayjs";

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

// スクロールバー
class ScrollBar {
  x1 = 0;
  y1 = 0;
  x2 = 0;
  y2 = 0;

  // スクロールバーを表示する
  render(frag: DocumentFragment) {
    let bar = document.createElement("div");
    bar.className = "scroll-bar";
    bar.style.top = `${this.y1}px`;
    bar.style.left = `${this.x1 + 1}px`;
    bar.style.width = `${this.x2 - this.x1}px`;
    bar.style.height = `${this.y2 - this.y1}px`;
    frag.append(bar);
  }
}

// カレンダー
class Calendar {
  start = new Date(2024, 5, 30);
  end = new Date(2024, 7, 31);

  // カレンダーを表示する
  render(frame: Frame, frag: DocumentFragment, startX: number) {
    let dt = this.start;
    let x = startX;
    let dtTop = LINE_HEIGHT + LINE_HEIGHT;
    console.log(dt.toLocaleDateString());
    while (dt.getTime() <= this.end.getTime()) {
      // カラムラベル
      let label = document.createElement("div");
      label.className = "caldt";
      label.style.top = `${HEADER_LABEL_Y}px`;
      label.style.left = `${x + 3}px`;
      label.textContent = `${dt.getDate()}`;
      frag.append(label);
      dt.setDate(dt.getDate() + 1);
      x += 22;

      // 日付区切り線
      let line = document.createElement("div");
      line.className = "dtline";
      line.style.top = `${dtTop}px`;
      line.style.left = `${x}px`;
      line.style.width = "1px";
      line.style.height = `${frame.height - dtTop}px`;
      frag.append(line);
    }
  }
}

// ガントチャート全体フレーム
class Frame {
  width = 0;
  height = 0;
  calendar = new Calendar();
  scrollBarH = new ScrollBar();
  scrollBarV = new ScrollBar();

  // ガントチャートを表示する
  render() {
    const chart = document.querySelector<HTMLDivElement>("#chart");
    const headerHeight = LINE_HEIGHT * 3;
    if (!chart) {
      console.error("Failed to get #chart!");
      return;
    }
    this.width = chart.offsetWidth;
    this.height = chart.offsetHeight;
    let frag = document.createDocumentFragment();

    // ヘッダー
    let header = document.createElement("div");
    header.className = "header";
    header.style.top = "0px";
    header.style.left = "0px";
    header.style.width = "100%";
    header.style.height = `${headerHeight}px`;
    frag.append(header);

    let line = document.createElement("div");
    line.className = "line";
    line.style.top = `${headerHeight}px`;
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
      header.append(label);
      x += col.width;

      // カラム区切り線
      line = document.createElement("div");
      line.className = "line";
      line.style.top = "0px";
      line.style.left = `${x}px`;
      line.style.width = "1px";
      line.style.height = "100%";
      frag.append(line);
    }

    // カレンダー境界線
    x += 2;
    let calendarX = x;
    line = document.createElement("div");
    line.className = "line";
    line.style.top = "0px";
    line.style.left = `${calendarX}px`;
    line.style.width = "1px";
    line.style.height = "100%";
    frag.append(line);

    this.calendar.render(this, frag, x);

    // カレンダーヘッダー横線1
    line = document.createElement("div");
    line.className = "line";
    line.style.top = `${LINE_HEIGHT}px`;
    line.style.left = `${calendarX}px`;
    line.style.width = `${chart.offsetWidth - calendarX}px`;
    line.style.height = "1px";
    frag.append(line);

    // カレンダーヘッダー横線2
    line = document.createElement("div");
    line.className = "line";
    line.style.top = `${LINE_HEIGHT + LINE_HEIGHT}px`;
    line.style.left = `${calendarX}px`;
    line.style.width = `${chart.offsetWidth - calendarX}px`;
    line.style.height = "1px";
    frag.append(line);

    // 横スクロールバー
    this.scrollBarH.x1 = calendarX;
    this.scrollBarH.y1 = this.height - SCROLL_BAR_WIDTH;
    this.scrollBarH.x2 = this.width - SCROLL_BAR_WIDTH;
    this.scrollBarH.y2 = this.height;
    this.scrollBarH.render(frag);

    // 縦スクロールバー
    this.scrollBarV.x1 = this.width - SCROLL_BAR_WIDTH;
    this.scrollBarV.y1 = LINE_HEIGHT * 3 + 1;
    this.scrollBarV.x2 = this.width;
    this.scrollBarV.y2 = this.height - SCROLL_BAR_WIDTH;
    this.scrollBarV.render(frag);

    // スクロールバーの角
    let corner = document.createElement("div");
    corner.className = "scroll-corner";
    corner.style.top = `${this.height - SCROLL_BAR_WIDTH}px`;
    corner.style.left = `${this.width - SCROLL_BAR_WIDTH + 1}px`;
    corner.style.width = `${SCROLL_BAR_WIDTH}px`;
    corner.style.height = `${SCROLL_BAR_WIDTH}px`;
    frag.append(corner);

    chart.append(frag);
  }
}

//let dt = dayjs();

let f = new Frame();
f.render();
