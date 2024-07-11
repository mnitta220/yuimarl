// カラム
class Column {
  name: string = "";
  width: number = 0;
}

// カラム定義
const cols = [
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
] as Column[];

class Ticket {
  id: string = "";
  idDisp: string = "";
  name: string = "";
  start: Date | null = null;
  end: Date | null = null;
  progress: number = 0;
  open: boolean = false;
  children: Ticket[] = [];
}

const tickets = [
  {
    id: "YU1",
    idDisp: "YU1",
    name: "チケットYU1",
    start: new Date(2024, 6, 1),
    end: new Date(2024, 6, 31),
    progress: 50,
    open: true,
    children: [
      {
        id: "YU11",
        idDisp: "YU11",
        name: "チケットYU11",
        start: new Date(2024, 6, 1),
        end: new Date(2024, 6, 2),
        progress: 100,
        open: true,
        children: [
          {
            id: "YU111",
            idDisp: "YU111",
            name: "チケットYU111あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
        ],
      },
    ],
  },
  {
    id: "YU2",
    idDisp: "YU2",
    name: "チケットYU2",
    start: new Date(2024, 6, 1),
    end: null,
    progress: 0,
    open: true,
    children: [
      {
        id: "YU21",
        idDisp: "YU21",
        name: "チケットYU21",
        start: new Date(2024, 6, 1),
        end: new Date(2024, 6, 2),
        progress: 100,
        open: true,
        children: [
          {
            id: "YU211",
            idDisp: "YU211",
            name: "チケットYU211あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
        ],
      },
      {
        id: "YU22",
        idDisp: "YU22",
        name: "チケットYU22",
        start: new Date(2024, 6, 1),
        end: new Date(2024, 6, 2),
        progress: 100,
        open: true,
        children: [
          {
            id: "YU221",
            idDisp: "YU221",
            name: "チケットYU221あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU222",
            idDisp: "YU222",
            name: "チケットYU222あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU223",
            idDisp: "YU223",
            name: "チケットYU223あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [
              {
                id: "YU2231",
                idDisp: "YU2231",
                name: "チケットYU2231あいうえおかきくけこさしすせそたちつてと",
                start: new Date(2024, 6, 10),
                end: new Date(2024, 6, 16),
                progress: 10,
                open: true,
                children: [
                  {
                    id: "YU223a",
                    idDisp: "YU223a",
                    name: "チケットYU223aあいうえおかきくけこさしすせそたちつてと",
                    start: new Date(2024, 6, 10),
                    end: new Date(2024, 6, 16),
                    progress: 10,
                    open: true,
                    children: [],
                  },
                ],
              },
            ],
          },
          {
            id: "YU224",
            idDisp: "YU224",
            name: "チケットYU224あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU225",
            idDisp: "YU225",
            name: "チケットYU225あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU226",
            idDisp: "YU226",
            name: "チケットYU226あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU227",
            idDisp: "YU227",
            name: "チケットYU227あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU228",
            idDisp: "YU228",
            name: "チケットYU228あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU229",
            idDisp: "YU229",
            name: "チケットYU229あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU2210",
            idDisp: "YU2210",
            name: "チケットYU2210あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
        ],
      },
    ],
  },
  {
    id: "YU3",
    idDisp: "YU3",
    name: "チケットYU3あいうえおかきくけこさしすせそたa",
    start: null,
    end: new Date(2024, 6, 1),
    progress: 25,
    open: true,
    children: [
      {
        id: "YU31",
        idDisp: "YU31",
        name: "チケットYU31",
        start: new Date(2024, 6, 1),
        end: new Date(2024, 6, 2),
        progress: 100,
        open: true,
        children: [
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
          {
            id: "YU311",
            idDisp: "YU311",
            name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
            start: new Date(2024, 6, 10),
            end: new Date(2024, 6, 16),
            progress: 10,
            open: true,
            children: [],
          },
        ],
      },
    ],
  },
  {
    id: "YU9999",
    idDisp: "YU9999",
    name: "チケットYU9999あいうえおかきくけこさしすせそたちつてと",
    start: new Date(2024, 6, 1),
    end: new Date(2024, 6, 3),
    progress: 40,
    open: true,
    children: [],
  },
] as Ticket[];

// 日本の祝日
const holidays = [new Date(2024, 6, 15), new Date(2024, 7, 12)];

const SCROLL_BAR_WIDTH = 16;
const HEADER_LABEL_Y = 42;
const LINE_HEIGHT = 21;
const HEADER_HEIGHT = 63;
const TICKET_HEIGHT = 22;
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
    line.style.width = `${frame.calendarLeft}px`;
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
  width = 0;
  height = 0;
  posX = 0;
  posY = 0;

  // カラムボディを構築する
  build(frag: DocumentFragment) {
    const body = document.createElement("canvas");
    this.height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    frame.ticketsFrameHeight = this.height;
    body.id = this.id;
    this.width = frame.calendarLeft;
    body.className = "gantt-body";
    body.style.top = `${HEADER_HEIGHT}px`;
    body.style.left = `${this.posX}px`;
    body.style.width = `${this.width}px`;
    body.style.height = `${this.height}px`;
    frag.append(body);
  }

  // イベントハンドラを登録する
  handler() {
    const calscroll = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (calscroll) {
      calscroll.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();
        frame.columnBody.mouseDown(e.layerX, e.layerY);
      });
    }
  }

  // 描画する
  draw() {
    const cnvs = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cnvs) {
      const width = cnvs.offsetWidth;
      const height = cnvs.offsetHeight;
      cnvs.width = width;
      cnvs.height = height;
      const ctx: CanvasRenderingContext2D | null = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();

        frame.ticketsTotalHeight = this.drawTickets(ctx, tickets, 0, 0);

        ctx.fillStyle = "#82a4c1";

        let x = 0;
        for (let col of cols) {
          x += col.width;
          // カラム区切り線
          ctx.fillRect(x, 0, 1, height);
          ctx.fill();
        }

        // カレンダー境界線
        x += 2;
        ctx.fillRect(x, 0, 1, height);
        ctx.fill();

        ctx.restore();
      }
    }
  }

  drawTickets(
    ctx: CanvasRenderingContext2D,
    ts: Ticket[],
    level: number,
    y: number
  ): number {
    let y1 = y;
    for (let t of ts) {
      this.drawTicket(ctx, t, level, y1);
      y1 += TICKET_HEIGHT;
      if (t.open && t.children.length > 0) {
        y1 = this.drawTickets(ctx, t.children, level + 1, y1);
      }
    }
    return y1;
  }

  getFrameHeight(ts: Ticket[], y: number): number {
    let y1 = y;
    for (let t of ts) {
      y1 += TICKET_HEIGHT;
      if (t.open && t.children.length > 0) {
        y1 = this.getFrameHeight(t.children, y1);
      }
    }
    return y1;
  }

  // チケットを描画する
  drawTicket(
    ctx: CanvasRenderingContext2D,
    ticket: Ticket,
    level: number,
    y: number
  ) {
    ctx.font = "9.5pt sans-serif";
    ctx.textBaseline = "bottom";
    const y1 = y + 18;
    ctx.fillStyle = "#00f";
    // ID
    ctx.fillText(ticket.idDisp, 3, y1 + this.posY);
    ctx.fillStyle = "#808080";
    // チケット□
    let x = cols[0].width + level * 12 + 6;
    ctx.fillRect(x, y + 7 + this.posY, 1, 8);
    ctx.fill();
    ctx.fillRect(x, y + 7 + this.posY, 8, 1);
    ctx.fill();
    ctx.fillRect(x, y + 15 + this.posY, 8, 1);
    ctx.fill();
    ctx.fillRect(x + 8, y + 7 + this.posY, 1, 9);
    ctx.fill();
    ctx.fillStyle = "#000";

    if (ticket.children.length > 0) {
      ctx.fillRect(x + 2, y + 11 + this.posY, 5, 1);
      ctx.fill();
      if (!ticket.open) {
        ctx.fillRect(x + 4, y + 9 + this.posY, 1, 5);
        ctx.fill();
      }
    }

    // チケット名
    let w = cols[0].width + cols[1].width - (x + 14);
    let m = ctx.measureText(ticket.name).width;
    if (m <= w) {
      ctx.fillText(ticket.name, x + 14, y1 + this.posY);
    } else {
      let s = ticket.name;
      while (ctx.measureText(`${s}…`).width > w) {
        s = s.slice(0, -1);
      }
      ctx.fillText(`${s}…`, x + 14, y1 + this.posY);
    }

    x = cols[0].width + cols[1].width + 3;
    // 開始日
    if (ticket.start) {
      ctx.fillText(this.dtDisp(ticket.start), x, y1 + this.posY);
    }
    x += cols[2].width;
    // 終了日
    if (ticket.end) {
      ctx.fillText(this.dtDisp(ticket.end), x, y1 + this.posY);
    }
    x += cols[3].width + cols[4].width - 6;
    // 進捗率
    const t1 = `${ticket.progress}`;
    const m1 = ctx.measureText(t1).width;
    ctx.fillText(t1, x - m1, y1 + this.posY);

    ctx.fillStyle = "#e5ebf2";
    // 区切り線
    ctx.fillRect(0, y + TICKET_HEIGHT + this.posY, this.width, 1);
    ctx.fill();
  }

  dtDisp(dt: Date): string {
    const y = `${dt.getFullYear()}`.slice(-2);
    const m = `0${dt.getMonth() + 1}`.slice(-2);
    const d = `0${dt.getDate()}`.slice(-2);
    return `${y}/${m}/${d}`;
  }

  scrollH(x: number) {
    this.posX = -x;
    const colbody = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (colbody) {
      colbody.style.left = `${this.posX}px`;
    }
  }

  scrollV(y: number) {
    this.posY = -y;
    this.draw();
  }

  mouseDown(x: number, y: number) {
    this.clickTickets(tickets, x, y, 0, 0);
  }

  clickTickets(
    ts: Ticket[],
    clickx: number,
    clicky: number,
    level: number,
    y: number
  ): number {
    let y1 = y;
    for (let t of ts) {
      if (this.clickTicket(t, clickx, clicky, level, y1)) {
        break;
      }
      y1 += TICKET_HEIGHT;
      if (t.open && t.children.length > 0) {
        y1 = this.clickTickets(t.children, clickx, clicky, level + 1, y1);
      }
    }
    return y1;
  }

  clickTicket(
    ticket: Ticket,
    clickx: number,
    clicky: number,
    level: number,
    y: number
  ): boolean {
    if (clicky < y + this.posY || clicky > y + TICKET_HEIGHT + this.posY) {
      return false;
    }
    if (clickx > cols[0].width && clickx < cols[0].width + cols[1].width) {
      let x = cols[0].width + level * 12 + 6;
      if (
        clickx > x - 1 &&
        clickx < x + 9 &&
        clicky > y + 6 + this.posY &&
        clicky < y + 16 + this.posY
      ) {
        // チケット名の□をクリックした
        ticket.open = !ticket.open;
        frame.draw();
        if (
          frame.ticketsTotalHeight < frame.ticketsFrameHeight &&
          this.posY != 0
        ) {
          this.posY = 0;
          frame.calendarBody.posY = 0;
          frame.scv.pos = 0;
          frame.draw();
        }
        return true;
      }
    }
    return false;
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

  resize() {
    this.width = frame.width - frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const cv = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cv) {
      cv.style.width = `${this.width}px`;
    }
    this.dtpos = 0;
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
            dt.setTime(dt.getTime() + DAY_MILISEC);
            first = false;
            continue;
          }
          if (x > this.width + this.dtpos) break;

          const date = dt.getDate();

          if (first) {
            first = false;
            if (date == 1 || date < 25) {
              ctx.fillStyle = "#000";
              ctx.fillText(
                `${dt.getFullYear()}/${dt.getMonth() + 1}`,
                x + 3 - this.dtpos,
                LINE_HEIGHT + LINE_HEIGHT - 3
              );
            }
          } else if (date === 1) {
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

          const holiday = holidays.filter((d) => d.getTime() === dt.getTime());
          if (holiday.length > 0) {
            ctx.fillStyle = "#f00";
          } else {
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
    this.draw();
    frame.calendarBody.dtpos = this.dtpos;
    frame.calendarBody.draw();
  }
}

// カレンダーボディ
class CalendarBody {
  id = "calbody";
  width = 0;
  posX = 0;
  posY = 0;
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
    cv.style.left = `${this.posX + frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${height}px`;
    frag.append(cv);
    this.dtStart = frame.calendarStart.getTime();
    this.dtEnd = frame.calendarEnd.getTime();
  }

  resize() {
    this.width = frame.width - frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const cv = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cv) {
      cv.style.width = `${this.width}px`;
    }
    this.dtpos = 0;
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

        let dt = new Date(this.dtStart);
        let x = 0;
        while (dt.getTime() <= this.dtEnd) {
          if (x < this.dtpos - DAY_WIDTH) {
            x += DAY_WIDTH;
            dt.setTime(dt.getTime() + DAY_MILISEC);
            continue;
          }
          if (x > this.width + this.dtpos) break;

          let dayoff = false;
          const holiday = holidays.filter((d) => d.getTime() === dt.getTime());
          if (holiday.length > 0) {
            dayoff = true;
          } else {
            switch (dt.getDay()) {
              case 0: // Sunday
              case 6: // Saturday
                dayoff = true;
                break;
            }
          }
          if (dayoff) {
            ctx.fillStyle = "#f2fef2";
            ctx.fillRect(x + 1 - this.dtpos, 0, DAY_WIDTH - 1, height);
            ctx.fill();
          }

          dt.setTime(dt.getTime() + DAY_MILISEC);
          x += DAY_WIDTH;
          if (x > width + this.dtpos) break;
          // 日付区切り線
          ctx.fillStyle = "#bdcede";
          ctx.fillRect(x - this.dtpos, 0, 1, height);
          ctx.fill();
        }

        this.drawTickets(ctx, tickets, 0);

        // 現在線
        x = ((new Date().getTime() - this.dtStart) / DAY_MILISEC) * DAY_WIDTH;
        ctx.fillStyle = "#0a0";
        for (let y = 0; y < height; y += 7) {
          ctx.fillRect(x - this.dtpos, y, 1, 4);
          ctx.fill();
        }

        ctx.restore();
      }
    }
  }

  drawTickets(ctx: CanvasRenderingContext2D, ts: Ticket[], y: number): number {
    let y1 = y;
    for (let t of ts) {
      this.drawTicket(ctx, t, y1);
      y1 += TICKET_HEIGHT;
      if (t.open && t.children.length > 0) {
        y1 = this.drawTickets(ctx, t.children, y1);
      }
    }
    return y1;
  }

  // チケットを描画する
  drawTicket(ctx: CanvasRenderingContext2D, ticket: Ticket, y: number) {
    // チケット開始日/終了日
    if (ticket.start) {
      if (ticket.end) {
        let x1 = (ticket.start.getTime() - this.dtStart) / DAY_MILISEC;
        x1 = x1 * DAY_WIDTH - this.dtpos;
        let x2 = (ticket.end.getTime() - this.dtStart) / DAY_MILISEC;
        x2 = (x2 + 1) * DAY_WIDTH - this.dtpos;
        if (ticket.progress === 0) {
          ctx.fillStyle = "#9bf";
          ctx.fillRect(x1, y + 8 + this.posY, x2 - x1, 6);
          ctx.fill();
        } else if (ticket.progress === 100) {
          ctx.fillStyle = "#57f";
          ctx.fillRect(x1, y + 8 + this.posY, x2 - x1, 6);
          ctx.fill();
        } else {
          ctx.fillStyle = "#9bf";
          ctx.fillRect(x1, y + 8 + this.posY, x2 - x1, 6);
          ctx.fill();
          const x3 = ((x2 - x1) * ticket.progress) / 100;
          ctx.fillStyle = "#57f";
          ctx.fillRect(x1, y + 8 + this.posY, x3, 6);
          ctx.fill();
        }
      } else {
        ctx.fillStyle = "#57f";
        let x1 = (ticket.start.getTime() - this.dtStart) / DAY_MILISEC;
        x1 = x1 * DAY_WIDTH - this.dtpos;
        ctx.fillRect(x1, y + 8 + this.posY, 12, 6);
        ctx.fill();
        ctx.fillStyle = "#68f";
        x1 += 12;
        ctx.fillRect(x1, y + 8 + this.posY, 7, 6);
        ctx.fill();
        ctx.fillStyle = "#8af";
        x1 += 7;
        ctx.fillRect(x1, y + 8 + this.posY, 6, 6);
        ctx.fill();
        ctx.fillStyle = "#bdf";
        x1 += 6;
        ctx.fillRect(x1, y + 8 + this.posY, 5, 6);
        ctx.fill();
      }
    } else {
      if (ticket.end) {
        ctx.fillStyle = "#57f";
        let x2 = (ticket.end.getTime() - this.dtStart) / DAY_MILISEC;
        x2 = (x2 + 1) * DAY_WIDTH - this.dtpos;
        x2 -= 12;
        ctx.fillRect(x2, y + 8 + this.posY, 12, 6);
        ctx.fill();
        ctx.fillStyle = "#68f";
        x2 -= 7;
        ctx.fillRect(x2, y + 8 + this.posY, 7, 6);
        ctx.fill();
        ctx.fillStyle = "#8af";
        x2 -= 6;
        ctx.fillRect(x2, y + 8 + this.posY, 6, 6);
        ctx.fill();
        ctx.fillStyle = "#bdf";
        x2 -= 5;
        ctx.fillRect(x2, y + 8 + this.posY, 5, 6);
        ctx.fill();
      }
    }

    ctx.fillStyle = "#e5ebf2";
    // 区切り線
    ctx.fillRect(0, y + TICKET_HEIGHT + this.posY, this.width, 1);
    ctx.fill();
  }

  scrollH(x: number) {
    this.posX = -x;
    const calbody = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (calbody) {
      calbody.style.left = `${this.posX + frame.calendarLeft}px`;
    }
  }

  scrollV(y: number) {
    this.posY = -y;
    this.draw();
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

  resize() {
    this.width = frame.width - frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const cv = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cv) {
      cv.style.width = `${this.width}px`;
    }
    this.scrollH(0);
    this.barpos = 0;
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
  width = SCROLL_BAR_WIDTH;
  height = 0;
  barHeight = 0;
  moving = false;
  visible = false;
  startY = 0;
  pos = 0;
  startPos = 0;

  // イベントハンドラを登録する
  handler() {
    const scrollv = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (scrollv) {
      scrollv.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseDownScrollV(e.pageY - scrollv.offsetTop);
      });
      scrollv.addEventListener("mousemove", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseMoveScrollV(e.pageY - scrollv.offsetTop);
      });
      scrollv.addEventListener("mouseup", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollV();
      });
      scrollv.addEventListener("mouseleave", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollV();
      });
    }
  }

  draw() {
    this.visible = false;
    const cnvs = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    if (cnvs) {
      this.height = cnvs.offsetHeight;
      cnvs.width = this.width;
      cnvs.height = this.height;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        //ctx.fillStyle =
        //  frame.ticketsHeight < frame.ticketTotalHeight ? "#505050" : "#a3a3a3";
        if (frame.ticketsFrameHeight < frame.ticketsTotalHeight) {
          this.visible = true;
          ctx.save();
          ctx.lineJoin = "miter";
          ctx.fillStyle = "#505050";
          ctx.beginPath();
          ctx.moveTo(8, 5);
          ctx.lineTo(4, 9);
          ctx.lineTo(12, 9);
          ctx.closePath();
          ctx.fill();
          ctx.beginPath();
          ctx.moveTo(4, this.height - 10);
          ctx.lineTo(8, this.height - 6);
          ctx.lineTo(12, this.height - 10);
          ctx.closePath();
          ctx.fill();
          //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
          ctx.fillStyle = "#c1c1c1";
          // バー
          this.barHeight =
            ((this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH) *
              frame.ticketsFrameHeight) /
            frame.ticketsTotalHeight;
          ctx.fillRect(
            2,
            SCROLL_BAR_WIDTH + this.pos,
            13,
            this.barHeight < 4 ? 4 : this.barHeight
          );
          ctx.restore();
        }
      }
    }
  }

  mouseDownScrollV(y: number) {
    if (!this.visible) return;
    if (
      SCROLL_BAR_WIDTH + this.pos < y &&
      y < SCROLL_BAR_WIDTH + this.pos + this.barHeight
    ) {
      this.moving = true;
      this.startY = y - SCROLL_BAR_WIDTH;
      this.startPos = this.pos;
      return;
    }

    if (y < SCROLL_BAR_WIDTH) {
      // 上ボタン
      this.pos -= SCROLL_BAR_WIDTH;
    } else if (y < SCROLL_BAR_WIDTH + this.pos) {
      // バーの上
      this.pos -= this.barHeight;
    } else if (this.height - SCROLL_BAR_WIDTH < y) {
      // 下ボタン
      this.pos += SCROLL_BAR_WIDTH;
    } else {
      // バーの下
      this.pos += this.barHeight;
    }
    if (this.pos < 0) this.pos = 0;
    let percent =
      this.pos /
      (this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight);
    if (percent > 1) {
      percent = 1;
      this.pos =
        this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight;
    }
    frame.scrollV(
      (frame.ticketsTotalHeight - frame.ticketsFrameHeight) * percent
    );
    this.draw();
    this.moving = false;
  }

  mouseMoveScrollV(y: number) {
    if (!this.visible || !this.moving) return;
    this.pos = y - SCROLL_BAR_WIDTH - this.startY + this.startPos;
    if (this.pos < 0) this.pos = 0;
    let percent =
      this.pos /
      (this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight);
    if (percent > 1) {
      percent = 1;
      this.pos =
        this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight;
    }
    frame.scrollV(
      (frame.ticketsTotalHeight - frame.ticketsFrameHeight) * percent
    );
    this.draw();
  }

  mouseUpScrollV() {
    this.moving = false;
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
  ticketsTotalHeight = 0;
  ticketsFrameHeight = 0;
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
    //ganttbase.style.width = `${flexbox.offsetWidth - SCROLL_BAR_WIDTH}px`;
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

const frame = new GanttFrame();
frame.build();
frame.handler();
frame.draw();

window.addEventListener("resize", function () {
  frame.resize();
  frame.draw();
});
