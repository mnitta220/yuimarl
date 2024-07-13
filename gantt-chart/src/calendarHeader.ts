import {
  CALENDAR_MIN,
  HEADER_HEIGHT,
  LINE_HEIGHT,
  DAY_WIDTH,
  DAY_MILISEC,
  SCROLL_BAR_WIDTH,
} from "./common";
import GanttFrame from "./ganttFrame";

// カレンダーヘッダー
export default class CalendarHeader {
  id = "calhead";
  width = 0;
  pos = 0;
  dtpos = 0;
  dtStart = 0;
  dtEnd = 0;

  constructor(private frame: GanttFrame) {}

  // カレンダーヘッダーを構築する
  build(frag: DocumentFragment) {
    const cv = document.createElement("canvas");
    this.width = this.frame.width - this.frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    cv.id = this.id;
    cv.className = "header";
    cv.style.top = `0px`;
    cv.style.left = `${this.pos + this.frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${HEADER_HEIGHT + 1}px`;
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
        let iteration = 1;
        let iter = -1;

        while (dt.getTime() <= this.dtEnd) {
          iter++;
          if (iter > 13) {
            iter = 0;
            iteration++;
          }
          if (x < this.dtpos - DAY_WIDTH) {
            x += DAY_WIDTH;
            dt.setTime(dt.getTime() + DAY_MILISEC);
            first = false;
            continue;
          }
          if (x > this.width + this.dtpos) break;

          const date = dt.getDate();

          if (iter === 0) {
            ctx.fillStyle = "#000";
            ctx.fillText(`${iteration}`, x + 3 - this.dtpos, LINE_HEIGHT - 3);
            if (!first) {
              ctx.fillStyle = "#bdcede";
              ctx.fillRect(x - this.dtpos, 0, 1, LINE_HEIGHT);
              ctx.fill();
            }
          }

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

          const holiday = this.frame.holidays.filter(
            (d) => d.getTime() === dt.getTime()
          );
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
      calhead.style.left = `${this.pos + this.frame.calendarLeft}px`;
    }
  }

  scroll(x: number) {
    this.dtpos =
      (x * this.frame.calendarTotalWidth) /
      (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
    this.draw();
    this.frame.calendarBody.dtpos = this.dtpos;
    this.frame.calendarBody.draw();
  }
}
