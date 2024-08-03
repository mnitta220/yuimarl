import { SCROLL_BAR_WIDTH, CALENDAR_MIN } from "./common";
import GanttFrame from "./ganttFrame";

// カレンダースクロールバー
export default class CalendarScroll {
  id = "calscroll";
  width = 0;
  pos = 0;
  barWidth = 0;
  height = SCROLL_BAR_WIDTH;
  moving = false;
  startX = 0;
  barpos = 0;
  startPos = 0;

  constructor(private frame: GanttFrame) {}

  // スクロールバーを構築する
  build(frag: DocumentFragment) {
    this.width = this.frame.width - this.frame.calendarLeft;
    if (this.width < CALENDAR_MIN) {
      this.width = CALENDAR_MIN;
    }
    const cv = document.createElement("canvas");
    cv.id = this.id;
    cv.className = "scroll-bar";
    cv.style.top = `${this.frame.height - SCROLL_BAR_WIDTH}px`;
    cv.style.left = `${this.pos + this.frame.calendarLeft}px`;
    cv.style.width = `${this.width}px`;
    cv.style.height = `${this.height}px`;
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
    this.scrollH(0);
    this.barpos = 0;
  }

  // イベントハンドラを登録する
  handler() {
    const ganttframe = document.querySelector<HTMLCanvasElement>(`#ganttframe`);
    const calscroll = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    const frame = this.frame;
    if (ganttframe && calscroll) {
      calscroll.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseDownCalScroll(e.layerX);
      });
      calscroll.addEventListener("touchstart", function (e: TouchEvent) {
        e.preventDefault();
        frame.mouseDownCalScroll(
          e.touches[0].pageX - ganttframe.offsetLeft - calscroll.offsetLeft
        );
      });
      calscroll.addEventListener("mousemove", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseMoveCalScroll(e.layerX);
      });
      calscroll.addEventListener("touchmove", function (e: TouchEvent) {
        e.preventDefault();
        frame.mouseMoveCalScroll(
          e.touches[0].pageX - ganttframe.offsetLeft - calscroll.offsetLeft
        );
      });
      calscroll.addEventListener("mouseup", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpCalScroll();
      });
      calscroll.addEventListener("mouseleave", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpCalScroll();
      });
      calscroll.addEventListener("touchend", function (e: TouchEvent) {
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
          this.frame.calendarTotalWidth;
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
      calscroll.style.left = `${this.pos + this.frame.calendarLeft}px`;
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

    this.frame.calendarHeader.scroll(this.barpos);
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
    this.frame.calendarHeader.scroll(this.barpos);
    this.draw();
  }

  mouseUpCalScroll() {
    this.moving = false;
  }
}
