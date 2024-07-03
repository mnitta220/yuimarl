//import * as dayjs from "dayjs";
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
// スクロールバー
var ScrollBar = /** @class */ (function () {
    function ScrollBar() {
        this.x1 = 0;
        this.y1 = 0;
        this.x2 = 0;
        this.y2 = 0;
    }
    // スクロールバーを表示する
    ScrollBar.prototype.render = function (frag) {
        var bar = document.createElement("div");
        bar.className = "scroll-bar";
        bar.style.top = "".concat(this.y1, "px");
        bar.style.left = "".concat(this.x1 + 1, "px");
        bar.style.width = "".concat(this.x2 - this.x1, "px");
        bar.style.height = "".concat(this.y2 - this.y1, "px");
        frag.append(bar);
    };
    return ScrollBar;
}());
// カレンダー
var Calendar = /** @class */ (function () {
    function Calendar() {
        this.start = new Date(2024, 5, 30);
        this.end = new Date(2024, 7, 31);
    }
    // カレンダーを表示する
    Calendar.prototype.render = function (frame, frag, startX) {
        var dt = this.start;
        var x = startX;
        var dtTop = LINE_HEIGHT + LINE_HEIGHT;
        console.log(dt.toLocaleDateString());
        while (dt.getTime() <= this.end.getTime()) {
            // カラムラベル
            var label = document.createElement("div");
            label.className = "caldt";
            label.style.top = "".concat(HEADER_LABEL_Y, "px");
            label.style.left = "".concat(x + 3, "px");
            label.textContent = "".concat(dt.getDate());
            frag.append(label);
            dt.setDate(dt.getDate() + 1);
            x += 22;
            // 日付区切り線
            var line = document.createElement("div");
            line.className = "dtline";
            line.style.top = "".concat(dtTop, "px");
            line.style.left = "".concat(x, "px");
            line.style.width = "1px";
            line.style.height = "".concat(frame.height - dtTop, "px");
            frag.append(line);
        }
    };
    return Calendar;
}());
// ガントチャート全体フレーム
var Frame = /** @class */ (function () {
    function Frame() {
        this.width = 0;
        this.height = 0;
        this.calendar = new Calendar();
        this.scrollBarH = new ScrollBar();
        this.scrollBarV = new ScrollBar();
    }
    // ガントチャートを表示する
    Frame.prototype.render = function () {
        var chart = document.querySelector("#chart");
        var headerHeight = LINE_HEIGHT * 3;
        if (!chart) {
            console.error("Failed to get #chart!");
            return;
        }
        this.width = chart.offsetWidth;
        this.height = chart.offsetHeight;
        var frag = document.createDocumentFragment();
        // ヘッダー
        var header = document.createElement("div");
        header.className = "header";
        header.style.top = "0px";
        header.style.left = "0px";
        header.style.width = "100%";
        header.style.height = "".concat(headerHeight, "px");
        frag.append(header);
        var line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(headerHeight, "px");
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
            header.append(label);
            x += col.width;
            // カラム区切り線
            line = document.createElement("div");
            line.className = "line";
            line.style.top = "0px";
            line.style.left = "".concat(x, "px");
            line.style.width = "1px";
            line.style.height = "100%";
            frag.append(line);
        }
        // カレンダー境界線
        x += 2;
        var calendarX = x;
        line = document.createElement("div");
        line.className = "line";
        line.style.top = "0px";
        line.style.left = "".concat(calendarX, "px");
        line.style.width = "1px";
        line.style.height = "100%";
        frag.append(line);
        this.calendar.render(this, frag, x);
        // カレンダーヘッダー横線1
        line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(LINE_HEIGHT, "px");
        line.style.left = "".concat(calendarX, "px");
        line.style.width = "".concat(chart.offsetWidth - calendarX, "px");
        line.style.height = "1px";
        frag.append(line);
        // カレンダーヘッダー横線2
        line = document.createElement("div");
        line.className = "line";
        line.style.top = "".concat(LINE_HEIGHT + LINE_HEIGHT, "px");
        line.style.left = "".concat(calendarX, "px");
        line.style.width = "".concat(chart.offsetWidth - calendarX, "px");
        line.style.height = "1px";
        frag.append(line);
        // 横スクロールバー
        this.scrollBarH.x1 = calendarX;
        this.scrollBarH.y1 = this.height - SCROLL_BAR_WIDTH;
        this.scrollBarH.x2 = this.width - SCROLL_BAR_WIDTH;
        this.scrollBarH.y2 = this.height;
        this.scrollBarH.render(frag);
        // 縦スクロールバー
        this.scrollBarV.x1 = this.width - SCROLL_BAR_WIDTH;
        this.scrollBarV.y1 = LINE_HEIGHT * 3 + 1;
        this.scrollBarV.x2 = this.width;
        this.scrollBarV.y2 = this.height - SCROLL_BAR_WIDTH;
        this.scrollBarV.render(frag);
        // スクロールバーの角
        var corner = document.createElement("div");
        corner.className = "scroll-corner";
        corner.style.top = "".concat(this.height - SCROLL_BAR_WIDTH, "px");
        corner.style.left = "".concat(this.width - SCROLL_BAR_WIDTH + 1, "px");
        corner.style.width = "".concat(SCROLL_BAR_WIDTH, "px");
        corner.style.height = "".concat(SCROLL_BAR_WIDTH, "px");
        frag.append(corner);
        chart.append(frag);
    };
    return Frame;
}());
//let dt = dayjs();
var f = new Frame();
f.render();
