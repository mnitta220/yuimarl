import isSameOrAfter from "dayjs/plugin/isSameOrAfter";
import {
  CALENDAR_MIN,
  LINE_HEIGHT,
  DAY_WIDTH,
  CANVAS_FONT,
  SCROLL_BAR_WIDTH,
} from "./common";
import GanttFrame from "./ganttFrame";
import dayjs from "dayjs";

dayjs.extend(isSameOrAfter);

// カレンダーヘッダー
export default class CalendarHeader {
  id = "calhead";
  width = 0;
  pos = 0;
  dtpos = 0;

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
    cv.style.height = `${this.frame.headerHeight}px`;
    frag.append(cv);

    let line = document.createElement("div");
    line.className = "line";
    line.style.top = `${this.frame.headerHeight}px`;
    line.style.left = `${this.pos + this.frame.calendarLeft}px`;
    line.style.width = `${this.width}px`;
    line.style.height = "1px";
    frag.append(line);
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
      const dtTop = this.frame.useIteration
        ? LINE_HEIGHT + LINE_HEIGHT
        : LINE_HEIGHT;
      const ctx = cnvs.getContext("2d");
      if (ctx) {
        ctx.save();
        // カレンダーヘッダー横線
        ctx.fillStyle = "#82a4c1";
        ctx.fillRect(0, LINE_HEIGHT, this.width, 1);
        ctx.fill();
        if (this.frame.useIteration) {
          ctx.fillRect(0, LINE_HEIGHT + LINE_HEIGHT, this.width, 1);
          ctx.fill();
        }

        let dt = this.frame.calendarStart.clone();
        let x = 0;
        let first = true;
        ctx.font = CANVAS_FONT;
        ctx.textBaseline = "bottom";
        ctx.textAlign = "left";
        let iteration = this.frame.iterationNo;
        let it = -1;
        let iter = 7;
        switch (this.frame.iterationUnit) {
          case "w2":
            iter = 14;
            break;
          case "w3":
            iter = 21;
            break;
          case "w4":
            iter = 28;
            break;
        }
        if (this.frame.useIteration && this.frame.iterationStart) {
          let diff = this.frame.iterationStart.diff(dt, "day");
          if (diff < 0) {
            it = (-diff % iter) - 1;
            iteration += Math.floor(-diff / iter);
          }
        }

        while (this.frame.calendarEnd.isSameOrAfter(dt)) {
          if (this.frame.useIteration && this.frame.iterationStart) {
            if (dt.isSameOrAfter(this.frame.iterationStart)) {
              it++;
              if (it >= iter) {
                it = 0;
                iteration++;
              }
            }
            if (x < this.dtpos - DAY_WIDTH) {
              x += DAY_WIDTH;
              dt = dt.add(1, "day");
              first = false;
              continue;
            }
            if (x > this.width + this.dtpos) break;

            if (it === 0) {
              ctx.fillStyle = "#000";
              ctx.fillText(`${iteration}`, x + 3 - this.dtpos, LINE_HEIGHT - 3);
              if (!first) {
                ctx.fillStyle = "#bdcede";
                ctx.fillRect(x - this.dtpos, 0, 1, LINE_HEIGHT);
                ctx.fill();
              }
            }
          } else {
            if (x < this.dtpos - DAY_WIDTH) {
              x += DAY_WIDTH;
              dt = dt.add(1, "day");
              first = false;
              continue;
            }
            if (x > this.width + this.dtpos) break;
          }

          if (first) {
            first = false;
            if (dt.date() == 1 || dt.date() < 25) {
              ctx.fillStyle = "#000";
              ctx.fillText(
                `${dt.format("YYYY/M")}`,
                x + 3 - this.dtpos,
                this.frame.useIteration
                  ? LINE_HEIGHT + LINE_HEIGHT - 3
                  : LINE_HEIGHT - 3
              );
            }
          } else if (dt.date() === 1) {
            ctx.fillStyle = "#000";
            ctx.fillText(
              `${dt.format("YYYY/M")}`,
              x + 3 - this.dtpos,
              this.frame.useIteration
                ? LINE_HEIGHT + LINE_HEIGHT - 3
                : LINE_HEIGHT - 3
            );
            ctx.fillStyle = "#bdcede";
            ctx.fillRect(
              x - this.dtpos,
              this.frame.useIteration ? LINE_HEIGHT : 0,
              1,
              LINE_HEIGHT
            );
            ctx.fill();
          }

          const holiday = this.frame.holidays.filter((d) => d.isSame(dt));
          if (holiday.length > 0) {
            ctx.fillStyle = "#f00";
          } else {
            switch (dt.day()) {
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
          ctx.fillText(
            `${dt.date()}`,
            x + 3 - this.dtpos,
            this.frame.headerHeight - 3
          );
          x += DAY_WIDTH;
          dt = dt.add(1, "day");
          // 日付区切り線
          ctx.fillStyle = "#bdcede";
          ctx.fillRect(x - this.dtpos, dtTop + 1, 1, LINE_HEIGHT);
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
    this.dtpos = Math.floor(
      (x * this.frame.calendarTotalWidth) /
        (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH)
    );
    this.draw();
    this.frame.calendarBody.dtpos = this.dtpos;
    this.frame.calendarBody.draw();
  }
}
