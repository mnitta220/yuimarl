var _a, _b, _c, _d;
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
        width: 70,
    },
    {
        name: "チケット名",
        width: 300,
    },
    {
        name: "開始日",
        width: 100,
    },
    {
        name: "終了日",
        width: 100,
    },
    {
        name: "進捗率",
        width: 70,
    },
];
var SCROLL_BAR_WIDTH = 16;
var HEADER_LABEL_Y = 42;
var LINE_HEIGHT = 21;
var HEADER_HEIGHT = 63;
var DAY_WIDTH = 22;
// カラムヘッダー
var ColumnHeader = /** @class */ (function () {
    function ColumnHeader() {
        this.x1 = 0;
    }
    // カラムヘッダーを表示する
    ColumnHeader.prototype.render = function (frame, frag) {
        var hd = document.createElement("div");
        hd.className = "header";
        hd.style.top = "0px";
        hd.style.left = "".concat(this.x1, "px");
        hd.style.width = "".concat(frame.colW, "px");
        hd.style.height = "".concat(HEADER_HEIGHT, "px");
        frag.append(hd);
        var line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(HEADER_HEIGHT, "px");
        line.style.left = "0px";
        line.style.width = "100%";
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
        frag.append(line);
    };
    return ColumnHeader;
}());
// カラムボディ
var ColumnBody = /** @class */ (function () {
    function ColumnBody() {
    }
    // カラムボディを表示する
    ColumnBody.prototype.render = function (frame, frag) {
        var bar = document.createElement("div");
        var height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
        bar.className = "gantt-body";
        bar.style.top = "".concat(HEADER_HEIGHT, "px");
        bar.style.left = "0px";
        bar.style.width = "".concat(frame.calendarLeft, "px");
        bar.style.height = "".concat(height, "px");
        frag.append(bar);
        var x = 0;
        for (var _i = 0, cols_2 = cols; _i < cols_2.length; _i++) {
            var col = cols_2[_i];
            x += col.width;
            // カラム区切り線
            var line_1 = document.createElement("div");
            line_1.className = "line";
            line_1.style.top = "0px";
            line_1.style.left = "".concat(x, "px");
            line_1.style.width = "1px";
            line_1.style.height = "".concat(height, "px");
            bar.append(line_1);
        }
        // カレンダー境界線
        x += 2;
        var line = document.createElement("div");
        line.className = "line";
        line.style.top = "0px";
        line.style.left = "".concat(x, "px");
        line.style.width = "1px";
        line.style.height = "".concat(height, "px");
        bar.append(line);
    };
    return ColumnBody;
}());
// カラムスクロールバー
var ColumnScroll = /** @class */ (function () {
    function ColumnScroll() {
    }
    // スクロールバーを表示する
    ColumnScroll.prototype.render = function (frame, frag) {
        var bar = document.createElement("div");
        bar.className = "scroll-corner";
        bar.style.top = "".concat(frame.height - SCROLL_BAR_WIDTH, "px");
        bar.style.left = "0px";
        bar.style.width = "".concat(frame.calendarLeft, "px");
        bar.style.height = "".concat(SCROLL_BAR_WIDTH, "px");
        frag.append(bar);
    };
    return ColumnScroll;
}());
// カレンダーヘッダー
var CalendarHeader = /** @class */ (function () {
    function CalendarHeader() {
        this.width = 0;
        this.pos = 0;
        this.dtStart = 0;
        this.dtEnd = 0;
    }
    // カレンダーヘッダーを表示する
    CalendarHeader.prototype.render = function (frame, frag) {
        var cv = document.createElement("canvas");
        this.width = frame.width - frame.calendarLeft;
        cv.id = "calhead";
        cv.className = "header";
        cv.style.top = "0px";
        cv.style.left = "".concat(frame.calendarLeft, "px");
        cv.style.width = "".concat(this.width, "px");
        cv.style.height = "".concat(HEADER_HEIGHT + 1, "px");
        frag.append(cv);
        this.dtStart = frame.calendarStart.getTime();
        this.dtEnd = frame.calendarEnd.getTime();
    };
    // 描画する
    CalendarHeader.prototype.draw = function () {
        var cnvs = document.querySelector("#calhead");
        if (cnvs) {
            var height = cnvs.offsetHeight;
            cnvs.width = this.width;
            cnvs.height = height;
            var dtTop = LINE_HEIGHT + LINE_HEIGHT;
            var ctx = cnvs.getContext("2d");
            var font2 = "9.5pt sans-serif";
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
                var dt = new Date(this.dtStart);
                var x = 0;
                ctx.font = font2;
                ctx.textBaseline = "bottom";
                ctx.textAlign = "left";
                while (dt.getTime() <= this.dtEnd) {
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
                    ctx.fillText("".concat(dt.getDate()), x + 3 - this.pos, HEADER_HEIGHT - 3);
                    x += DAY_WIDTH;
                    if (x > this.width + this.pos)
                        break;
                    dt.setDate(dt.getDate() + 1);
                    // 日付区切り線
                    ctx.fillStyle = "#bdcede";
                    ctx.fillRect(x - this.pos, dtTop, 1, LINE_HEIGHT);
                    ctx.fill();
                }
                ctx.restore();
            }
        }
    };
    CalendarHeader.prototype.scroll = function (frame, x) {
        this.pos =
            (x * frame.calendarTotalWidth) /
                (this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
        this.draw();
        frame.calendarBody.pos = this.pos;
        frame.calendarBody.draw();
    };
    return CalendarHeader;
}());
// カレンダーボディ
var CalendarBody = /** @class */ (function () {
    function CalendarBody() {
        this.width = 0;
        this.pos = 0;
        this.dtStart = 0;
        this.dtEnd = 0;
    }
    // カレンダーボディを表示する
    CalendarBody.prototype.render = function (frame, frag) {
        var cv = document.createElement("canvas");
        this.width = frame.width - frame.calendarLeft;
        var height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
        cv.id = "calbody";
        cv.className = "gantt-body";
        cv.style.top = "".concat(HEADER_HEIGHT, "px");
        cv.style.left = "".concat(frame.calendarLeft, "px");
        cv.style.width = "".concat(this.width, "px");
        cv.style.height = "".concat(height, "px");
        frag.append(cv);
        this.dtStart = frame.calendarStart.getTime();
        this.dtEnd = frame.calendarEnd.getTime();
    };
    // 描画する
    CalendarBody.prototype.draw = function () {
        var cnvs = document.querySelector("#calbody");
        if (cnvs) {
            var width = cnvs.offsetWidth;
            var height = cnvs.offsetHeight;
            cnvs.width = width;
            cnvs.height = height;
            var ctx = cnvs.getContext("2d");
            if (ctx) {
                ctx.save();
                ctx.fillStyle = "#bdcede";
                var dt = new Date(this.dtStart);
                var x = 0;
                while (dt.getTime() <= this.dtEnd) {
                    dt.setDate(dt.getDate() + 1);
                    x += DAY_WIDTH;
                    if (x > width + this.pos)
                        break;
                    // 日付区切り線
                    ctx.fillRect(x - this.pos, 0, 1, height);
                    ctx.fill();
                }
                ctx.restore();
            }
        }
    };
    return CalendarBody;
}());
// カレンダースクロールバー
var CalendarScroll = /** @class */ (function () {
    function CalendarScroll() {
        this.width = 0;
        this.barWidth = 0;
        this.height = SCROLL_BAR_WIDTH;
        this.moving = false;
        this.startX = 0;
        this.pos = 0;
        this.startPos = 0;
        this.calendarTotalWidth = 0;
    }
    // スクロールバーを表示する
    CalendarScroll.prototype.render = function (frame, frag) {
        this.width = frame.width - frame.calendarLeft;
        this.calendarTotalWidth = frame.calendarTotalWidth;
        var cv = document.createElement("canvas");
        cv.id = "calscroll";
        cv.className = "scroll-bar";
        cv.style.top = "".concat(frame.height - SCROLL_BAR_WIDTH, "px");
        cv.style.left = "".concat(frame.calendarLeft, "px");
        cv.style.width = "".concat(this.width, "px");
        cv.style.height = "".concat(this.height, "px");
        frag.append(cv);
    };
    // 描画する
    CalendarScroll.prototype.draw = function () {
        var cnvs = document.querySelector("#calscroll");
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
                ctx.fillRect(SCROLL_BAR_WIDTH + this.pos, 2, this.barWidth, 13);
                ctx.restore();
            }
        }
    };
    CalendarScroll.prototype.mouseDownCalScroll = function (x) {
        if (x < SCROLL_BAR_WIDTH + this.pos ||
            SCROLL_BAR_WIDTH + this.pos + this.barWidth < x) {
            return;
        }
        this.moving = true;
        this.startX = x - SCROLL_BAR_WIDTH;
        this.startPos = this.pos;
    };
    CalendarScroll.prototype.mouseMoveCalScroll = function (frame, x) {
        if (!this.moving)
            return;
        this.pos = x - SCROLL_BAR_WIDTH - this.startX + this.startPos;
        if (this.pos < 0)
            this.pos = 0;
        var w = this.width - this.barWidth - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH;
        if (this.pos > w) {
            this.pos = w;
        }
        frame.calendarHeader.scroll(frame, this.pos);
        this.draw();
    };
    CalendarScroll.prototype.mouseUpCalScroll = function () {
        this.moving = false;
    };
    return CalendarScroll;
}());
// ガントチャート全体フレーム
var GanttFrame = /** @class */ (function () {
    function GanttFrame() {
        this.width = 0;
        this.height = 0;
        this.columnHeader = new ColumnHeader();
        this.columnBody = new ColumnBody();
        this.columnScroll = new ColumnScroll();
        this.calendarHeader = new CalendarHeader();
        this.calendarBody = new CalendarBody();
        this.calendarScroll = new CalendarScroll();
        this.colW = 0;
        this.calendarLeft = 0;
        this.calendarStart = new Date(2024, 5, 30);
        this.calendarEnd = new Date(2024, 7, 31);
        this.calendarTotalWidth = 0;
    }
    // ガントチャートを表示する
    GanttFrame.prototype.render = function () {
        var frame = document.querySelector("#ganttframe");
        if (!frame) {
            console.error("Failed to get #ganttframe!");
            return;
        }
        this.width = frame.offsetWidth;
        this.height = frame.offsetHeight;
        this.colW = this.columnWidth();
        this.calendarLeft = this.colW + 1;
        var frag = document.createDocumentFragment();
        this.columnBody.render(this, frag);
        this.columnHeader.render(this, frag);
        this.columnScroll.render(this, frag);
        this.calendarBody.render(this, frag);
        this.calendarHeader.render(this, frag);
        this.calendarScroll.render(this, frag);
        // カレンダーの開始日と終了日の差分から全体の幅を求める。
        var diff = (this.calendarEnd.getTime() - this.calendarStart.getTime()) /
            (1000 * 60 * 60 * 24);
        this.calendarTotalWidth = (diff + 1) * DAY_WIDTH;
        frame.append(frag);
    };
    GanttFrame.prototype.draw = function () {
        this.calendarHeader.draw();
        this.calendarBody.draw();
        this.calendarScroll.draw();
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
        this.calendarScroll.mouseMoveCalScroll(this, x);
    };
    GanttFrame.prototype.mouseUpCalScroll = function () {
        this.calendarScroll.mouseUpCalScroll();
    };
    return GanttFrame;
}());
// 横スクロールバー
var ScrollH = /** @class */ (function () {
    function ScrollH() {
    }
    ScrollH.prototype.draw = function () {
        var cnvs = document.querySelector("#scrollh");
        if (cnvs) {
            var width = cnvs.offsetWidth;
            var height = cnvs.offsetHeight;
            cnvs.width = width;
            cnvs.height = height;
            var ctx = cnvs.getContext("2d");
            if (ctx) {
                ctx.save();
                //ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
                ctx.lineJoin = "miter";
                ctx.fillStyle = "#505050";
                ctx.beginPath();
                ctx.moveTo(5, 8);
                ctx.lineTo(9, 12);
                ctx.lineTo(9, 4);
                ctx.closePath();
                ctx.fill();
                ctx.beginPath();
                ctx.moveTo(width - 10, 4);
                ctx.lineTo(width - 10, 12);
                ctx.lineTo(width - 6, 8);
                ctx.closePath();
                ctx.fill();
                //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
                ctx.fillStyle = "#c1c1c1";
                ctx.fillRect(16, 2, width - 32, 13);
                ctx.restore();
            }
        }
    };
    return ScrollH;
}());
// 縦スクロールバー
var ScrollV = /** @class */ (function () {
    function ScrollV() {
    }
    ScrollV.prototype.draw = function () {
        var cnvs = document.querySelector("#scrollv");
        if (cnvs) {
            var width = cnvs.offsetWidth;
            var height = cnvs.offsetHeight;
            cnvs.width = width;
            cnvs.height = height;
            var ctx = cnvs.getContext("2d");
            if (ctx) {
                ctx.save();
                //ctx.fillStyle = scroll_h ? "#505050" : "#a3a3a3";
                ctx.lineJoin = "miter";
                ctx.fillStyle = "#505050";
                ctx.beginPath();
                ctx.moveTo(8, 5);
                ctx.lineTo(4, 9);
                ctx.lineTo(12, 9);
                ctx.closePath();
                ctx.fill();
                ctx.beginPath();
                ctx.moveTo(4, height - 10);
                ctx.lineTo(8, height - 6);
                ctx.lineTo(12, height - 10);
                ctx.closePath();
                ctx.fill();
                //ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
                ctx.fillStyle = "#c1c1c1";
                ctx.fillRect(2, 16, 13, height - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH);
                ctx.restore();
            }
        }
    };
    return ScrollV;
}());
var frame = new GanttFrame();
frame.render();
frame.draw();
var sch = new ScrollH();
sch.draw();
var scv = new ScrollV();
scv.draw();
(_a = document
    .querySelector("#calscroll")) === null || _a === void 0 ? void 0 : _a.addEventListener("mousedown", function (e) {
    e.preventDefault();
    frame.mouseDownCalScroll(e.layerX);
});
(_b = document
    .querySelector("#calscroll")) === null || _b === void 0 ? void 0 : _b.addEventListener("mousemove", function (e) {
    e.preventDefault();
    frame.mouseMoveCalScroll(e.layerX);
});
(_c = document
    .querySelector("#calscroll")) === null || _c === void 0 ? void 0 : _c.addEventListener("mouseup", function (e) {
    e.preventDefault();
    frame.mouseUpCalScroll();
});
(_d = document
    .querySelector("#calscroll")) === null || _d === void 0 ? void 0 : _d.addEventListener("mouseleave", function (e) {
    e.preventDefault();
    frame.mouseUpCalScroll();
});
