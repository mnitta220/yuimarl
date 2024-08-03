import { SCROLL_BAR_WIDTH } from "./common";
import GanttFrame from "./ganttFrame";

// 縦スクロールバー
export default class ScrollV {
  id = "scrollv";
  width = SCROLL_BAR_WIDTH;
  height = 0;
  barHeight = 0;
  moving = false;
  visible = false;
  startY = 0;
  pos = 0;
  startPos = 0;

  constructor(private frame: GanttFrame) {}

  // イベントハンドラを登録する
  handler() {
    const scrollv = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    const frame = this.frame;
    if (scrollv) {
      scrollv.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseDownScrollV(e.pageY - scrollv.offsetTop);
      });
      scrollv.addEventListener("touchstart", function (e: TouchEvent) {
        e.preventDefault();
        frame.mouseDownScrollV(e.touches[0].pageY - scrollv.offsetTop);
      });
      scrollv.addEventListener("mousemove", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseMoveScrollV(e.pageY - scrollv.offsetTop);
      });
      scrollv.addEventListener("touchmove", function (e: TouchEvent) {
        e.preventDefault();
        frame.mouseMoveScrollV(e.touches[0].pageY - scrollv.offsetTop);
      });
      scrollv.addEventListener("mouseup", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollV();
      });
      scrollv.addEventListener("mouseleave", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollV();
      });
      scrollv.addEventListener("touchend", function (e: TouchEvent) {
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
        if (this.frame.ticketsFrameHeight < this.frame.ticketsTotalHeight) {
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
              this.frame.ticketsFrameHeight) /
            this.frame.ticketsTotalHeight;
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
    this.frame.scrollV(
      (this.frame.ticketsTotalHeight - this.frame.ticketsFrameHeight) * percent
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
    this.frame.scrollV(
      (this.frame.ticketsTotalHeight - this.frame.ticketsFrameHeight) * percent
    );
    this.draw();
  }

  mouseUpScrollV() {
    this.moving = false;
  }
}
