import {
  SCROLL_BAR_WIDTH,
  CALENDAR_MIN,
  HEADER_HEIGHT,
  DAY_WIDTH,
  DAY_MILISEC,
  TICKET_HEIGHT,
  Ticket,
} from "./common";
import GanttFrame from "./ganttFrame";

// カレンダーボディ
export default class CalendarBody {
  id = "calbody";
  width = 0;
  posX = 0;
  posY = 0;
  dtpos = 0;
  dtStart = 0;
  dtEnd = 0;

  constructor(private frame: GanttFrame) {}

  // カレンダーボディを構築する
  build(frag: DocumentFragment) {
    const cv = document.createElement("canvas");
    this.width = this.frame.width - this.frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const height = this.frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    cv.id = this.id;
    cv.className = "gantt-body";
    cv.style.top = `${HEADER_HEIGHT}px`;
    cv.style.left = `${this.posX + this.frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${height}px`;
    frag.append(cv);
    this.dtStart = this.frame.calendarStart.getTime();
    this.dtEnd = this.frame.calendarEnd.getTime();
  }

  resize() {
    this.width = this.frame.width - this.frame.calendarLeft;
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
          const holiday = this.frame.holidays.filter(
            (d) => d.getTime() === dt.getTime()
          );
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

        this.drawTickets(ctx, this.frame.tickets, 0);

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
      calbody.style.left = `${this.posX + this.frame.calendarLeft}px`;
    }
  }

  scrollV(y: number) {
    this.posY = -y;
    this.draw();
  }
}
