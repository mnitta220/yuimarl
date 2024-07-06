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
        this.pos = 0;
    }
    // カラムボディを構築する
    ColumnBody.prototype.build = function (frag) {
        var body = document.createElement("div");
        var height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
        body.id = this.id;
        body.className = "gantt-body";
        body.style.top = "".concat(HEADER_HEIGHT, "px");
        body.style.left = "".concat(this.pos, "px");
        body.style.width = "".concat(frame.calendarLeft, "px");
        body.style.height = "".concat(height, "px");
        frag.append(body);
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
            body.append(line_1);
        }
        // カレンダー境界線
        x += 2;
        var line = document.createElement("div");
        line.className = "line";
        line.style.top = "0px";
        line.style.left = "".concat(x, "px");
        line.style.width = "1px";
        line.style.height = "".concat(height, "px");
        body.append(line);
    };
    ColumnBody.prototype.scrollH = function (x) {
        this.pos = -x;
        var colbody = document.querySelector("#".concat(this.id));
        if (colbody) {
            colbody.style.left = "".concat(this.pos, "px");
        }
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
                var dt = new Date(this.dtStart);
                var x = 0;
                var first = true;
                ctx.font = font;
                ctx.textBaseline = "bottom";
                ctx.textAlign = "left";
                while (dt.getTime() <= this.dtEnd) {
                    if (x < this.dtpos - DAY_WIDTH) {
                        x += DAY_WIDTH;
                        dt.setDate(dt.getDate() + 1);
                        first = false;
                        continue;
                    }
                    if (x > this.width + this.dtpos)
                        break;
                    var date = dt.getDate();
                    if (date == 1 || (first && date < 25)) {
                        ctx.fillStyle = "#000";
                        ctx.fillText("".concat(dt.getFullYear(), "/").concat(dt.getMonth() + 1), x + 3 - this.dtpos, LINE_HEIGHT + LINE_HEIGHT - 3);
                        ctx.fillStyle = "#bdcede";
                        ctx.fillRect(x - this.dtpos, LINE_HEIGHT, 1, LINE_HEIGHT);
                        ctx.fill();
                    }
                    first = false;
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
                    ctx.fillText("".concat(date), x + 3 - this.dtpos, HEADER_HEIGHT - 3);
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
        //console.log(`scroll ${x} ${this.dtpos}`);
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
        this.pos = 0;
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
        cv.style.left = "".concat(this.pos + frame.calendarLeft, "px");
        cv.style.width = "".concat(this.width, "px");
        cv.style.height = "".concat(height, "px");
        frag.append(cv);
        this.dtStart = frame.calendarStart.getTime();
        this.dtEnd = frame.calendarEnd.getTime();
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
                ctx.fillStyle = "#bdcede";
                var dt = new Date(this.dtStart);
                var x = 0;
                while (dt.getTime() <= this.dtEnd) {
                    if (x < this.dtpos - DAY_WIDTH) {
                        x += DAY_WIDTH;
                        dt.setDate(dt.getDate() + 1);
                        continue;
                    }
                    if (x > this.width + this.dtpos)
                        break;
                    dt.setDate(dt.getDate() + 1);
                    x += DAY_WIDTH;
                    if (x > width + this.dtpos)
                        break;
                    // 日付区切り線
                    ctx.fillRect(x - this.dtpos, 0, 1, height);
                    ctx.fill();
                }
                ctx.restore();
            }
        }
    };
    CalendarBody.prototype.scrollH = function (x) {
        this.pos = -x;
        var calbody = document.querySelector("#".concat(this.id));
        if (calbody) {
            calbody.style.left = "".concat(this.pos + frame.calendarLeft, "px");
        }
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
        //debugger;
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
            //const height = cnvs.offsetHeight;
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
                //ctx.fillRect(16, 2, this.width - 32, 13);
                this.barWidth =
                    ((this.width - SCROLL_BAR_WIDTH - SCROLL_BAR_WIDTH) * this.width) /
                        frame.schThreshold;
                ctx.fillRect(SCROLL_BAR_WIDTH + this.pos, 2, this.barWidth < 4 ? 4 : this.barWidth, 13);
                ctx.restore();
            }
        }
    };
    ScrollH.prototype.mouseDownScrollH = function (x) {
        //console.log(`mouseDownScrollH: ${x} ${this.pos} ${this.barWidth}`);
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
    }
    ScrollV.prototype.draw = function () {
        var cnvs = document.querySelector("#".concat(this.id));
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
        this.calendarScroll.handler();
        this.sch.handler();
    };
    // ガントチャートを表示する
    GanttFrame.prototype.draw = function () {
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
    return GanttFrame;
}());
var frame = new GanttFrame();
frame.build();
frame.handler();
frame.draw();
