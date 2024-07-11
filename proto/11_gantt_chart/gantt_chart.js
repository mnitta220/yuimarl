// カラム
var Column = /** @class */ (function () {
    function Column() {
        this.name = "";
        this.width = 0;
    }
    return Column;
}());
// カラム定義
var cols = [
    {
        name: "ID",
        width: 54,
    },
    {
        name: "チケット名",
        width: 320,
    },
    {
        name: "開始日",
        width: 67,
    },
    {
        name: "終了日",
        width: 67,
    },
    {
        name: "進捗",
        width: 36,
    },
];
var Ticket = /** @class */ (function () {
    function Ticket() {
        this.id = "";
        this.idDisp = "";
        this.name = "";
        this.start = null;
        this.end = null;
        this.progress = 0;
        this.open = false;
        this.children = [];
    }
    return Ticket;
}());
var tickets = [
    {
        id: "YU1",
        idDisp: "YU1",
        name: "チケットYU1",
        start: new Date(2024, 6, 1),
        end: new Date(2024, 6, 31),
        progress: 50,
        open: true,
        children: [
            {
                id: "YU11",
                idDisp: "YU11",
                name: "チケットYU11",
                start: new Date(2024, 6, 1),
                end: new Date(2024, 6, 2),
                progress: 100,
                open: true,
                children: [
                    {
                        id: "YU111",
                        idDisp: "YU111",
                        name: "チケットYU111あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                ],
            },
        ],
    },
    {
        id: "YU2",
        idDisp: "YU2",
        name: "チケットYU2",
        start: new Date(2024, 6, 1),
        end: null,
        progress: 0,
        open: true,
        children: [
            {
                id: "YU21",
                idDisp: "YU21",
                name: "チケットYU21",
                start: new Date(2024, 6, 1),
                end: new Date(2024, 6, 2),
                progress: 100,
                open: true,
                children: [
                    {
                        id: "YU211",
                        idDisp: "YU211",
                        name: "チケットYU211あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                ],
            },
            {
                id: "YU22",
                idDisp: "YU22",
                name: "チケットYU22",
                start: new Date(2024, 6, 1),
                end: new Date(2024, 6, 2),
                progress: 100,
                open: true,
                children: [
                    {
                        id: "YU221",
                        idDisp: "YU221",
                        name: "チケットYU221あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU222",
                        idDisp: "YU222",
                        name: "チケットYU222あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU223",
                        idDisp: "YU223",
                        name: "チケットYU223あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [
                            {
                                id: "YU2231",
                                idDisp: "YU2231",
                                name: "チケットYU2231あいうえおかきくけこさしすせそたちつてと",
                                start: new Date(2024, 6, 10),
                                end: new Date(2024, 6, 16),
                                progress: 10,
                                open: true,
                                children: [
                                    {
                                        id: "YU223a",
                                        idDisp: "YU223a",
                                        name: "チケットYU223aあいうえおかきくけこさしすせそたちつてと",
                                        start: new Date(2024, 6, 10),
                                        end: new Date(2024, 6, 16),
                                        progress: 10,
                                        open: true,
                                        children: [],
                                    },
                                ],
                            },
                        ],
                    },
                    {
                        id: "YU224",
                        idDisp: "YU224",
                        name: "チケットYU224あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU225",
                        idDisp: "YU225",
                        name: "チケットYU225あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU226",
                        idDisp: "YU226",
                        name: "チケットYU226あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU227",
                        idDisp: "YU227",
                        name: "チケットYU227あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU228",
                        idDisp: "YU228",
                        name: "チケットYU228あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU229",
                        idDisp: "YU229",
                        name: "チケットYU229あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU2210",
                        idDisp: "YU2210",
                        name: "チケットYU2210あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                ],
            },
        ],
    },
    {
        id: "YU3",
        idDisp: "YU3",
        name: "チケットYU3あいうえおかきくけこさしすせそたa",
        start: null,
        end: new Date(2024, 6, 1),
        progress: 25,
        open: true,
        children: [
            {
                id: "YU31",
                idDisp: "YU31",
                name: "チケットYU31",
                start: new Date(2024, 6, 1),
                end: new Date(2024, 6, 2),
                progress: 100,
                open: true,
                children: [
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                    {
                        id: "YU311",
                        idDisp: "YU311",
                        name: "チケットYU311あいうえおかきくけこさしすせそたちつてと",
                        start: new Date(2024, 6, 10),
                        end: new Date(2024, 6, 16),
                        progress: 10,
                        open: true,
                        children: [],
                    },
                ],
            },
        ],
    },
    {
        id: "YU9999",
        idDisp: "YU9999",
        name: "チケットYU9999あいうえおかきくけこさしすせそたちつてと",
        start: new Date(2024, 6, 1),
        end: new Date(2024, 6, 3),
        progress: 40,
        open: true,
        children: [],
    },
];
// 日本の祝日
var holidays = [new Date(2024, 6, 15), new Date(2024, 7, 12)];
var SCROLL_BAR_WIDTH = 16;
var HEADER_LABEL_Y = 42;
var LINE_HEIGHT = 21;
var HEADER_HEIGHT = 63;
var TICKET_HEIGHT = 22;
var DAY_WIDTH = 22;
var CALENDAR_MIN = DAY_WIDTH * 10;
var DAY_MILISEC = 1000 * 60 * 60 * 24;
// カラムヘッダー
var ColumnHeader = /** @class */ (function () {
    function ColumnHeader() {
        this.id = "colhead";
        this.pos = 0;
    }
    // カラムヘッダーを構築する
    ColumnHeader.prototype.build = function (frag) {
        var hd = document.createElement("div");
        hd.id = this.id;
        hd.className = "header";
        hd.style.top = "0px";
        hd.style.left = "".concat(this.pos, "px");
        hd.style.width = "".concat(frame.calendarLeft, "px");
        hd.style.height = "".concat(HEADER_HEIGHT, "px");
        frag.append(hd);
        var line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(HEADER_HEIGHT, "px");
        line.style.left = "0px";
        line.style.width = "".concat(frame.calendarLeft, "px");
        line.style.height = "1px";
        frag.append(line);
        var x = 0;
        for (var _i = 0, cols_1 = cols; _i < cols_1.length; _i++) {
            var col = cols_1[_i];
            // カラムラベル
            var label = document.createElement("div");
            label.className = "label";
            label.style.top = "".concat(HEADER_LABEL_Y, "px");
            label.style.left = "".concat(x, "px");
            label.style.width = "".concat(col.width, "px");
            label.textContent = col.name;
            hd.append(label);
            x += col.width;
            // カラム区切り線
            line = document.createElement("div");
            line.className = "line";
            line.style.top = "0px";
            line.style.left = "".concat(x, "px");
            line.style.width = "1px";
            line.style.height = "".concat(HEADER_HEIGHT, "px");
            hd.append(line);
        }
        // カレンダー境界線
        x += 2;
        line = document.createElement("div");
        line.className = "line";
        line.style.top = "0px";
        line.style.left = "".concat(x, "px");
        line.style.width = "1px";
        line.style.height = "".concat(HEADER_HEIGHT, "px");
        hd.append(line);
    };
    ColumnHeader.prototype.scrollH = function (x) {
        this.pos = -x;
        var colhead = document.querySelector("#".concat(this.id));
        if (colhead) {
            colhead.style.left = "".concat(this.pos, "px");
        }
    };
    return ColumnHeader;
}());
// カラムボディ
var ColumnBody = /** @class */ (function () {
    function ColumnBody() {
        this.id = "colbody";
        this.width = 0;
        this.height = 0;
        this.posX = 0;
        this.posY = 0;
    }
    // カラムボディを構築する
    ColumnBody.prototype.build = function (frag) {
        var body = document.createElement("canvas");
        this.height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
        frame.ticketsFrameHeight = this.height;
        body.id = this.id;
        this.width = frame.calendarLeft;
        body.className = "gantt-body";
        body.style.top = "".concat(HEADER_HEIGHT, "px");
        body.style.left = "".concat(this.posX, "px");
        body.style.width = "".concat(this.width, "px");
        body.style.height = "".concat(this.height, "px");
        frag.append(body);
    };
    // イベントハンドラを登録する
    ColumnBody.prototype.handler = function () {
        var calscroll = document.querySelector("#".concat(this.id));
        if (calscroll) {
            calscroll.addEventListener("mousedown", function (e) {
                e.preventDefault();
                frame.columnBody.mouseDown(e.layerX, e.layerY);
            });
        }
    };
    // 描画する
    ColumnBody.prototype.draw = function () {
        var cnvs = document.querySelector("#".concat(this.id));
        if (cnvs) {
            var width = cnvs.offsetWidth;
            var height = cnvs.offsetHeight;
            cnvs.width = width;
            cnvs.height = height;
            var ctx = cnvs.getContext("2d");
            if (ctx) {
                ctx.save();
                frame.ticketsTotalHeight = this.drawTickets(ctx, tickets, 0, 0);
                ctx.fillStyle = "#82a4c1";
                var x = 0;
                for (var _i = 0, cols_2 = cols; _i < cols_2.length; _i++) {
                    var col = cols_2[_i];
                    x += col.width;
                    // カラム区切り線
                    ctx.fillRect(x, 0, 1, height);
                    ctx.fill();
                }
                // カレンダー境界線
                x += 2;
                ctx.fillRect(x, 0, 1, height);
                ctx.fill();
                ctx.restore();
            }
        }
    };
    ColumnBody.prototype.drawTickets = function (ctx, ts, level, y) {
        var y1 = y;
        for (var _i = 0, ts_1 = ts; _i < ts_1.length; _i++) {
            var t = ts_1[_i];
            this.drawTicket(ctx, t, level, y1);
            y1 += TICKET_HEIGHT;
            if (t.open && t.children.length > 0) {
                y1 = this.drawTickets(ctx, t.children, level + 1, y1);
            }
        }
        return y1;
    };
    ColumnBody.prototype.getFrameHeight = function (ts, y) {
        var y1 = y;
        for (var _i = 0, ts_2 = ts; _i < ts_2.length; _i++) {
            var t = ts_2[_i];
            y1 += TICKET_HEIGHT;
            if (t.open && t.children.length > 0) {
                y1 = this.getFrameHeight(t.children, y1);
            }
        }
        return y1;
    };
    // チケットを描画する
    ColumnBody.prototype.drawTicket = function (ctx, ticket, level, y) {
        ctx.font = "9.5pt sans-serif";
        ctx.textBaseline = "bottom";
        var y1 = y + 18;
        ctx.fillStyle = "#00f";
        // ID
        ctx.fillText(ticket.idDisp, 3, y1 + this.posY);
        ctx.fillStyle = "#808080";
        // チケット□
        var x = cols[0].width + level * 12 + 6;
        ctx.fillRect(x, y + 7 + this.posY, 1, 8);
        ctx.fill();
        ctx.fillRect(x, y + 7 + this.posY, 8, 1);
        ctx.fill();
        ctx.fillRect(x, y + 15 + this.posY, 8, 1);
        ctx.fill();
        ctx.fillRect(x + 8, y + 7 + this.posY, 1, 9);
        ctx.fill();
        ctx.fillStyle = "#000";
        if (ticket.children.length > 0) {
            ctx.fillRect(x + 2, y + 11 + this.posY, 5, 1);
            ctx.fill();
            if (!ticket.open) {
                ctx.fillRect(x + 4, y + 9 + this.posY, 1, 5);
                ctx.fill();
            }
        }
        // チケット名
        var w = cols[0].width + cols[1].width - (x + 14);
        var m = ctx.measureText(ticket.name).width;
        if (m <= w) {
            ctx.fillText(ticket.name, x + 14, y1 + this.posY);
        }
        else {
            var s = ticket.name;
            while (ctx.measureText("".concat(s, "\u2026")).width > w) {
                s = s.slice(0, -1);
            }
            ctx.fillText("".concat(s, "\u2026"), x + 14, y1 + this.posY);
        }
        x = cols[0].width + cols[1].width + 3;
        // 開始日
        if (ticket.start) {
            ctx.fillText(this.dtDisp(ticket.start), x, y1 + this.posY);
        }
        x += cols[2].width;
        // 終了日
        if (ticket.end) {
            ctx.fillText(this.dtDisp(ticket.end), x, y1 + this.posY);
        }
        x += cols[3].width + cols[4].width - 6;
        // 進捗率
        var t1 = "".concat(ticket.progress);
        var m1 = ctx.measureText(t1).width;
        ctx.fillText(t1, x - m1, y1 + this.posY);
        ctx.fillStyle = "#e5ebf2";
        // 区切り線
        ctx.fillRect(0, y + TICKET_HEIGHT + this.posY, this.width, 1);
        ctx.fill();
    };
    ColumnBody.prototype.dtDisp = function (dt) {
        var y = "".concat(dt.getFullYear()).slice(-2);
        var m = "0".concat(dt.getMonth() + 1).slice(-2);
        var d = "0".concat(dt.getDate()).slice(-2);
        return "".concat(y, "/").concat(m, "/").concat(d);
    };
    ColumnBody.prototype.scrollH = function (x) {
        this.posX = -x;
        var colbody = document.querySelector("#".concat(this.id));
        if (colbody) {
            colbody.style.left = "".concat(this.posX, "px");
        }
    };
    ColumnBody.prototype.scrollV = function (y) {
        this.posY = -y;
        this.draw();
    };
    ColumnBody.prototype.mouseDown = function (x, y) {
        this.clickTickets(tickets, x, y, 0, 0);
    };
    ColumnBody.prototype.clickTickets = function (ts, clickx, clicky, level, y) {
        var y1 = y;
        for (var _i = 0, ts_3 = ts; _i < ts_3.length; _i++) {
            var t = ts_3[_i];
            if (this.clickTicket(t, clickx, clicky, level, y1)) {
                break;
            }
            y1 += TICKET_HEIGHT;
            if (t.open && t.children.length > 0) {
                y1 = this.clickTickets(t.children, clickx, clicky, level + 1, y1);
            }
        }
        return y1;
    };
    ColumnBody.prototype.clickTicket = function (ticket, clickx, clicky, level, y) {
        if (clicky < y + this.posY || clicky > y + TICKET_HEIGHT + this.posY) {
            return false;
        }
        if (clickx > cols[0].width && clickx < cols[0].width + cols[1].width) {
            var x = cols[0].width + level * 12 + 6;
            if (clickx > x - 1 &&
                clickx < x + 9 &&
                clicky > y + 6 + this.posY &&
                clicky < y + 16 + this.posY) {
                // チケット名の□をクリックした
                ticket.open = !ticket.open;
                frame.draw();
                if (frame.ticketsTotalHeight < frame.ticketsFrameHeight &&
                    this.posY != 0) {
                    this.posY = 0;
                    frame.calendarBody.posY = 0;
                    frame.scv.pos = 0;
                    frame.draw();
                }
                return true;
            }
        }
        return false;
    };
    return ColumnBody;
}());
// カラムスクロールバー
var ColumnScroll = /** @class */ (function () {
    function ColumnScroll() {
        this.id = "colscr";
        this.pos = 0;
    }
    // スクロールバーを構築する
    ColumnScroll.prototype.build = function (frag) {
        var bar = document.createElement("div");
        bar.id = this.id;
        bar.className = "scroll-corner";
        bar.style.top = "".concat(frame.height - SCROLL_BAR_WIDTH, "px");
        bar.style.left = "".concat(this.pos, "px");
        bar.style.width = "".concat(frame.calendarLeft, "px");
        bar.style.height = "".concat(SCROLL_BAR_WIDTH, "px");
        frag.append(bar);
    };
    ColumnScroll.prototype.scrollH = function (x) {
        this.pos = -x;
        var colscr = document.querySelector("#".concat(this.id));
        if (colscr) {
            colscr.style.left = "".concat(this.pos, "px");
        }
    };
    return ColumnScroll;
}());
// カレンダーヘッダー
var CalendarHeader = /** @class */ (function () {
    function CalendarHeader() {
        this.id = "calhead";
        this.width = 0;
        this.pos = 0;
        this.dtpos = 0;
        this.dtStart = 0;
        this.dtEnd = 0;
    }
    // カレンダーヘッダーを構築する
    CalendarHeader.prototype.build = function (frag) {
        var cv = document.createElement("canvas");
        this.width = frame.width - frame.calendarLeft;
        if (this.width < CALENDAR_MIN) {
            this.width = CALENDAR_MIN;
        }
        cv.id = this.id;
        cv.className = "header";
        cv.style.top = "0px";
        cv.style.left = "".concat(this.pos + frame.calendarLeft, "px");
        cv.style.width = "".concat(this.width, "px");
        cv.style.height = "".concat(HEADER_HEIGHT + 1, "px");
        frag.append(cv);
        this.dtStart = frame.calendarStart.getTime();
        this.dtEnd = frame.calendarEnd.getTime();
    };
    CalendarHeader.prototype.resize = function () {
        this.width = frame.width - frame.calendarLeft;
        if (this.width < CALENDAR_MIN) {
            this.width = CALENDAR_MIN;
        }
        var cv = document.querySelector("#".concat(this.id));
        if (cv) {
            cv.style.width = "".concat(this.width, "px");
        }
        this.dtpos = 0;
    };
    // 描画する
    CalendarHeader.prototype.draw = function () {
        var cnvs = document.querySelector("#".concat(this.id));
        if (cnvs) {
            var height = cnvs.offsetHeight;
            cnvs.width = this.width;
            cnvs.height = height;
            var dtTop = LINE_HEIGHT + LINE_HEIGHT;
            var ctx = cnvs.getContext("2d");
            var font = "9.5pt sans-serif";
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
                var dt_1 = new Date(this.dtStart);
                var x = 0;
                var first = true;
                ctx.font = font;
                ctx.textBaseline = "bottom";
                ctx.textAlign = "left";
                while (dt_1.getTime() <= this.dtEnd) {
                    if (x < this.dtpos - DAY_WIDTH) {
                        x += DAY_WIDTH;
                        dt_1.setTime(dt_1.getTime() + DAY_MILISEC);
                        first = false;
                        continue;
                    }
                    if (x > this.width + this.dtpos)
                        break;
                    var date = dt_1.getDate();
                    if (first) {
                        first = false;
                        if (date == 1 || date < 25) {
                            ctx.fillStyle = "#000";
                            ctx.fillText("".concat(dt_1.getFullYear(), "/").concat(dt_1.getMonth() + 1), x + 3 - this.dtpos, LINE_HEIGHT + LINE_HEIGHT - 3);
                        }
                    }
                    else if (date === 1) {
                        ctx.fillStyle = "#000";
                        ctx.fillText("".concat(dt_1.getFullYear(), "/").concat(dt_1.getMonth() + 1), x + 3 - this.dtpos, LINE_HEIGHT + LINE_HEIGHT - 3);
                        ctx.fillStyle = "#bdcede";
                        ctx.fillRect(x - this.dtpos, LINE_HEIGHT, 1, LINE_HEIGHT);
                        ctx.fill();
                    }
                    var holiday = holidays.filter(function (d) { return d.getTime() === dt_1.getTime(); });
                    if (holiday.length > 0) {
                        ctx.fillStyle = "#f00";
                    }
                    else {
                        switch (dt_1.getDay()) {
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
                    ctx.fillText("".concat(date), x + 3 - this.dtpos, HEADER_HEIGHT - 3);
                    x += DAY_WIDTH;
                    dt_1.setTime(dt_1.getTime() + DAY_MILISEC);
                    // 日付区切り線
                    ctx.fillStyle = "#bdcede";
                    ctx.fillRect(x - this.dtpos, dtTop, 1, LINE_HEIGHT);
                    ctx.fill();
                }
                ctx.restore();
            }
        }
    };
    CalendarHeader.prototype.scrollH = function (x) {
        this.pos = -x;
        var calhead = document.querySelector("#".concat(this.id));
        if (calhead) {
            calhead.style.left = "".concat(this.pos + frame.calendarLeft, "px");
        }
    };
    CalendarHeader.prototype.scroll = function (x) {
        this.dtpos =
            (x * frame.calendarTotalWidth) /
                (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
        this.draw();
        frame.calendarBody.dtpos = this.dtpos;
        frame.calendarBody.draw();
    };
    return CalendarHeader;
}());
// カレンダーボディ
var CalendarBody = /** @class */ (function () {
    function CalendarBody() {
        this.id = "calbody";
        this.width = 0;
        this.posX = 0;
        this.posY = 0;
        this.dtpos = 0;
        this.dtStart = 0;
        this.dtEnd = 0;
    }
    // カレンダーボディを構築する
    CalendarBody.prototype.build = function (frag) {
        var cv = document.createElement("canvas");
        this.width = frame.width - frame.calendarLeft;
        if (this.width < CALENDAR_MIN) {
            this.width = CALENDAR_MIN;
        }
        var height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
        cv.id = this.id;
        cv.className = "gantt-body";
        cv.style.top = "".concat(HEADER_HEIGHT, "px");
        cv.style.left = "".concat(this.posX + frame.calendarLeft, "px");
        cv.style.width = "".concat(this.width, "px");
        cv.style.height = "".concat(height, "px");
        frag.append(cv);
        this.dtStart = frame.calendarStart.getTime();
        this.dtEnd = frame.calendarEnd.getTime();
    };
    CalendarBody.prototype.resize = function () {
        this.width = frame.width - frame.calendarLeft;
        if (this.width < CALENDAR_MIN) {
            this.width = CALENDAR_MIN;
        }
        var cv = document.querySelector("#".concat(this.id));
        if (cv) {
            cv.style.width = "".concat(this.width, "px");
        }
        this.dtpos = 0;
    };
    // 描画する
    CalendarBody.prototype.draw = function () {
        var cnvs = document.querySelector("#".concat(this.id));
        if (cnvs) {
            var width = cnvs.offsetWidth;
            var height = cnvs.offsetHeight;
            cnvs.width = width;
            cnvs.height = height;
            var ctx = cnvs.getContext("2d");
            if (ctx) {
                ctx.save();
                var dt_2 = new Date(this.dtStart);
                var x = 0;
                while (dt_2.getTime() <= this.dtEnd) {
                    if (x < this.dtpos - DAY_WIDTH) {
                        x += DAY_WIDTH;
                        dt_2.setTime(dt_2.getTime() + DAY_MILISEC);
                        continue;
                    }
                    if (x > this.width + this.dtpos)
                        break;
                    var dayoff = false;
                    var holiday = holidays.filter(function (d) { return d.getTime() === dt_2.getTime(); });
                    if (holiday.length > 0) {
                        dayoff = true;
                    }
                    else {
                        switch (dt_2.getDay()) {
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
                    dt_2.setTime(dt_2.getTime() + DAY_MILISEC);
                    x += DAY_WIDTH;
                    if (x > width + this.dtpos)
                        break;
                    // 日付区切り線
                    ctx.fillStyle = "#bdcede";
                    ctx.fillRect(x - this.dtpos, 0, 1, height);
                    ctx.fill();
                }
                this.drawTickets(ctx, tickets, 0);
                // 現在線
                x = ((new Date().getTime() - this.dtStart) / DAY_MILISEC) * DAY_WIDTH;
                ctx.fillStyle = "#0a0";
                for (var y = 0; y < height; y += 7) {
                    ctx.fillRect(x - this.dtpos, y, 1, 4);
                    ctx.fill();
                }
                ctx.restore();
            }
        }
    };
    CalendarBody.prototype.drawTickets = function (ctx, ts, y) {
        var y1 = y;
        for (var _i = 0, ts_4 = ts; _i < ts_4.length; _i++) {
            var t = ts_4[_i];
            this.drawTicket(ctx, t, y1);
            y1 += TICKET_HEIGHT;
            if (t.open && t.children.length > 0) {
                y1 = this.drawTickets(ctx, t.children, y1);
            }
        }
        return y1;
    };
    // チケットを描画する
    CalendarBody.prototype.drawTicket = function (ctx, ticket, y) {
        // チケット開始日/終了日
        if (ticket.start) {
            if (ticket.end) {
                var x1 = (ticket.start.getTime() - this.dtStart) / DAY_MILISEC;
                x1 = x1 * DAY_WIDTH - this.dtpos;
                var x2 = (ticket.end.getTime() - this.dtStart) / DAY_MILISEC;
                x2 = (x2 + 1) * DAY_WIDTH - this.dtpos;
                if (ticket.progress === 0) {
                    ctx.fillStyle = "#9bf";
                    ctx.fillRect(x1, y + 8 + this.posY, x2 - x1, 6);
                    ctx.fill();
                }
                else if (ticket.progress === 100) {
                    ctx.fillStyle = "#57f";
                    ctx.fillRect(x1, y + 8 + this.posY, x2 - x1, 6);
                    ctx.fill();
                }
                else {
                    ctx.fillStyle = "#9bf";
                    ctx.fillRect(x1, y + 8 + this.posY, x2 - x1, 6);
                    ctx.fill();
                    var x3 = ((x2 - x1) * ticket.progress) / 100;
                    ctx.fillStyle = "#57f";
                    ctx.fillRect(x1, y + 8 + this.posY, x3, 6);
                    ctx.fill();
                }
            }
            else {
                ctx.fillStyle = "#57f";
                var x1 = (ticket.start.getTime() - this.dtStart) / DAY_MILISEC;
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
        }
        else {
            if (ticket.end) {
                ctx.fillStyle = "#57f";
                var x2 = (ticket.end.getTime() - this.dtStart) / DAY_MILISEC;
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
    };
    CalendarBody.prototype.scrollH = function (x) {
        this.posX = -x;
        var calbody = document.querySelector("#".concat(this.id));
        if (calbody) {
            calbody.style.left = "".concat(this.posX + frame.calendarLeft, "px");
        }
    };
    CalendarBody.prototype.scrollV = function (y) {
        this.posY = -y;
        this.draw();
    };
    return CalendarBody;
}());
// カレンダースクロールバー
var CalendarScroll = /** @class */ (function () {
    function CalendarScroll() {
        this.id = "calscroll";
        this.width = 0;
        this.pos = 0;
        this.barWidth = 0;
        this.height = SCROLL_BAR_WIDTH;
        this.moving = false;
        this.startX = 0;
        this.barpos = 0;
        this.startPos = 0;
    }
    // スクロールバーを構築する
    CalendarScroll.prototype.build = function (frag) {
        this.width = frame.width - frame.calendarLeft;
        if (this.width < CALENDAR_MIN) {
            this.width = CALENDAR_MIN;
        }
        var cv = document.createElement("canvas");
        cv.id = this.id;
        cv.className = "scroll-bar";
        cv.style.top = "".concat(frame.height - SCROLL_BAR_WIDTH, "px");
        cv.style.left = "".concat(this.pos + frame.calendarLeft, "px");
        cv.style.width = "".concat(this.width, "px");
        cv.style.height = "".concat(this.height, "px");
        frag.append(cv);
    };
    CalendarScroll.prototype.resize = function () {
        this.width = frame.width - frame.calendarLeft;
        if (this.width < CALENDAR_MIN) {
            this.width = CALENDAR_MIN;
        }
        var cv = document.querySelector("#".concat(this.id));
        if (cv) {
            cv.style.width = "".concat(this.width, "px");
        }
        this.scrollH(0);
        this.barpos = 0;
    };
    // イベントハンドラを登録する
    CalendarScroll.prototype.handler = function () {
        var calscroll = document.querySelector("#".concat(this.id));
        if (calscroll) {
            calscroll.addEventListener("mousedown", function (e) {
                e.preventDefault();
                frame.mouseDownCalScroll(e.layerX);
            });
            calscroll.addEventListener("mousemove", function (e) {
                e.preventDefault();
                frame.mouseMoveCalScroll(e.layerX);
            });
            calscroll.addEventListener("mouseup", function (e) {
                e.preventDefault();
                frame.mouseUpCalScroll();
            });
            calscroll.addEventListener("mouseleave", function (e) {
                e.preventDefault();
                frame.mouseUpCalScroll();
            });
        }
    };
    // 描画する
    CalendarScroll.prototype.draw = function () {
        var cnvs = document.querySelector("#".concat(this.id));
        if (cnvs) {
            cnvs.width = this.width;
            cnvs.height = this.height;
            var ctx = cnvs.getContext("2d");
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
                        frame.calendarTotalWidth;
                ctx.fillRect(SCROLL_BAR_WIDTH + this.barpos, 2, this.barWidth < 4 ? 4 : this.barWidth, 13);
                ctx.restore();
            }
        }
    };
    CalendarScroll.prototype.scrollH = function (x) {
        this.pos = -x;
        var calscroll = document.querySelector("#".concat(this.id));
        if (calscroll) {
            calscroll.style.left = "".concat(this.pos + frame.calendarLeft, "px");
        }
    };
    CalendarScroll.prototype.mouseDownCalScroll = function (x) {
        if (SCROLL_BAR_WIDTH + this.barpos < x &&
            x < SCROLL_BAR_WIDTH + this.barpos + this.barWidth) {
            this.moving = true;
            this.startX = x - SCROLL_BAR_WIDTH;
            this.startPos = this.barpos;
            return;
        }
        if (x < SCROLL_BAR_WIDTH) {
            // 左ボタン
            this.barpos -= SCROLL_BAR_WIDTH;
        }
        else if (x < SCROLL_BAR_WIDTH + this.barpos) {
            // バーの左側
            this.barpos -= this.barWidth;
        }
        else if (this.width - SCROLL_BAR_WIDTH < x) {
            // 右ボタン
            this.barpos += SCROLL_BAR_WIDTH;
        }
        else {
            // バーの右側
            this.barpos += this.barWidth;
        }
        if (this.barpos < 0)
            this.barpos = 0;
        if (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth <
            this.barpos) {
            this.barpos =
                this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth;
        }
        frame.calendarHeader.scroll(this.barpos);
        this.draw();
        this.moving = false;
    };
    CalendarScroll.prototype.mouseMoveCalScroll = function (x) {
        if (!this.moving)
            return;
        this.barpos = x - SCROLL_BAR_WIDTH - this.startX + this.startPos;
        if (this.barpos < 0)
            this.barpos = 0;
        var w = this.width - this.barWidth - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH;
        if (this.barpos > w) {
            this.barpos = w;
        }
        frame.calendarHeader.scroll(this.barpos);
        this.draw();
    };
    CalendarScroll.prototype.mouseUpCalScroll = function () {
        this.moving = false;
    };
    return CalendarScroll;
}());
// 横スクロールバー
var ScrollH = /** @class */ (function () {
    function ScrollH() {
        this.id = "scrollh";
        this.width = 0;
        this.barWidth = 0;
        this.height = SCROLL_BAR_WIDTH;
        this.moving = false;
        this.startX = 0;
        this.pos = 0;
        this.startPos = 0;
    }
    // イベントハンドラを登録する
    ScrollH.prototype.handler = function () {
        var scrollh = document.querySelector("#".concat(this.id));
        if (scrollh) {
            scrollh.addEventListener("mousedown", function (e) {
                e.preventDefault();
                frame.mouseDownScrollH(e.pageX - scrollh.offsetLeft);
            });
            scrollh.addEventListener("mousemove", function (e) {
                e.preventDefault();
                frame.mouseMoveScrollH(e.pageX - scrollh.offsetLeft);
            });
            scrollh.addEventListener("mouseup", function (e) {
                e.preventDefault();
                frame.mouseUpScrollH();
            });
            scrollh.addEventListener("mouseleave", function (e) {
                e.preventDefault();
                frame.mouseUpScrollH();
            });
        }
    };
    ScrollH.prototype.draw = function () {
        var cnvs = document.querySelector("#".concat(this.id));
        if (cnvs) {
            this.width = cnvs.offsetWidth;
            cnvs.width = this.width;
            cnvs.height = this.height;
            if (this.width > frame.schThreshold)
                return;
            var ctx = cnvs.getContext("2d");
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
                        frame.schThreshold;
                ctx.fillRect(SCROLL_BAR_WIDTH + this.pos, 2, this.barWidth < 4 ? 4 : this.barWidth, 13);
                ctx.restore();
            }
        }
    };
    ScrollH.prototype.mouseDownScrollH = function (x) {
        if (SCROLL_BAR_WIDTH + this.pos < x &&
            x < SCROLL_BAR_WIDTH + this.pos + this.barWidth) {
            this.moving = true;
            this.startX = x - SCROLL_BAR_WIDTH;
            this.startPos = this.pos;
            return;
        }
        if (x < SCROLL_BAR_WIDTH) {
            // 左ボタン
            this.pos -= SCROLL_BAR_WIDTH;
        }
        else if (x < SCROLL_BAR_WIDTH + this.pos) {
            // バーの左側
            this.pos -= this.barWidth;
        }
        else if (this.width - SCROLL_BAR_WIDTH < x) {
            // 右ボタン
            this.pos += SCROLL_BAR_WIDTH;
        }
        else {
            // バーの右側
            this.pos += this.barWidth;
        }
        if (this.pos < 0)
            this.pos = 0;
        if (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth <
            this.pos) {
            this.pos =
                this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barWidth;
        }
        frame.scrollH(this.pos);
        this.draw();
        this.moving = false;
    };
    ScrollH.prototype.mouseMoveScrollH = function (x) {
        if (!this.moving)
            return;
        this.pos = x - SCROLL_BAR_WIDTH - this.startX + this.startPos;
        if (this.pos < 0)
            this.pos = 0;
        var w = this.width - this.barWidth - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH;
        if (this.pos > w) {
            this.pos = w;
        }
        frame.scrollH(this.pos);
        this.draw();
    };
    ScrollH.prototype.mouseUpScrollH = function () {
        this.moving = false;
    };
    return ScrollH;
}());
// 縦スクロールバー
var ScrollV = /** @class */ (function () {
    function ScrollV() {
        this.id = "scrollv";
        this.width = SCROLL_BAR_WIDTH;
        this.height = 0;
        this.barHeight = 0;
        this.moving = false;
        this.visible = false;
        this.startY = 0;
        this.pos = 0;
        this.startPos = 0;
    }
    // イベントハンドラを登録する
    ScrollV.prototype.handler = function () {
        var scrollv = document.querySelector("#".concat(this.id));
        if (scrollv) {
            scrollv.addEventListener("mousedown", function (e) {
                e.preventDefault();
                frame.mouseDownScrollV(e.pageY - scrollv.offsetTop);
            });
            scrollv.addEventListener("mousemove", function (e) {
                e.preventDefault();
                frame.mouseMoveScrollV(e.pageY - scrollv.offsetTop);
            });
            scrollv.addEventListener("mouseup", function (e) {
                e.preventDefault();
                frame.mouseUpScrollV();
            });
            scrollv.addEventListener("mouseleave", function (e) {
                e.preventDefault();
                frame.mouseUpScrollV();
            });
        }
    };
    ScrollV.prototype.draw = function () {
        this.visible = false;
        var cnvs = document.querySelector("#".concat(this.id));
        if (cnvs) {
            this.height = cnvs.offsetHeight;
            cnvs.width = this.width;
            cnvs.height = this.height;
            var ctx = cnvs.getContext("2d");
            if (ctx) {
                //ctx.fillStyle =
                //  frame.ticketsHeight < frame.ticketTotalHeight ? "#505050" : "#a3a3a3";
                if (frame.ticketsFrameHeight < frame.ticketsTotalHeight) {
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
                            frame.ticketsFrameHeight) /
                            frame.ticketsTotalHeight;
                    ctx.fillRect(2, SCROLL_BAR_WIDTH + this.pos, 13, this.barHeight < 4 ? 4 : this.barHeight);
                    ctx.restore();
                }
            }
        }
    };
    ScrollV.prototype.mouseDownScrollV = function (y) {
        if (!this.visible)
            return;
        if (SCROLL_BAR_WIDTH + this.pos < y &&
            y < SCROLL_BAR_WIDTH + this.pos + this.barHeight) {
            this.moving = true;
            this.startY = y - SCROLL_BAR_WIDTH;
            this.startPos = this.pos;
            return;
        }
        if (y < SCROLL_BAR_WIDTH) {
            // 上ボタン
            this.pos -= SCROLL_BAR_WIDTH;
        }
        else if (y < SCROLL_BAR_WIDTH + this.pos) {
            // バーの上
            this.pos -= this.barHeight;
        }
        else if (this.height - SCROLL_BAR_WIDTH < y) {
            // 下ボタン
            this.pos += SCROLL_BAR_WIDTH;
        }
        else {
            // バーの下
            this.pos += this.barHeight;
        }
        if (this.pos < 0)
            this.pos = 0;
        var percent = this.pos /
            (this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight);
        if (percent > 1) {
            percent = 1;
            this.pos =
                this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight;
        }
        frame.scrollV((frame.ticketsTotalHeight - frame.ticketsFrameHeight) * percent);
        this.draw();
        this.moving = false;
    };
    ScrollV.prototype.mouseMoveScrollV = function (y) {
        if (!this.visible || !this.moving)
            return;
        this.pos = y - SCROLL_BAR_WIDTH - this.startY + this.startPos;
        if (this.pos < 0)
            this.pos = 0;
        var percent = this.pos /
            (this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight);
        if (percent > 1) {
            percent = 1;
            this.pos =
                this.height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH - this.barHeight;
        }
        frame.scrollV((frame.ticketsTotalHeight - frame.ticketsFrameHeight) * percent);
        this.draw();
    };
    ScrollV.prototype.mouseUpScrollV = function () {
        this.moving = false;
    };
    return ScrollV;
}());
// ガントチャート全体フレーム
var GanttFrame = /** @class */ (function () {
    function GanttFrame() {
        this.id = "ganttframe";
        this.width = 0;
        this.height = 0;
        this.columnHeader = new ColumnHeader();
        this.columnBody = new ColumnBody();
        this.columnScroll = new ColumnScroll();
        this.calendarHeader = new CalendarHeader();
        this.calendarBody = new CalendarBody();
        this.calendarScroll = new CalendarScroll();
        this.sch = new ScrollH();
        this.scv = new ScrollV();
        this.colW = 0;
        this.calendarLeft = 0;
        this.calendarStart = new Date(2024, 5, 24);
        this.calendarEnd = new Date(2024, 7, 31);
        this.calendarTotalWidth = 0;
        this.ticketsTotalHeight = 0;
        this.ticketsFrameHeight = 0;
        this.schThreshold = 0; // 横スクロールバーを表示するしきい値
        this.posX = 0;
    }
    // ガントチャートを構築する
    GanttFrame.prototype.build = function () {
        var frame = document.querySelector("#".concat(this.id));
        if (!frame) {
            console.error("Failed to get #ganttframe!");
            return;
        }
        this.width = frame.offsetWidth;
        this.height = frame.offsetHeight;
        this.colW = this.columnWidth();
        this.calendarLeft = this.colW + 1;
        this.schThreshold = this.calendarLeft + CALENDAR_MIN; // フレームの幅がこれよりも小さければ横スクロールバーを表示する
        var frag = document.createDocumentFragment();
        this.columnBody.build(frag);
        this.columnHeader.build(frag);
        this.columnScroll.build(frag);
        this.calendarBody.build(frag);
        this.calendarHeader.build(frag);
        this.calendarScroll.build(frag);
        // カレンダーの開始日と終了日の差分から全体の幅を求める
        var diff = (this.calendarEnd.getTime() - this.calendarStart.getTime()) / DAY_MILISEC;
        this.calendarTotalWidth = (diff + 1) * DAY_WIDTH;
        frame.append(frag);
    };
    // イベントハンドラを登録する
    GanttFrame.prototype.handler = function () {
        this.columnBody.handler();
        this.calendarScroll.handler();
        this.sch.handler();
        this.scv.handler();
    };
    GanttFrame.prototype.resize = function () {
        var flexbox = document.querySelector("#flexbox");
        if (!flexbox) {
            console.error("Failed to get #flexbox!");
            return;
        }
        var ganttbase = document.querySelector("#ganttbase");
        if (!ganttbase) {
            console.error("Failed to get #ganttbase!");
            return;
        }
        ganttbase.style.width = "".concat(flexbox.offsetWidth - SCROLL_BAR_WIDTH, "px");
        var frame = document.querySelector("#".concat(this.id));
        if (!frame) {
            console.error("Failed to get #ganttframe!");
            return;
        }
        //ganttbase.style.width = `${flexbox.offsetWidth - SCROLL_BAR_WIDTH}px`;
        this.width = frame.offsetWidth;
        this.height = frame.offsetHeight;
        this.colW = this.columnWidth();
        this.calendarLeft = this.colW + 1;
        this.schThreshold = this.calendarLeft + CALENDAR_MIN;
        this.sch.pos = 0;
        this.calendarScroll.pos = 0;
        this.calendarBody.resize();
        this.calendarHeader.resize();
        this.calendarScroll.resize();
        this.scrollH(0);
    };
    // ガントチャートを表示する
    GanttFrame.prototype.draw = function () {
        this.columnBody.draw();
        this.calendarHeader.draw();
        this.calendarBody.draw();
        this.calendarScroll.draw();
        this.sch.draw();
        this.scv.draw();
    };
    GanttFrame.prototype.scrollH = function (x) {
        this.posX =
            (x * this.schThreshold) /
                (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
        this.columnBody.scrollH(this.posX);
        this.columnHeader.scrollH(this.posX);
        this.columnScroll.scrollH(this.posX);
        this.calendarHeader.scrollH(this.posX);
        this.calendarBody.scrollH(this.posX);
        this.calendarScroll.scrollH(this.posX);
    };
    GanttFrame.prototype.scrollV = function (y) {
        this.columnBody.scrollV(y);
        this.calendarBody.scrollV(y);
    };
    GanttFrame.prototype.columnWidth = function () {
        var w = 2;
        for (var _i = 0, cols_3 = cols; _i < cols_3.length; _i++) {
            var col = cols_3[_i];
            w += col.width;
        }
        return w;
    };
    GanttFrame.prototype.mouseDownCalScroll = function (x) {
        this.calendarScroll.mouseDownCalScroll(x);
    };
    GanttFrame.prototype.mouseMoveCalScroll = function (x) {
        this.calendarScroll.mouseMoveCalScroll(x);
    };
    GanttFrame.prototype.mouseUpCalScroll = function () {
        this.calendarScroll.mouseUpCalScroll();
    };
    GanttFrame.prototype.mouseDownScrollH = function (x) {
        this.sch.mouseDownScrollH(x);
    };
    GanttFrame.prototype.mouseMoveScrollH = function (x) {
        this.sch.mouseMoveScrollH(x);
    };
    GanttFrame.prototype.mouseUpScrollH = function () {
        this.sch.mouseUpScrollH();
    };
    GanttFrame.prototype.mouseDownScrollV = function (y) {
        this.scv.mouseDownScrollV(y);
    };
    GanttFrame.prototype.mouseMoveScrollV = function (y) {
        this.scv.mouseMoveScrollV(y);
    };
    GanttFrame.prototype.mouseUpScrollV = function () {
        this.scv.mouseUpScrollV();
    };
    return GanttFrame;
}());
var frame = new GanttFrame();
frame.build();
frame.handler();
frame.draw();
window.addEventListener("resize", function () {
    frame.resize();
    frame.draw();
});
