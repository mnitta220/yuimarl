import "./style.css";
//import typescriptLogo from "./typescript.svg";
//import viteLogo from "/vite.svg";
//import { setupCounter } from "./counter.ts";

/*
document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
*/

class Ticket {
  constructor(
    public id_disp: string,
    public name: string,
    public x: number,
    public y: number,
    public color: string,
    public x1: number,
    public y1: number,
    public x2: number,
    public y2: number
  ) {}
}

let tickets = [
  new Ticket("YU60", "チケットマップ", 30, 200, "warning", 0, 0, 0, 0),
  new Ticket("YU17", "WBS", 50, 150, "info", 0, 0, 0, 0),
  new Ticket("YU38", "更新バリデーション", -250, -50, "light", 0, 0, 0, 0),
];

let selected: Ticket | null = null;
let start_x = 0;
let start_y = 0;
let max_x = 2000;
let min_x = -2000;
let max_y = 500;
let min_y = -500;
let scroll_v = true;
let scroll_h = true;
let scroll_width = max_x - min_x;
let scroll_height = max_y - min_y;
let scroll_h_x1 = 0;
let scroll_h_x2 = 0;
let moving_h = false;
let moving_v = false;
let scroll_v_y1 = 0;
let scroll_v_y2 = 0;
let pos_x = 0;
let pos_y = 0;
let bar_length = 0;
let bar_height = 0;
let frame_width = 0;
let frame_height = 0;

function draw(scr_h: boolean, scr_v: boolean) {
  const cnvs = document.querySelector<HTMLCanvasElement>("#cnvs");
  if (cnvs) {
    const width = cnvs.offsetWidth;
    const height = cnvs.offsetHeight;
    //console.log(`width:${width} height:${height}`);
    cnvs.width = width;
    cnvs.height = height;
    const ctx = cnvs.getContext("2d");
    if (ctx) {
      ctx.save();

      frame_width = width - 16;
      frame_height = height - 16;

      // 中心線描画
      ctx.strokeStyle = "#7777ff";
      ctx.lineWidth = 1;
      let center_y = (height - 16) / 2;
      let center_x = (width - 16) / 2;
      let h = height - 16;
      let w = width - 16;
      ctx.beginPath();
      ctx.moveTo(0, center_y - pos_y);
      ctx.lineTo(w, center_y - pos_y);
      ctx.moveTo(center_x - pos_x, 0);
      ctx.lineTo(center_x - pos_x, h);
      ctx.stroke();
      ctx.font = "9pt sans-serif";
      ctx.textAlign = "left";
      ctx.textBaseline = "top";
      ctx.fillStyle = "#7777ff";
      ctx.fillText("低工数 ←", 4, center_y + 4 - pos_y);
      ctx.textAlign = "right";
      ctx.textBaseline = "top";
      ctx.fillText("→ 高工数", w - 4, center_y + 4 - pos_y);
      ctx.textBaseline = "bottom";
      ctx.textAlign = "left";
      ctx.rotate(Math.PI / 2);
      ctx.fillText("高価値 ←", 4, -center_x - 4 + pos_x);
      ctx.textAlign = "right";
      ctx.fillText("→ 低価値", h - 4, -center_x - 4 + pos_x);
      ctx.restore();

      // チケット描画
      const font1 = "8.5pt sans-serif";
      const font2 = "9.5pt sans-serif";

      ctx.save();
      for (let ticket of tickets) {
        ctx.font = font2;
        ctx.textBaseline = "bottom";
        ctx.textAlign = "left";

        switch (ticket.color) {
          case "info":
            ctx.strokeStyle = "#b3d3ff";
            ctx.fillStyle = "#cff4fc";
            break;
          case "primary":
            ctx.strokeStyle = "#9ec5fe";
            ctx.fillStyle = "#cfe2ff";
            break;
          case "warning":
            ctx.strokeStyle = "#ffe69c";
            ctx.fillStyle = "#fff3cd";
            break;
          case "success":
            ctx.strokeStyle = "#a3cfbb";
            ctx.fillStyle = "#d1e7dd";
            break;
          case "danger":
            ctx.strokeStyle = "#f1aeb5";
            ctx.fillStyle = "#f8d7da";
            break;
          case "secondary":
            ctx.strokeStyle = "#c4c8cb";
            ctx.fillStyle = "#e2e3e5";
            break;
          case "dark":
            ctx.strokeStyle = "#adb5bd";
            ctx.fillStyle = "#ced4da";
            break;
          default:
            ctx.strokeStyle = "#cccdce";
            ctx.fillStyle = "#f2f2f2";
            break;
        }

        let t1 = ticket.id_disp;
        let t2 = ticket.name;
        let m1 = ctx.measureText(t1).width;
        let m2 = ctx.measureText(t2).width;
        let m = m1 + m2 + 10;
        ctx.beginPath();
        let x = width / 2 + ticket.x - pos_x;
        let y = height / 2 - ticket.y - pos_y;
        ticket.x1 = x;
        ticket.y1 = y;
        ticket.x2 = x + m;
        ticket.y2 = y + 20;
        ctx.moveTo(x, y);
        ctx.lineTo(x + m, y);
        ctx.lineTo(x + m, y + 20);
        ctx.lineTo(x, y + 20);
        ctx.closePath();
        ctx.lineWidth = 1;
        ctx.fill();
        ctx.stroke();
        ctx.font = font1;
        ctx.fillStyle = "#4444ff";
        ctx.fillText(t1, x + 4, y + 15);
        ctx.font = font2;
        ctx.fillStyle = "black";
        ctx.fillText(t2, x + 6 + m1, y + 16);
      }

      // スクロールバー描画
      ctx.fillStyle = "#f1f1f1";
      ctx.fillRect(0, frame_height, frame_width, 16);
      ctx.fillRect(frame_width, 0, 16, frame_height);
      ctx.fillStyle = "#dcdcdc";
      ctx.fillRect(frame_width, frame_height, 16, 16);
      ctx.lineJoin = "miter";
      ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
      ctx.beginPath();
      ctx.moveTo(5, height - 8);
      ctx.lineTo(9, height - 4);
      ctx.lineTo(9, height - 12);
      ctx.closePath();
      ctx.fill();
      ctx.beginPath();
      ctx.moveTo(width - 26, height - 12);
      ctx.lineTo(width - 26, height - 4);
      ctx.lineTo(width - 22, height - 8);
      ctx.closePath();
      ctx.fill();
      ctx.fillStyle = scroll_v ? "#505050" : "#a3a3a3";
      ctx.beginPath();
      ctx.moveTo(width - 8, 5);
      ctx.lineTo(width - 12, 9);
      ctx.lineTo(width - 4, 9);
      ctx.closePath();
      ctx.fill();
      ctx.beginPath();
      ctx.moveTo(width - 12, height - 26);
      ctx.lineTo(width - 8, height - 22);
      ctx.lineTo(width - 4, height - 26);
      ctx.closePath();
      ctx.fill();

      if (scroll_h) {
        bar_length = width - 48;
        let w1 = scroll_width / 2 + pos_x - frame_width / 2;
        let w2 = w1 + frame_width;
        let x1 = (bar_length * w1) / scroll_width;
        let x2 = (bar_length * w2) / scroll_width;
        scroll_h_x1 = x1 + 16;
        scroll_h_x2 = x2 + 16;
        ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
        ctx.fillRect(scroll_h_x1, height - 14, x2 - x1, 13);
      }

      if (scroll_v) {
        bar_height = height - 48;
        let h1 = scroll_height / 2 + pos_y - frame_height / 2;
        let h2 = h1 + frame_height;
        let y1 = (bar_height * h1) / scroll_height;
        let y2 = (bar_height * h2) / scroll_height;
        scroll_v_y1 = y1 + 16;
        scroll_v_y2 = y2 + 16;
        ctx.fillStyle = scr_v ? "#a8a8a8" : "#c1c1c1";
        ctx.fillRect(width - 14, scroll_v_y1, 13, y2 - y1);
      }

      ctx.restore();
    }
  }
}

draw(false, false);

document
  .querySelector<HTMLCanvasElement>("#cnvs")
  ?.addEventListener("mousedown", function (e) {
    e.preventDefault();
    let $element = this;
    let x = e.pageX - this.offsetLeft;
    let y = e.pageY - this.offsetTop;
    console.log(`mousedown: ${e.pageX} ${e.pageY} ${x} ${y}`);
    clickDown(x, y, $element.offsetWidth, $element.offsetHeight);
  });

document
  .querySelector<HTMLCanvasElement>("#cnvs")
  ?.addEventListener("touchstart", function (e) {
    e.preventDefault();
    let $element = this;
    let x = e.touches[0].pageX - this.offsetLeft;
    let y = e.touches[0].pageY - this.offsetTop;
    //console.log(`touchstart: ${e.pageX} ${e.pageY} ${x} ${y}`);
    clickDown(x, y, $element.offsetWidth, $element.offsetHeight);
  });

document
  .querySelector<HTMLCanvasElement>("#cnvs")
  ?.addEventListener("mousemove", function (e) {
    e.preventDefault();
    let $element = this;
    let x = e.pageX - this.offsetLeft;
    let y = e.pageY - this.offsetTop;
    console.log(`mousemove: ${e.pageX} ${e.pageY} ${x} ${y}`);
    //clickDown(x, y, $element.offsetWidth, $element.offsetHeight);
    clickMove($element, x, y);
  });

document
  .querySelector<HTMLCanvasElement>("#cnvs")
  ?.addEventListener("touchmove", function (e) {
    e.preventDefault();
    let $element = this;
    let x = e.touches[0].pageX - this.offsetLeft;
    let y = e.touches[0].pageY - this.offsetTop;
    //console.log(`mousemove: ${e.pageX} ${e.pageY} ${x} ${y}`);
    //clickDown(x, y, $element.offsetWidth, $element.offsetHeight);
    clickMove($element, x, y);
  });

document
  .querySelector<HTMLCanvasElement>("#cnvs")
  ?.addEventListener("mouseup", function (e) {
    e.preventDefault();
    selected = null;
    moving_h = false;
    moving_v = false;
    this.classList.remove("cursor-pointer");
  });

document
  .querySelector<HTMLCanvasElement>("#cnvs")
  ?.addEventListener("touchend", function (e) {
    e.preventDefault();
    selected = null;
    moving_h = false;
    moving_v = false;
    this.classList.remove("cursor-pointer");
  });

function clickDown(x: number, y: number, width: number, height: number) {
  selected = null;

  if (y > height - 16) {
    // 横スクロールバー
    if (x > scroll_h_x1 && x < scroll_h_x2) {
      moving_h = true;
      start_x = x;
    }
  } else if (x > width - 16) {
    // 縦スクロールバー
    if (y > scroll_v_y1 && y < scroll_v_y2) {
      moving_v = true;
      start_y = y;
    }
  } else {
    // マップエリア
    for (let i in tickets) {
      let ticket = tickets[i];
      if (x > ticket.x1 && x < ticket.x2 && y > ticket.y1 && y < ticket.y2) {
        selected = ticket;
        tickets.splice(Number(i), 1);
        tickets.push(selected);
        break;
      }
    }
    if (selected) {
      start_x = x;
      start_y = y;
    }
  }
}

function clickMove($element: HTMLCanvasElement, x: number, y: number) {
  let pointer = false;

  if (y > $element.offsetHeight - 16) {
    // 横スクロールバー
    selected = null;
    moving_v = false;
    $element.classList.remove("cursor-pointer");
    if (moving_h) {
      let diff_x = x - start_x;
      scroll_h_x1 += diff_x;
      let x1 = scroll_h_x1 - 16;
      let w1 = (x1 * scroll_width) / bar_length;
      if (w1 < 0 || w1 + frame_width > scroll_width) return;
      pos_x = w1 + frame_width / 2 - scroll_width / 2;
      start_x = x;
    }
    draw(x > scroll_h_x1 && x < scroll_h_x2, false);
  } else if (x > $element.offsetWidth - 16) {
    // 縦スクロールバー
    selected = null;
    $element.classList.remove("cursor-pointer");
    moving_h = false;
    if (moving_v) {
      let diff_y = y - start_y;
      scroll_v_y1 += diff_y;
      let y1 = scroll_v_y1 - 16;
      let h1 = (y1 * scroll_height) / bar_height;
      if (h1 < 0 || h1 + frame_height > scroll_height) return;
      pos_y = h1 + frame_height / 2 - scroll_height / 2;
      start_y = y;
    }
    draw(false, y > scroll_v_y1 && y < scroll_v_y2);
  } else {
    moving_h = false;
    moving_v = false;
    // マップエリア
    for (let ticket of tickets) {
      if (x > ticket.x1 && x < ticket.x2 && y > ticket.y1 && y < ticket.y2) {
        pointer = true;
        break;
      }
    }
    if (pointer) {
      $element.classList.add("cursor-pointer");
    } else {
      $element.classList.remove("cursor-pointer");
    }
    if (selected) {
      let diff_x = x - start_x;
      let diff_y = y - start_y;
      selected.x += diff_x;
      selected.y -= diff_y;
      start_x = x;
      start_y = y;
    }
    draw(false, false);
  }
}
