import {
  HEADER_HEIGHT,
  SCROLL_BAR_WIDTH,
  TICKET_HEIGHT,
  Ticket,
} from "./common";
import GanttFrame from "./ganttFrame";

// カラムボディ
export default class ColumnBody {
  id = "colbody";
  width = 0;
  height = 0;
  posX = 0;
  posY = 0;

  constructor(private frame: GanttFrame) {}

  // カラムボディを構築する
  build(frag: DocumentFragment) {
    const body = document.createElement("canvas");
    this.height = this.frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    this.frame.ticketsFrameHeight = this.height;
    body.id = this.id;
    this.width = this.frame.calendarLeft;
    body.className = "gantt-body";
    body.style.top = `${HEADER_HEIGHT}px`;
    body.style.left = `${this.posX}px`;
    body.style.width = `${this.width}px`;
    body.style.height = `${this.height}px`;
    frag.append(body);
  }

  // イベントハンドラを登録する
  handler() {
    const ganttframe = document.querySelector<HTMLCanvasElement>(`#ganttframe`);
    const colbody = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    const frame = this.frame;
    if (ganttframe && colbody) {
      colbody.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();

        /*
        // debug start
        const ctx: CanvasRenderingContext2D | null = colbody.getContext("2d");
        if (ctx) {
          ctx.save();
          ctx.fillStyle = "white";
          ctx.fillRect(100, 320, 200, 50);
          ctx.fillStyle = "black";
          ctx.fillText(`click ${e.layerX} ${e.layerY}`, 100, 350);
          ctx.restore();
        }
        // debug end
        */

        frame.columnBody.mouseDown(e.layerX, e.layerY);
      });
      colbody.addEventListener("touchstart", function (e: TouchEvent) {
        e.preventDefault();

        /*
        // debug start
        const ctx: CanvasRenderingContext2D | null = colbody.getContext("2d");
        if (ctx) {
          ctx.save();
          ctx.fillStyle = "white";
          ctx.fillRect(100, 320, 400, 50);
          ctx.fillStyle = "black";
          ctx.fillText(
            `click ${e.touches[0].clientX}, ${e.touches[0].clientY}, ${e.touches[0].pageX}, ${e.touches[0].pageY}, ${e.touches[0].screenX}, ${e.touches[0].screenY}, ${this.offsetLeft}, ${this.offsetTop}, ${ganttframe.offsetLeft}, ${ganttframe.offsetTop}`,
            100,
            350
          );
          ctx.restore();
        }
        // debug end
        */

        frame.columnBody.mouseDown(
          e.touches[0].pageX - ganttframe.offsetLeft - colbody.offsetLeft,
          e.touches[0].pageY - ganttframe.offsetTop - colbody.offsetTop
        );
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

        this.frame.ticketsTotalHeight = this.drawTickets(
          ctx,
          this.frame.tickets,
          0,
          0
        );

        ctx.fillStyle = "#82a4c1";

        let x = 0;
        for (let col of this.frame.cols) {
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
    let x = this.frame.cols[0].width + level * 12 + 6;
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
    let w = this.frame.cols[0].width + this.frame.cols[1].width - (x + 14);
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

    x = this.frame.cols[0].width + this.frame.cols[1].width + 3;
    // 開始日
    if (ticket.start) {
      ctx.fillText(this.dtDisp(ticket.start), x, y1 + this.posY);
    }
    x += this.frame.cols[2].width;
    // 終了日
    if (ticket.end) {
      ctx.fillText(this.dtDisp(ticket.end), x, y1 + this.posY);
    }
    x += this.frame.cols[3].width + this.frame.cols[4].width - 6;
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
    this.clickTickets(this.frame.tickets, x, y, 0, 0);
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
    if (
      clickx > this.frame.cols[0].width &&
      clickx < this.frame.cols[0].width + this.frame.cols[1].width
    ) {
      let x = this.frame.cols[0].width + level * 12;
      if (clickx > x && clickx < x + 13) {
        // チケット名の□をクリックした
        ticket.open = !ticket.open;
        this.frame.draw();
        if (
          this.frame.ticketsTotalHeight < this.frame.ticketsFrameHeight &&
          this.posY != 0
        ) {
          this.posY = 0;
          this.frame.calendarBody.posY = 0;
          this.frame.scv.pos = 0;
          this.frame.draw();
        }
        return true;
      }
    }
    return false;
  }
}
