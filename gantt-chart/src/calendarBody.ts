import dayjs from "dayjs";
import isSameOrAfter from "dayjs/plugin/isSameOrAfter";
import {
  SCROLL_BAR_WIDTH,
  CALENDAR_MIN,
  DAY_WIDTH,
  DAY_MILISEC,
  TICKET_HEIGHT,
  GanttTicket,
} from "./common";
import GanttFrame from "./ganttFrame";

dayjs.extend(isSameOrAfter);

const BLUE1 = "#57f";
const BLUE2 = "#9bf";
const BLUE3 = "#68f";
const BLUE4 = "#8af";
const BLUE5 = "#bdf";
const RED1 = "#f66";
const RED2 = "#faa";
const RED3 = "#f77";
const RED4 = "#f99";
const RED5 = "#fcc";

// カレンダーボディ
export default class CalendarBody {
  id = "calbody";
  width = 0;
  posX = 0;
  posY = 0;
  dtpos = 0;
  nowpos = 0;

  constructor(private frame: GanttFrame) {}

  // カレンダーボディを構築する
  build(frag: DocumentFragment) {
    const cv = document.createElement("canvas");
    this.width = this.frame.width - this.frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const height =
      this.frame.height - this.frame.headerHeight - SCROLL_BAR_WIDTH;
    cv.id = this.id;
    cv.className = "gantt-body";
    cv.style.top = `${this.frame.headerHeight + 1}px`;
    cv.style.left = `${this.posX + this.frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${height}px`;
    frag.append(cv);
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

        this.nowpos = Math.floor(
          (this.frame.getNow().diff(this.frame.calendarStart) / DAY_MILISEC) *
            DAY_WIDTH
        );
        let dt = this.frame.calendarStart.clone();
        let x = 0;
        while (this.frame.calendarEnd.isSameOrAfter(dt)) {
          if (x < this.dtpos - DAY_WIDTH) {
            x += DAY_WIDTH;
            dt = dt.add(1, "day");
            continue;
          }
          if (x > this.width + this.dtpos) break;

          let dayoff = false;
          const holiday = this.frame.holidays.filter((d) => d.isSame(dt));
          if (holiday.length > 0) {
            dayoff = true;
          } else {
            switch (dt.day()) {
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

          dt = dt.add(1, "day");
          x += DAY_WIDTH;
          if (x > width + this.dtpos) break;
          // 日付区切り線
          ctx.fillStyle = "#bdcede";
          ctx.fillRect(x - this.dtpos, 0, 1, height);
          ctx.fill();
        }

        for (let t of this.frame.lines) {
          this.drawTicket(ctx, t);
        }

        // 現在線
        ctx.fillStyle = "#0b0";
        for (let y = 0; y < height; y += 8) {
          ctx.fillRect(this.nowpos - this.dtpos, y, 2, 4);
          ctx.fill();
        }

        if (this.frame.ganttRow.y1 != -1) {
          if (this.frame.moving) {
            ctx.fillStyle = "#fffdef";
            ctx.fillRect(
              0,
              this.frame.ganttRow.y1 + this.posY + this.frame.diffY,
              width,
              TICKET_HEIGHT
            );
            ctx.fill();

            if (this.frame.movingTicket) {
              this.drawMovingTicket(
                ctx,
                this.frame.movingTicket,
                this.frame.ganttRow.y1 + this.posY + this.frame.diffY
              );
            }
          }

          ctx.fillStyle = "#f6f";
          ctx.fillRect(
            0,
            this.frame.ganttRow.y1 + this.posY + this.frame.diffY,
            width,
            1
          );
          ctx.fill();
          ctx.fillRect(
            0,
            this.frame.ganttRow.y2 + this.posY + this.frame.diffY,
            width,
            1
          );
          ctx.fill();
        }

        ctx.restore();
      }
    }
  }

  // チケットを描画する
  drawTicket(ctx: CanvasRenderingContext2D, ticket: GanttTicket) {
    if (this.frame.moving && ticket.id === this.frame.movingTicket?.id) return;
    let y1 = ticket.y1;
    let y2 = ticket.y2;

    if (
      this.frame.moving &&
      this.frame.movingTicket &&
      this.frame.movingLine !== -1 &&
      this.frame.hoverLine !== -1
    ) {
      if (this.frame.diffY < 0) {
        // 上に移動
        if (ticket.line === this.frame.hoverLine) {
          let l = this.frame.movingLine - this.frame.hoverLine;
          let d = -this.frame.diffY;
          let p = d - (l - 1) * TICKET_HEIGHT;
          if (p > 10) {
            y1 += TICKET_HEIGHT;
            y2 += TICKET_HEIGHT;
          }
        } else if (
          ticket.line > this.frame.hoverLine &&
          ticket.line < this.frame.movingLine
        ) {
          y1 += TICKET_HEIGHT;
          y2 += TICKET_HEIGHT;
        }
      } else {
        // 下に移動
        if (ticket.line === this.frame.hoverLine) {
          let l = this.frame.hoverLine - this.frame.movingLine;
          let d = this.frame.diffY;
          let p = d - (l - 1) * TICKET_HEIGHT;
          if (p > 10) {
            y1 -= TICKET_HEIGHT;
            y2 -= TICKET_HEIGHT;
          }
        } else if (
          ticket.line < this.frame.hoverLine &&
          ticket.line > this.frame.movingLine
        ) {
          y1 -= TICKET_HEIGHT;
          y2 -= TICKET_HEIGHT;
        }
      }
    }

    // チケット開始日/終了日
    if (ticket.start_date) {
      if (ticket.end_date) {
        let start_date = ticket.start_date;
        let end_date = ticket.end_date;
        if (start_date > end_date) {
          [start_date, end_date] = [end_date, start_date];
        }
        let x1 = Math.floor(
          dayjs(start_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        x1 = x1 * DAY_WIDTH - this.dtpos;
        let x2 = Math.floor(
          dayjs(end_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        x2 = (x2 + 1) * DAY_WIDTH - this.dtpos;
        if (ticket.progress === 0) {
          const delay = this.frame.delayRed && x1 < this.nowpos - this.dtpos;
          ctx.fillStyle = delay ? RED2 : BLUE2;
          ctx.fillRect(x1, y1 + 8 + this.posY, x2 - x1, 6);
          ctx.fill();
        } else if (ticket.progress === 100) {
          ctx.fillStyle = BLUE1;
          ctx.fillRect(x1, y1 + 8 + this.posY, x2 - x1, 6);
          ctx.fill();
        } else {
          const x3 = Math.floor(((x2 - x1) * ticket.progress) / 100);
          const delay =
            this.frame.delayRed && x1 + x3 < this.nowpos - this.dtpos;
          ctx.fillStyle = delay ? RED2 : BLUE2;
          ctx.fillRect(x1, y1 + 8 + this.posY, x2 - x1, 6);
          ctx.fill();
          ctx.fillStyle = delay ? RED1 : BLUE1;
          ctx.fillRect(x1, y1 + 8 + this.posY, x3, 6);
          ctx.fill();
        }
      } else {
        let x1 = Math.floor(
          dayjs(ticket.start_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        x1 = x1 * DAY_WIDTH - this.dtpos;
        const delay =
          this.frame.delayRed &&
          x1 < this.nowpos - this.dtpos &&
          ticket.progress === 0;
        ctx.fillStyle = delay ? RED1 : BLUE1;
        ctx.fillRect(x1, y1 + 8 + this.posY, 12, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED3 : BLUE3;
        x1 += 12;
        ctx.fillRect(x1, y1 + 8 + this.posY, 7, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED4 : BLUE4;
        x1 += 7;
        ctx.fillRect(x1, y1 + 8 + this.posY, 6, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED5 : BLUE5;
        x1 += 6;
        ctx.fillRect(x1, y1 + 8 + this.posY, 5, 6);
        ctx.fill();
      }
    } else {
      if (ticket.end_date) {
        let x2 = Math.floor(
          dayjs(ticket.end_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        x2 = (x2 + 1) * DAY_WIDTH - this.dtpos;
        const delay =
          this.frame.delayRed &&
          x2 < this.nowpos - this.dtpos &&
          ticket.progress !== 100;
        x2 -= 12;
        ctx.fillStyle = delay ? RED1 : BLUE1;
        ctx.fillRect(x2, y1 + 8 + this.posY, 12, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED3 : BLUE3;
        x2 -= 7;
        ctx.fillRect(x2, y1 + 8 + this.posY, 7, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED4 : BLUE4;
        x2 -= 6;
        ctx.fillRect(x2, y1 + 8 + this.posY, 6, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED5 : BLUE5;
        x2 -= 5;
        ctx.fillRect(x2, y1 + 8 + this.posY, 5, 6);
        ctx.fill();
      }
    }

    ctx.fillStyle = "#e5ebf2";
    // 区切り線
    ctx.fillRect(0, y2 + this.posY, this.width, 1);
    ctx.fill();
  }

  // チケットを描画する
  drawMovingTicket(
    ctx: CanvasRenderingContext2D,
    ticket: GanttTicket,
    y: number
  ) {
    let y2 = y;

    // チケット開始日/終了日
    if (ticket.start_date) {
      if (ticket.end_date) {
        let start_date = ticket.start_date;
        let end_date = ticket.end_date;
        if (start_date > end_date) {
          [start_date, end_date] = [end_date, start_date];
        }
        let x1 = Math.floor(
          dayjs(start_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        x1 = x1 * DAY_WIDTH - this.dtpos;
        let x2 = Math.floor(
          dayjs(end_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        x2 = (x2 + 1) * DAY_WIDTH - this.dtpos;
        if (ticket.progress === 0) {
          const delay = this.frame.delayRed && x1 < this.nowpos - this.dtpos;
          ctx.fillStyle = delay ? RED2 : BLUE2;
          ctx.fillRect(x1, y2 + 8, x2 - x1, 6);
          ctx.fill();
        } else if (ticket.progress === 100) {
          ctx.fillStyle = BLUE1;
          ctx.fillRect(x1, y2 + 8, x2 - x1, 6);
          ctx.fill();
        } else {
          const x3 = Math.floor(((x2 - x1) * ticket.progress) / 100);
          const delay =
            this.frame.delayRed && x1 + x3 < this.nowpos - this.dtpos;
          ctx.fillStyle = delay ? RED2 : BLUE2;
          ctx.fillRect(x1, y2 + 8, x2 - x1, 6);
          ctx.fill();
          ctx.fillStyle = delay ? RED1 : BLUE1;
          ctx.fillRect(x1, y2 + 8, x3, 6);
          ctx.fill();
        }
      } else {
        let x1 = Math.floor(
          dayjs(ticket.start_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        const delay = this.frame.delayRed && x1 < this.nowpos - this.dtpos;
        x1 = x1 * DAY_WIDTH - this.dtpos;
        ctx.fillStyle = delay ? RED1 : BLUE1;
        ctx.fillRect(x1, y2 + 8, 12, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED3 : BLUE3;
        x1 += 12;
        ctx.fillRect(x1, y2 + 8, 7, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED4 : BLUE4;
        x1 += 7;
        ctx.fillRect(x1, y2 + 8, 6, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED5 : BLUE5;
        x1 += 6;
        ctx.fillRect(x1, y2 + 8, 5, 6);
        ctx.fill();
      }
    } else {
      if (ticket.end_date) {
        let x2 = Math.floor(
          dayjs(ticket.end_date).diff(this.frame.calendarStart) / DAY_MILISEC
        );
        x2 = (x2 + 1) * DAY_WIDTH - this.dtpos;
        const delay = this.frame.delayRed && x2 < this.nowpos - this.dtpos;
        ctx.fillStyle = delay ? RED1 : BLUE1;
        x2 -= 12;
        ctx.fillRect(x2, y2 + 8, 12, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED3 : BLUE3;
        x2 -= 7;
        ctx.fillRect(x2, y2 + 8, 7, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED4 : BLUE4;
        x2 -= 6;
        ctx.fillRect(x2, y2 + 8, 6, 6);
        ctx.fill();
        ctx.fillStyle = delay ? RED5 : BLUE5;
        x2 -= 5;
        ctx.fillRect(x2, y2 + 8, 5, 6);
        ctx.fill();
      }
    }

    ctx.fillStyle = "#e5ebf2";
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
