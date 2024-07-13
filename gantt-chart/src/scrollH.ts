import { SCROLL_BAR_WIDTH } from "./common";
import GanttFrame from "./ganttFrame";

// 横スクロールバー
export default class ScrollH {
  id = "scrollh";
  width = 0;
  barWidth = 0;
  height = SCROLL_BAR_WIDTH;
  moving = false;
  startX = 0;
  pos = 0;
  startPos = 0;

  constructor(private frame: GanttFrame) {}

  // イベントハンドラを登録する
  handler() {
    const scrollh = document.querySelector<HTMLCanvasElement>(`#${this.id}`);
    const frame = this.frame;
    if (scrollh) {
      scrollh.addEventListener("mousedown", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseDownScrollH(e.pageX - scrollh.offsetLeft);
      });
      scrollh.addEventListener("touchstart", function (e: TouchEvent) {
        e.preventDefault();
        frame.mouseDownScrollH(e.touches[0].pageX - scrollh.offsetLeft);
      });
      scrollh.addEventListener("mousemove", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseMoveScrollH(e.pageX - scrollh.offsetLeft);
      });
      scrollh.addEventListener("touchmove", function (e: TouchEvent) {
        e.preventDefault();
        frame.mouseMoveScrollH(e.touches[0].pageX - scrollh.offsetLeft);
      });
      scrollh.addEventListener("mouseup", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollH();
      });
      scrollh.addEventListener("mouseleave", function (e: MouseEvent) {
        e.preventDefault();
        frame.mouseUpScrollH();
      });
      scrollh.addEventListener("touchend", function (e: TouchEvent) {
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
      if (this.width > this.frame.schThreshold) return;
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
          this.frame.schThreshold;
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
    this.frame.scrollH(this.pos);
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
    this.frame.scrollH(this.pos);
    this.draw();
  }

  mouseUpScrollH() {
    this.moving = false;
  }
}
