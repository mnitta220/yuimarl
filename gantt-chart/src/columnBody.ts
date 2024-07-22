import dayjs from "dayjs";
import {
  HEADER_HEIGHT,
  SCROLL_BAR_WIDTH,
  TICKET_HEIGHT,
  GanttTicket,
  TicketModalResult,
} from "./common";
import GanttFrame from "./ganttFrame";

/*
enum MovingPart {
  TOP = 1,
  MIDDLE = 2,
  BOTTOM = 3,
  NONE = 0,
}
*/

// カラムボディ
export default class ColumnBody {
  id = "colbody";
  width = 0;
  height = 0;
  posX = 0;
  posY = 0;
  //cursorTicket: GanttTicket | null = null;
  //targetTicket: GanttTicket | null = null;
  //cursorIndex = -1;
  //targetIndex = -1;
  clickedId = "";
  movingLevel = 0;
  upper: GanttTicket | null = null;
  lower: GanttTicket | null = null;
  //movingPart: MovingPart = MovingPart.NONE; // ドラッグ位置 1:上, 2:中, 3:下
  //movingPartSave: MovingPart = MovingPart.NONE; // ドラッグ位置 1:上, 2:中, 3:下

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
        frame.columnBody.mouseMove(e.layerX, e.layerY);
      });
      colbody.addEventListener("touchmove", function (e: TouchEvent) {
        e.preventDefault();
        frame.columnBody.mouseMove(
          e.touches[0].pageX - ganttframe.offsetLeft - colbody.offsetLeft,
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

        /*
        this.frame.ticketsTotalHeight = this.drawTickets(
          ctx,
          this.frame.tickets,
          0
        );
        */
        for (let t of this.frame.lines) {
          this.drawTicket(ctx, t);
        }

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
            /*
            this.drawTickets(
              ctx,
              [this.frame.movingTicket],
              this.frame.ganttRow.y1 + this.posY + this.frame.diffY,
              true
            );
            */
            this.drawMovingTicket(
              ctx,
              this.frame.movingTicket,
              this.frame.ganttRow.y1 + this.posY + this.frame.diffY
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

  /*
  drawTickets(
    ctx: CanvasRenderingContext2D,
    ts: GanttTicket[],
    //level: number,
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
        this.drawTicket(ctx, t, y1, moving);
        y1 += TICKET_HEIGHT;
        if (t.open && t.children.length > 0 && !moving) {
          y1 = this.drawTickets(ctx, t.children, y1, moving);
        }
      } else {
        y1 += TICKET_HEIGHT;
      }
    }
    return y1;
  }
  */

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

  /*
  // チケットを描画する
  drawTicket(
    ctx: CanvasRenderingContext2D,
    ticket: GanttTicket,
    //level: number,
    y: number,
    moving: boolean = false
  ) {
    ctx.font = "9.5pt sans-serif";
    ctx.textBaseline = "bottom";
    let y2 = y;
    if (this.frame.moving && !moving) {
      let d = this.frame.startY + this.frame.diffY;
      if (d > y && y > this.frame.startY) {
        this.frame.dropPos = ticket.pos ?? "";
        y2 -= TICKET_HEIGHT;
      } else if (d - TICKET_HEIGHT < y && y < this.frame.startY) {
        if (!this.frame.dropPos) {
          this.frame.dropPos = ticket.pos ?? "";
        }
        y2 += TICKET_HEIGHT;
      }
    }
    const y1 = y2 + 18;
    ctx.fillStyle = "#00f";
    // ID
    ctx.fillText(ticket.id_disp, 3, y1 + this.posY);
    ctx.fillStyle = "#808080";
    // チケット□
    let x = this.frame.cols[0].width + ticket.level * 12 + 6;
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
  */

  // チケットを描画する
  drawTicket(ctx: CanvasRenderingContext2D, ticket: GanttTicket) {
    if (this.frame.moving && ticket.id === this.frame.movingTicket?.id) return;
    ctx.font = "9.5pt sans-serif";
    ctx.textBaseline = "bottom";
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
          //console.log(`drawTicket: ticket=${ticket.id_disp} ${l} ${d} ${p}`);
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
          //console.log(`drawTicket: ticket=${ticket.id_disp} ${l} ${d} ${p}`);
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

      /*
      if (ticket.line < this.frame.movingLine) {
        if (this.frame.movingTicket.y1 + this.frame.diffY < ticket.y2 - 10) {
          y1 += TICKET_HEIGHT;
          y2 += TICKET_HEIGHT;
        }
      } else if (ticket.line > this.frame.movingLine) {
        if (this.frame.movingTicket.y2 + this.frame.diffY > ticket.y1 + 10) {
          y1 -= TICKET_HEIGHT;
          y2 -= TICKET_HEIGHT;
        }
      }
      */
    }

    const y3 = y1 + 18;
    ctx.fillStyle = "#00f";
    // ID
    ctx.fillText(ticket.id_disp, 3, y3 + this.posY);
    ctx.fillStyle = "#808080";
    // チケット□
    let x = this.frame.cols[0].width + ticket.level * 12 + 6;
    ctx.fillRect(x, y1 + 7 + this.posY, 1, 8);
    ctx.fill();
    ctx.fillRect(x, y1 + 7 + this.posY, 8, 1);
    ctx.fill();
    ctx.fillRect(x, y1 + 15 + this.posY, 8, 1);
    ctx.fill();
    ctx.fillRect(x + 8, y1 + 7 + this.posY, 1, 9);
    ctx.fill();
    ctx.fillStyle = "#000";

    if (ticket.children.length > 0) {
      ctx.fillRect(x + 2, y1 + 11 + this.posY, 5, 1);
      ctx.fill();
      if (!ticket.open) {
        ctx.fillRect(x + 4, y1 + 9 + this.posY, 1, 5);
        ctx.fill();
      }
    }

    // チケット名
    let w = this.frame.cols[0].width + this.frame.cols[1].width - (x + 14);
    let m = ctx.measureText(ticket.name).width;
    if (m <= w) {
      ctx.fillText(ticket.name, x + 14, y3 + this.posY);
    } else {
      let s = ticket.name;
      while (ctx.measureText(`${s}…`).width > w) {
        s = s.slice(0, -1);
      }
      ctx.fillText(`${s}…`, x + 14, y3 + this.posY);
    }

    x = this.frame.cols[0].width + this.frame.cols[1].width + 3;
    // 開始日
    if (ticket.start_date) {
      ctx.fillText(
        dayjs(ticket.start_date).format("YY/MM/DD"),
        x,
        y3 + this.posY
      );
    }
    x += this.frame.cols[2].width;
    // 終了日
    if (ticket.end_date) {
      ctx.fillText(
        dayjs(ticket.end_date).format("YY/MM/DD"),
        x,
        y3 + this.posY
      );
    }
    x += this.frame.cols[3].width + this.frame.cols[4].width - 6;
    // 進捗率
    const t1 = `${ticket.progress}`;
    const m1 = ctx.measureText(t1).width;
    ctx.fillText(t1, x - m1, y3 + this.posY);

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
    ctx.font = "9.5pt sans-serif";
    ctx.textBaseline = "bottom";
    let y2 = y;
    const y1 = y2 + 18;
    ctx.fillStyle = "#00f";
    // ID
    ctx.fillText(ticket.id_disp, 3, y1 + this.posY);
    ctx.fillStyle = "#808080";
    // チケット□
    this.movingLevel = ticket.level;

    if (
      this.frame.movingLine !== -1 &&
      this.frame.hoverLine != -1 &&
      this.frame.movingTicket
    ) {
      let hover = this.frame.lines[this.frame.hoverLine];
      //let upper: GanttTicket | null = hover;
      if (this.frame.diffY < 0) {
        // 上に移動
        this.upper = hover;
        if (this.frame.lines.length > this.frame.hoverLine + 1) {
          this.lower = this.frame.lines[this.frame.hoverLine + 1];
          if (this.lower === this.frame.movingTicket) {
            if (this.frame.lines.length > this.frame.hoverLine + 2) {
              this.lower = this.frame.lines[this.frame.hoverLine + 2];
            } else {
              this.lower = null;
            }
          }
        } else {
          this.lower = null;
        }
        let l = this.frame.movingLine - this.frame.hoverLine;
        let d = -this.frame.diffY;
        let p = d - (l - 1) * TICKET_HEIGHT;
        //console.log(
        //  `drawMovingTicket: ticket=${ticket.id_disp} ${l} ${d} ${p}`
        //);
        if (p > 10) {
          if (this.frame.hoverLine === 0) {
            this.upper = null;
            if (this.frame.lines.length > 0) {
              this.lower = this.frame.lines[0];
              if (this.lower === this.frame.movingTicket) {
                if (this.frame.lines.length > 1) {
                  this.lower = this.frame.lines[1];
                } else {
                  this.lower = null;
                }
              }
            } else {
              this.lower = null;
            }
          } else {
            this.upper = this.frame.lines[this.frame.hoverLine - 1];
            this.lower = this.frame.lines[this.frame.hoverLine];
            if (this.lower === this.frame.movingTicket) {
              if (this.frame.lines.length > this.frame.hoverLine + 1) {
                this.lower = this.frame.lines[this.frame.hoverLine + 1];
              } else {
                this.lower = null;
              }
            }
          }
        }
      } else {
        // 下に移動
        if (this.frame.movingLine === 0) {
          this.upper = null;
          if (this.frame.lines.length > 0) {
            this.lower = this.frame.lines[0];
            if (this.lower === this.frame.movingTicket) {
              if (this.frame.lines.length > 1) {
                this.lower = this.frame.lines[1];
              } else {
                this.lower = null;
              }
            }
          } else {
            this.lower = null;
          }
        } else {
          this.upper = this.frame.lines[this.frame.movingLine - 1];
          this.lower = this.frame.lines[this.frame.movingLine];
          if (this.lower === this.frame.movingTicket) {
            if (this.frame.lines.length > this.frame.movingLine + 1) {
              this.lower = this.frame.lines[this.frame.movingLine + 1];
            } else {
              this.lower = null;
            }
          }
        }
        let l = this.frame.hoverLine - this.frame.movingLine;
        let d = this.frame.diffY;
        let p = d - (l - 1) * TICKET_HEIGHT;
        //console.log(
        //  `drawMovingTicket: ticket=${ticket.id_disp} ${l} ${d} ${p}`
        //);
        //console.log(`drawTicket: ticket=${ticket.id_disp} ${l} ${d} ${p}`);
        if (p > 10) {
          this.upper = hover;
          if (this.frame.lines.length > this.frame.hoverLine + 1) {
            this.lower = this.frame.lines[this.frame.hoverLine + 1];
            if (this.lower === this.frame.movingTicket) {
              if (this.frame.lines.length > this.frame.hoverLine + 2) {
                this.lower = this.frame.lines[this.frame.hoverLine + 2];
              } else {
                this.lower = null;
              }
            }
          } else {
            this.lower = null;
          }
        }
      }

      if (this.upper) {
        if (this.upper.children.length > 0 && this.upper.open) {
          this.movingLevel = this.upper.level + 1;
          if (
            this.upper.children[this.upper.children.length - 1] ===
            this.frame.movingTicket
          ) {
            if (this.frame.diffX < -12) {
              this.movingLevel =
                this.frame.movingTicket.level -
                Math.floor(-this.frame.diffX / 12);
              if (this.lower) {
                if (this.movingLevel < this.lower.level)
                  this.movingLevel = this.lower.level;
              } else {
                if (this.movingLevel < 0) this.movingLevel = 0;
              }
            }
          }
        } else {
          this.movingLevel = this.upper.level;
          if (this.frame.diffX > 12) {
            this.movingLevel++;
          } else if (this.frame.diffX < -12) {
            if (this.lower) {
              this.movingLevel = this.lower.level;
            } else {
              this.movingLevel =
                this.frame.movingTicket.level -
                Math.floor(-this.frame.diffX / 12);
              if (this.movingLevel < 0) this.movingLevel = 0;
            }
          }
        }
      } else {
        this.movingLevel = 0;
      }

      /*
      if (hover.children.length > 0 && hover.open) {
        this.movingLevel = hover.level + 1;
      } else {
        this.movingLevel = hover.level;
        if (this.frame.diffX > 12) {
          this.movingLevel++;
        }
      }
      */
    }

    let x = this.frame.cols[0].width + this.movingLevel * 12 + 6;
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
      //if (!ticket.open || moving) {
      ctx.fillRect(x + 4, y2 + 9 + this.posY, 1, 5);
      ctx.fill();
      //}
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
      this.frame.startX = x;
      this.frame.startY = y;
      this.frame.movingLine = -1;
      this.clickedId = "";
      //this.movingPartSave = MovingPart.NONE;
      //this.clickTickets(this.frame.tickets, x, y, 0);
      /*
      for (let t of this.frame.lines) {
        if (t.y1 <= y && y <= t.y2) {
          this.clickTicket(t, x);
          break;
        }
      }
      */
      for (let i = 0; i < this.frame.lines.length; i++) {
        let t = this.frame.lines[i];
        if (t.y1 <= y && y <= t.y2) {
          this.frame.movingLine = i;
          this.clickTicket(t, x);
          break;
        }
      }

      if (this.clickedId) {
        fetch(`/api/ticket/${this.clickedId}`, {
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

  mouseMove(x: number, y: number) {
    if (!this.frame.clicked) return;
    try {
      if (!this.frame.moving) {
        this.frame.resetLines(this.frame.movingTicket?.id ?? "");
        this.frame.moving = true;
      }
      this.frame.diffX = x - this.frame.startX;
      this.frame.diffY = y - this.frame.startY;

      this.frame.hoverLine = -1;
      for (let i = 0; i < this.frame.lines.length; i++) {
        let t = this.frame.lines[i];
        if (t == this.frame.movingTicket) continue;
        /*
        if (t.y1 < y && t.y2 > y) {
          this.frame.hoverLine = i;
          if (t.y1 + t.y2 > y + y) {
            --this.frame.hoverLine;
          }
          break;
        }
        */
        //*
        if (
          t.y1 < this.frame.ganttRow.y2 + this.posY + this.frame.diffY &&
          t.y2 > this.frame.ganttRow.y1 + this.posY + this.frame.diffY
        ) {
          this.frame.hoverLine = i;
          break;
        }
        //*/
        /*
        if (t.y1 < y && t.y2 > y) {
          this.frame.hoverLine = i;
          break;
        }
        */
        /*
        if (
          t.y1 < this.frame.ganttRow.y2 + this.posY + this.frame.diffY &&
          t.y2 > this.frame.ganttRow.y1 + this.posY + this.frame.diffY
        ) {
          this.frame.hoverLine = i;
          this.movingPart = MovingPart.NONE;
          if (this.frame.diffY > 14) {
            this.movingPart = MovingPart.BOTTOM;
          } else if (this.frame.diffY > 7) {
            this.movingPart = MovingPart.MIDDLE;
          } else if (this.frame.diffY > 0) {
            this.movingPart = MovingPart.TOP;
          }
          //if (this.movingPartSave === MovingPart.NONE) {
          //  this.movingPartSave = this.movingPart;
          //}
          break;
        }
        */
      }
      //console.log(`mouseMove: hoverLine=${this.frame.hoverLine}`);

      /*
      if (
        this.frame.movingLine != -1 &&
        this.frame.hoverLine !== -1 //&&
        //this.movingPart !== this.movingPartSave
      ) {
        //this.movingPartSave = this.movingPart;
        console.log(
          `movingLine=${this.frame.movingLine} hoverLine=${this.frame.hoverLine} movingPart=${this.movingPart}`
        );
        this.cursorTicket = this.frame.lines[this.frame.movingLine];
        this.targetTicket = this.frame.lines[this.frame.hoverLine];
        let cursorParent: GanttTicket | null = null;
        let targetParent: GanttTicket | null = null;
        this.cursorIndex = -1;
        this.targetIndex = -1;
        for (let i = this.frame.movingLine - 1; i >= 0; i--) {
          let t = this.frame.lines[i];
          if (t.level < this.cursorTicket.level) {
            cursorParent = t;
            for (let j = 0; j < t.children.length; j++) {
              if (t.children[j].id === this.cursorTicket.id) {
                this.cursorIndex = j;
                break;
              }
            }
            break;
          }
        }

        for (let i = this.frame.hoverLine - 1; i >= 0; i--) {
          let t = this.frame.lines[i];
          if (t.level < this.targetTicket.level) {
            targetParent = t;
            for (let j = 0; j < t.children.length; j++) {
              if (t.children[j].id === this.targetTicket.id) {
                this.targetIndex = j;
                break;
              }
            }
            break;
          }
        }
        //console.log(
        //  `cursorIndex=${this.cursorIndex} targetIndex=${this.targetIndex}`
        //);

        switch (this.movingPart) {
          case MovingPart.TOP:
            break;
          case MovingPart.MIDDLE:
            if (
              cursorParent &&
              targetParent &&
              cursorIndex !== -1 &&
              targetIndex !== -1
            ) {
              cursorParent.children.splice(cursorIndex, 1);
              targetTicket.children.push(cursorTicket);
              this.frame.resetLines(cursorTicket.id);
            }
            break;
          case MovingPart.BOTTOM:
            break;
        }
      }
      */

      this.frame.dropPos = "";
      this.frame.draw();
    } catch (e) {
      window.alert(`エラーが発生しました。: ${e}`);
    }
  }

  mouseUp() {
    if (!this.frame.moving) {
      this.frame.clicked = false;
      return;
    }

    try {
      // チケットの移動処理
      console.log(
        `mouseUp: ${this.frame.movingTicket?.pos} ${this.upper?.pos ?? ""} ${
          this.movingLevel
        }`
      );
      const movePos = `${this.frame.movingTicket?.pos}`.split(",");
      //const dropPos = `${this.frame.dropPos}`.split(",");
      let moveTicket: GanttTicket | null = null;
      let upperTicket: GanttTicket | null = null;
      //let dropTicket: GanttTicket | null = null;
      let moveParent: GanttTicket | null = null;
      //let upperParent: GanttTicket | null = null;
      let moveIndex = 0;
      //let upperIndex = -1;
      let dropParent: GanttTicket | null = null;
      let dropIndex = -1;
      //let movingParent: GanttTicket | null = null;
      //let dropParent: GanttTicket | null = null;
      let w = this.frame.tickets;
      for (let i = 0; i < movePos.length; i++) {
        let m = movePos[i];
        moveTicket = w[Number(m)];
        w = moveTicket.children;
        if (i == movePos.length - 2) {
          moveParent = moveTicket;
        }
        if (i == movePos.length - 1) {
          moveIndex = Number(m);
        }
      }
      console.log(
        `moveParent=${
          moveParent ? moveParent.id_disp : "null"
        } moveIndex=${moveIndex}`
      );

      if (this.upper) {
        const upperPos = `${this.upper.pos}`.split(",");
        w = this.frame.tickets;
        for (let i = 0; i < upperPos.length; i++) {
          let m = upperPos[i];
          upperTicket = w[Number(m)];
          w = upperTicket.children;
          if (i + 1 === this.movingLevel) {
            dropParent = upperTicket;
            dropIndex = -1;
            if (i + 1 < upperPos.length) {
              dropIndex = Number(upperPos[i + 1]);
            }
          }
        }
      } else {
        //
      }
      console.log(
        `dropParent=${
          dropParent ? dropParent.id_disp : "null"
        } dropIndex=${dropIndex}`
      );
      /*
      console.log(
        `mouseUp: movingLine=${this.frame.movingLine} hoverLine=${this.frame.hoverLine} movingLevel=${this.movingLevel} diffY=${this.frame.diffY}`
      );
      if (
        this.frame.movingTicket &&
        this.frame.movingLine !== -1 &&
        this.frame.hoverLine !== -1 &&
        (this.frame.movingLine !== this.frame.hoverLine ||
          this.frame.movingTicket.level !== this.movingLevel)
      ) {
        let target = this.frame.lines[this.frame.hoverLine];
        if (this.frame.diffY < 0) {
          // 上に移動
          let l = this.frame.movingLine - this.frame.hoverLine;
          let d = -this.frame.diffY;
          let p = d - (l - 1) * TICKET_HEIGHT;
          console.log(
            `mouseUp: target=${target.id_disp} ${target.y1} ${target.y2} ${l} ${d} ${p}`
          );
        } else {
          // 下に移動
        }
      }
      */
      /*
      console.log(`${this.frame.movingTicket?.pos} ${this.frame.dropPos}`);
      if (`${this.frame.movingTicket?.pos}` != `${this.frame.dropPos}`) {
        // チケットの移動処理
        const movePos = `${this.frame.movingTicket?.pos}`.split(",");
        const dropPos = `${this.frame.dropPos}`.split(",");
        let moveTicket: GanttTicket | null = null;
        let dropTicket: GanttTicket | null = null;
        let moveParent: GanttTicket | null = null;
        let moveIndex = 0;
        let dropParent: GanttTicket | null = null;
        let dropIndex = 0;
        let w = this.frame.tickets;
        for (let i = 0; i < movePos.length; i++) {
          let m = movePos[i];
          moveTicket = w[Number(m)];
          w = moveTicket.children;
          if (i == movePos.length - 2) {
            moveParent = moveTicket;
          }
          if (i == movePos.length - 1) {
            moveIndex = Number(m);
          }
        }
        w = this.frame.tickets;
        for (let i = 0; i < dropPos.length; i++) {
          let d = dropPos[i];
          dropTicket = w[Number(d)];
          w = dropTicket.children;
          if (i == dropPos.length - 2) {
            dropParent = dropTicket;
          }
          if (i == dropPos.length - 1) {
            dropIndex = Number(d);
          }
        }
        console.log(
          `moveTicket: ${moveTicket?.id_disp}, dropTicket: ${dropTicket?.id_disp}, moveParent: ${moveParent?.id_disp}, moveIndex: ${moveIndex} `
        );
        if (
          moveTicket &&
          dropTicket &&
          moveTicket.id != dropTicket.id &&
          moveParent &&
          dropParent
        ) {
          moveParent.children.splice(moveIndex, 1);
          dropParent.children.splice(dropIndex, 0, moveTicket);
          //this.frame.resetLines(moveTicket.id);
        }
      }
      */
      this.frame.clicked = false;
      this.frame.moving = false;
      this.frame.diffY = 0;
      this.frame.resetLines("");
      this.frame.draw();
      this.frame.movingTicket = null;
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

  /*
  clickTickets(
    ts: GanttTicket[],
    clickx: number,
    clicky: number,
    //level: number,
    y: number
  ): number {
    let y1 = y;
    for (let t of ts) {
      if (t.progress === 100 && this.frame.showDone === false) {
        continue;
      }

      if (this.clickTicket(t, clickx, clicky, y1)) {
        return y1;
      }
      y1 += TICKET_HEIGHT;
      if (t.open && t.children.length > 0) {
        let y2 = this.clickTickets(t.children, clickx, clicky, y1);
        if (this.clickedId) {
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
    //level: number,
    y: number
  ): boolean {
    if (clicky < y + this.posY || clicky > y + TICKET_HEIGHT + this.posY) {
      return false;
    }
    this.frame.ganttRow.y1 = y;
    this.frame.ganttRow.y2 = y + TICKET_HEIGHT;
    this.frame.movingTicket = ticket;
    //this.frame.movingLevel = level;
    this.frame.dropPos = "";

    if (clickx < this.frame.cols[0].width) {
      // IDをクリックした
      this.clickedId = ticket.id;
      return true;
    }
    if (clickx < this.frame.cols[0].width + this.frame.cols[1].width) {
      const x = this.frame.cols[0].width + ticket.level * 12 + 2;
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
  */

  clickTicket(
    ticket: GanttTicket,
    clickx: number
    //clicky: number,
    //level: number,
    //y: number
  ) {
    //if (clicky < y + this.posY || clicky > y + TICKET_HEIGHT + this.posY) {
    //  return false;
    //}
    this.frame.ganttRow.y1 = ticket.y1;
    this.frame.ganttRow.y2 = ticket.y2;
    this.frame.movingTicket = ticket;
    //this.frame.movingLevel = level;
    this.frame.dropPos = "";

    if (clickx < this.frame.cols[0].width) {
      // IDをクリックした
      this.clickedId = ticket.id;
      return;
    }
    if (clickx < this.frame.cols[0].width + this.frame.cols[1].width) {
      const x = this.frame.cols[0].width + ticket.level * 12 + 2;
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
        //return true;
      }
    }
    //return false;
  }
}
