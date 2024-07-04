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
var HEADER_LABEL_Y = 46;
var LINE_HEIGHT = 22;
var HEADER_HEIGHT = 66;
// カラムヘッダー
var ColumnHeader = /** @class */ (function () {
    function ColumnHeader() {
        this.x1 = 0;
    }
    //y1 = 0;
    //x2 = 0;
    //y2 = 0;
    // カラムヘッダーを表示する
    ColumnHeader.prototype.render = function (frame, frag) {
        //const headerHeight = LINE_HEIGHT * 3;
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
        var calendarX = x;
        line = document.createElement("div");
        line.className = "line";
        line.style.top = "0px";
        line.style.left = "".concat(calendarX, "px");
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
        //let calendarX = x;
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
        this.x1 = 0;
    }
    // カレンダーヘッダーを表示する
    CalendarHeader.prototype.render = function (frame, frag) {
        var hd = document.createElement("div");
        var width = frame.width - frame.calendarLeft;
        hd.className = "header";
        hd.style.top = "0px";
        hd.style.left = "".concat(frame.calendarLeft, "px");
        hd.style.width = "".concat(width, "px");
        hd.style.height = "".concat(HEADER_HEIGHT, "px");
        frag.append(hd);
        var line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(HEADER_HEIGHT, "px");
        line.style.left = "".concat(frame.calendarLeft, "px");
        line.style.width = "".concat(width, "px");
        line.style.height = "1px";
        frag.append(line);
        // カレンダーヘッダー横線1
        line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(LINE_HEIGHT, "px");
        line.style.left = "".concat(frame.calendarLeft, "px");
        line.style.width = "".concat(width, "px");
        line.style.height = "1px";
        frag.append(line);
        // カレンダーヘッダー横線2
        line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(LINE_HEIGHT + LINE_HEIGHT, "px");
        line.style.left = "".concat(frame.calendarLeft, "px");
        line.style.width = "".concat(width, "px");
        line.style.height = "1px";
        frag.append(line);
        var dt = new Date(frame.calendarStart.getTime()); //this.start;
        var x = 0;
        var dtTop = LINE_HEIGHT + LINE_HEIGHT;
        //console.log(dt.toLocaleDateString());
        while (dt.getTime() <= frame.calendarEnd.getTime()) {
            // カラムラベル
            var label = document.createElement("div");
            label.className = "caldt";
            label.style.top = "".concat(HEADER_LABEL_Y, "px");
            label.style.left = "".concat(x + 3, "px");
            label.textContent = "".concat(dt.getDate());
            hd.append(label);
            dt.setDate(dt.getDate() + 1);
            x += 22;
            // 日付区切り線
            var line_2 = document.createElement("div");
            line_2.className = "dtline";
            line_2.style.top = "".concat(dtTop, "px");
            line_2.style.left = "".concat(x, "px");
            line_2.style.width = "1px";
            line_2.style.height = "".concat(LINE_HEIGHT, "px");
            hd.append(line_2);
        }
    };
    return CalendarHeader;
}());
// カレンダーボディ
var CalendarBody = /** @class */ (function () {
    function CalendarBody() {
    }
    // カレンダーボディを表示する
    CalendarBody.prototype.render = function (frame, frag) {
        var body = document.createElement("div");
        var height = frame.height - HEADER_HEIGHT - SCROLL_BAR_WIDTH;
        body.className = "gantt-body";
        body.style.top = "".concat(HEADER_HEIGHT, "px");
        body.style.left = "".concat(frame.calendarLeft, "px");
        body.style.width = "".concat(frame.width - frame.calendarLeft, "px");
        body.style.height = "".concat(height, "px");
        frag.append(body);
        var dt = new Date(frame.calendarStart.getTime());
        var x = 0;
        while (dt.getTime() <= frame.calendarEnd.getTime()) {
            dt.setDate(dt.getDate() + 1);
            x += 22;
            // 日付区切り線
            var line = document.createElement("div");
            line.className = "dtline";
            line.style.top = "0px";
            line.style.left = "".concat(x, "px");
            line.style.width = "1px";
            line.style.height = "".concat(height, "px");
            body.append(line);
        }
    };
    return CalendarBody;
}());
// カレンダースクロールバー
var CalendarScroll = /** @class */ (function () {
    function CalendarScroll() {
    }
    // スクロールバーを表示する
    CalendarScroll.prototype.render = function (frame, frag) {
        var bar = document.createElement("div");
        bar.className = "scroll-bar";
        bar.style.top = "".concat(frame.height - SCROLL_BAR_WIDTH, "px");
        bar.style.left = "".concat(frame.calendarLeft, "px");
        bar.style.width = "".concat(frame.width - frame.calendarLeft, "px");
        bar.style.height = "".concat(SCROLL_BAR_WIDTH, "px");
        frag.append(bar);
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
        frame.append(frag);
    };
    GanttFrame.prototype.columnWidth = function () {
        var w = 2;
        for (var _i = 0, cols_3 = cols; _i < cols_3.length; _i++) {
            var col = cols_3[_i];
            w += col.width;
        }
        return w;
    };
    return GanttFrame;
}());
var f = new GanttFrame();
f.render();
