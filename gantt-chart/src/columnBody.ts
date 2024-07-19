import dayjs from "dayjs";
import {
  HEADER_HEIGHT,
  SCROLL_BAR_WIDTH,
  TICKET_HEIGHT,
  GanttTicket,
  TicketModalResult,
} from "./common";
import GanttFrame from "./ganttFrame";

// カラムボディ
export default class ColumnBody {
  id = "colbody";
  width = 0;
  height = 0;
  posX = 0;
  posY = 0;
  clicked = "";
  //startX = 0;
  //startY = 0;

  constructor(private frame: GanttFrame) {}

  // カラムボディを構築する
  build(frag: DocumentFragment) {
    const body = document.createElement("canvas");
    this.height = this.frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
    this.frame.ticketsFrameHeight = this.height;
    body.id = this.id;
    this.width = this.frame.calendarLeft;
    body.className = "gantt-body";
    body.style.top = `${HEADER_HEIGHT + 1}px`;
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
        frame.columnBody.mouseDown(e.layerX, e.layerY);
      });
      colbody.addEventListener("touchstart", function (e: TouchEvent) {
        e.preventDefault();
        frame.columnBody.mouseDown(
          e.touches[0].pageX - ganttframe.offsetLeft - colbody.offsetLeft,
          e.touches[0].pageY - ganttframe.offsetTop - colbody.offsetTop
        );
      });
      colbody.addEventListener("mousemove", function (e: MouseEvent) {
        e.preventDefault();
        frame.columnBody.mouseMove(e.layerY);
      });
      colbody.addEventListener("touchmove", function (e: TouchEvent) {
        e.preventDefault();
        frame.columnBody.mouseMove(
          e.touches[0].pageY - ganttframe.offsetTop - colbody.offsetTop
        );
      });
      colbody.addEventListener("mouseup", function (e: MouseEvent) {
        e.preventDefault();
        frame.columnBody.mouseUp();
      });
      colbody.addEventListener("mouseleave", function (e: MouseEvent) {
        e.preventDefault();
        frame.columnBody.mouseLeave();
      });
      colbody.addEventListener("touchend", function (e: TouchEvent) {
        e.preventDefault();
        frame.columnBody.mouseUp();
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

        if (this.frame.moving && this.frame.ganttRow.y1 != -1) {
          ctx.fillStyle = "#fffdef";
          ctx.fillRect(
            0,
            this.frame.ganttRow.y1 + this.posY + this.frame.diffY,
            width,
            TICKET_HEIGHT
          );
          ctx.fill();
          if (this.frame.movingTicket) {
            this.drawTickets(
              ctx,
              [this.frame.movingTicket],
              this.frame.movingLevel,
              this.frame.ganttRow.y1 + this.posY + this.frame.diffY,
              true
            );
          }
        }

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

        if (this.frame.ganttRow.y1 != -1) {
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
          ctx.fillRect(
            0,
            this.frame.ganttRow.y1 + this.posY + this.frame.diffY,
            1,
            TICKET_HEIGHT
          );
          ctx.fill();
        }

        ctx.restore();
      }
    }
  }

  drawTickets(
    ctx: CanvasRenderingContext2D,
    ts: GanttTicket[],
    level: number,
    y: number,
    moving: boolean = false
  ): number {
    let y1 = y;
    for (let t of ts) {
      if (t.progress === 100 && this.frame.showDone === false) {
        continue;
      }

      if (
        !this.frame.moving ||
        !this.frame.movingTicket ||
        this.frame.movingTicket != t ||
        moving
      ) {
        this.drawTicket(ctx, t, level, y1, moving);
        y1 += TICKET_HEIGHT;
        if (t.open && t.children.length > 0 && !moving) {
          y1 = this.drawTickets(ctx, t.children, level + 1, y1, moving);
        }
      } else {
        y1 += TICKET_HEIGHT;
      }
    }
    return y1;
  }

  getFrameHeight(ts: GanttTicket[], y: number): number {
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
    ticket: GanttTicket,
    level: number,
    y: number,
    moving: boolean = false
  ) {
    ctx.font = "9.5pt sans-serif";
    ctx.textBaseline = "bottom";
    let y2 = y;
    if (this.frame.moving && !moving) {
      let d = this.frame.startY + this.frame.diffY;
      if (d > y && y > this.frame.startY) {
        y2 -= TICKET_HEIGHT;
      } else if (d - TICKET_HEIGHT < y && y < this.frame.startY) {
        y2 += TICKET_HEIGHT;
      }
    }
    const y1 = y2 + 18;
    ctx.fillStyle = "#00f";
    // ID
    ctx.fillText(ticket.id_disp, 3, y1 + this.posY);
    ctx.fillStyle = "#808080";
    // チケット□
    let x = this.frame.cols[0].width + level * 12 + 6;
    ctx.fillRect(x, y2 + 7 + this.posY, 1, 8);
    ctx.fill();
    ctx.fillRect(x, y2 + 7 + this.posY, 8, 1);
    ctx.fill();
    ctx.fillRect(x, y2 + 15 + this.posY, 8, 1);
    ctx.fill();
    ctx.fillRect(x + 8, y2 + 7 + this.posY, 1, 9);
    ctx.fill();
    ctx.fillStyle = "#000";

    if (ticket.children.length > 0) {
      ctx.fillRect(x + 2, y2 + 11 + this.posY, 5, 1);
      ctx.fill();
      if (!ticket.open || moving) {
        ctx.fillRect(x + 4, y2 + 9 + this.posY, 1, 5);
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
    if (ticket.start_date) {
      ctx.fillText(
        dayjs(ticket.start_date).format("YY/MM/DD"),
        x,
        y1 + this.posY
      );
    }
    x += this.frame.cols[2].width;
    // 終了日
    if (ticket.end_date) {
      ctx.fillText(
        dayjs(ticket.end_date).format("YY/MM/DD"),
        x,
        y1 + this.posY
      );
    }
    x += this.frame.cols[3].width + this.frame.cols[4].width - 6;
    // 進捗率
    const t1 = `${ticket.progress}`;
    const m1 = ctx.measureText(t1).width;
    ctx.fillText(t1, x - m1, y1 + this.posY);

    ctx.fillStyle = "#e5ebf2";
    // 区切り線
    ctx.fillRect(0, y2 + TICKET_HEIGHT + this.posY, this.width, 1);
    ctx.fill();
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
    try {
      this.frame.clicked = true;
      //this.startX = x;
      this.frame.startY = y;
      this.clicked = "";
      this.clickTickets(this.frame.tickets, x, y, 0, 0);

      if (this.clicked) {
        fetch(`/api/ticket/${this.clicked}`, {
          method: "GET",
        })
          .then((response) => response.json())
          .then((data: TicketModalResult) => {
            if (data.result) {
              this.frame.ticketModal.show(data);
            } else {
              window.alert(`エラーが発生しました。: ${data.message}`);
            }
          })
          .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
      } else {
        this.frame.draw();
      }
    } catch (e) {
      window.alert(`エラーが発生しました。: ${e}`);
    }
  }

  mouseMove(y: number) {
    if (!this.frame.clicked) return;
    this.frame.moving = true;
    try {
      this.frame.diffY = y - this.frame.startY;
      this.frame.draw();
    } catch (e) {
      window.alert(`エラーが発生しました。: ${e}`);
    }
  }

  mouseUp() {
    try {
      this.frame.clicked = false;
      this.frame.moving = false;
      this.frame.diffY = 0;
      this.frame.movingTicket = null;
      this.frame.draw();
    } catch (e) {
      window.alert(`エラーが発生しました。: ${e}`);
    }
  }

  mouseLeave() {
    try {
      this.frame.clicked = false;
      this.frame.moving = false;
      this.frame.diffY = 0;
      this.frame.movingTicket = null;
      this.frame.draw();
    } catch (e) {
      window.alert(`エラーが発生しました。: ${e}`);
    }
  }

  clickTickets(
    ts: GanttTicket[],
    clickx: number,
    clicky: number,
    level: number,
    y: number
  ): number {
    let y1 = y;
    for (let t of ts) {
      if (t.progress === 100 && this.frame.showDone === false) {
        continue;
      }

      if (this.clickTicket(t, clickx, clicky, level, y1)) {
        return y1;
      }
      y1 += TICKET_HEIGHT;
      if (t.open && t.children.length > 0) {
        let y2 = this.clickTickets(t.children, clickx, clicky, level + 1, y1);
        if (this.clicked) {
          return y2;
        }
        y1 = y2;
      }
    }
    return y1;
  }

  clickTicket(
    ticket: GanttTicket,
    clickx: number,
    clicky: number,
    level: number,
    y: number
  ): boolean {
    if (clicky < y + this.posY || clicky > y + TICKET_HEIGHT + this.posY) {
      return false;
    }
    this.frame.ganttRow.y1 = y;
    this.frame.ganttRow.y2 = y + TICKET_HEIGHT;
    this.frame.movingTicket = ticket;
    this.frame.movingLevel = level;

    if (clickx < this.frame.cols[0].width) {
      // IDをクリックした
      this.clicked = ticket.id;
      return true;
    }
    if (clickx < this.frame.cols[0].width + this.frame.cols[1].width) {
      const x = this.frame.cols[0].width + level * 12 + 2;
      if (clickx > x && clickx < x + 16) {
        // チケット名の□をクリックした
        ticket.open = !ticket.open;
        this.frame.nodeOpenClose(ticket.id, ticket.open);
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
