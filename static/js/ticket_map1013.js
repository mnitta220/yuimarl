var tickets = [
    {
        id_disp: "YU60",
        name: "チケットマップ",
        x: 30,
        y: 200,
        color: "warning",
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    },
    {
        id_disp: "YU17",
        name: "WBS",
        x: 50,
        y: 150,
        color: "info",
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    },
    {
        id_disp: "YU38",
        name: "更新バリデーション",
        x: -250,
        y: -50,
        color: "light",
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    },
];

var selected = null;
var start_x = 0;
var start_y = 0;
var max_x = 2000;
var min_x = -2000;
var max_y = 500;
var min_y = -500;
var scroll_v = true;
var scroll_h = true;
var scroll_width = max_x - min_x;
var scroll_height = max_y - min_y;
var scroll_h_x1 = 0;
var scroll_h_x2 = 0;
var moving_h = false;
var moving_v = false;
var scroll_v_y1 = 0;
var scroll_v_y2 = 0;
var pos_x = 0;
var pos_y = 0;
var bar_length = 0;
var bar_height = 0;
var frame_width = 0;
var frame_height = 0;

$(document).ready(function () {
    draw(false, false);
});

function draw(scr_h, scr_v) {
    var width = $("#cnvs").width();
    var height = $("#cnvs").height();
    const canvas = document.getElementById("cnvs");
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext("2d");
    ctx.save();

    frame_width = width - 16;
    frame_height = height - 16;

    // 中心線描画
    ctx.strokeStyle = "#7777ff";
    ctx.lineWidth = 1;
    var center_y = (height - 16) / 2;
    var center_x = (width - 16) / 2;
    var h = height - 16;
    var w = width - 16;
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
    for (var i in tickets) {
        var ticket = tickets[i];
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

        var t1 = ticket.id_disp;
        var t2 = ticket.name;
        var m1 = ctx.measureText(t1).width;
        var m2 = ctx.measureText(t2).width;
        var m = m1 + m2 + 10;
        ctx.beginPath();
        var x = width / 2 + ticket.x - pos_x;
        var y = height / 2 - ticket.y - pos_y;
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
        var w1 = scroll_width / 2 + pos_x - frame_width / 2;
        var w2 = w1 + frame_width;
        var x1 = bar_length * w1 / scroll_width;
        var x2 = bar_length * w2 / scroll_width;
        scroll_h_x1 = x1 + 16;
        scroll_h_x2 = x2 + 16;
        ctx.fillStyle = scr_h ? "#a8a8a8" : "#c1c1c1";
        ctx.fillRect(scroll_h_x1, height - 14, x2 - x1, 13);
    }

    if (scroll_v) {
        bar_height = height - 48;
        var h1 = scroll_height / 2 + pos_y - frame_height / 2;
        var h2 = h1 + frame_height;
        var y1 = bar_height * h1 / scroll_height;
        var y2 = bar_height * h2 / scroll_height;
        scroll_v_y1 = y1 + 16;
        scroll_v_y2 = y2 + 16;
        ctx.fillStyle = scr_v ? "#a8a8a8" : "#c1c1c1";
        ctx.fillRect(width - 14, scroll_v_y1, 13, y2 - y1);
    }

    ctx.restore();
}

$('#cnvs').on('mousedown touchstart', function (e) {
    var $element = $(this);
    var x = e.clientX - $element.offset().left;
    var y = e.clientY - $element.offset().top;
    selected = null;

    if (y > ($element.height() - 16)) {
        // 横スクロールバー
        if (x > scroll_h_x1 && x < scroll_h_x2) {
            moving_h = true;
            start_x = x;
        }
    } else if (x > ($element.width() - 16)) {
        // 縦スクロールバー
        if (y > scroll_v_y1 && y < scroll_v_y2) {
            moving_v = true;
            start_y = y;
        }
    } else {
        // マップエリア
        for (var i in tickets) {
            var ticket = tickets[i];
            if (x > ticket.x1 && x < ticket.x2 && y > ticket.y1 && y < ticket.y2) {
                selected = ticket;
                tickets.splice(i, 1);
                tickets.push(selected);
                break;
            }
        }
        if (selected) {
            start_x = x;
            start_y = y;
        }
    }
});

$('#cnvs').on('mousemove touchmove', function (e) {
    var $element = $(this);
    var x = e.clientX - $element.offset().left;
    var y = e.clientY - $element.offset().top;
    var pointer = false;

    if (y > ($element.height() - 16)) {
        // 横スクロールバー
        selected = null;
        moving_v = false;
        $element.removeClass('cursor-pointer');
        if (moving_h) {
            var diff_x = x - start_x;
            scroll_h_x1 += diff_x;
            var x1 = scroll_h_x1 - 16;
            var w1 = x1 * scroll_width / bar_length;
            if (w1 < 0 || (w1 + frame_width) > scroll_width) return;
            pos_x = w1 + frame_width / 2 - scroll_width / 2;
            start_x = x;
        }
        draw(x > scroll_h_x1 && x < scroll_h_x2, false);
    } else if (x > ($element.width() - 16)) {
        // 縦スクロールバー
        selected = null;
        $element.removeClass('cursor-pointer');
        moving_h = false;
        if (moving_v) {
            var diff_y = y - start_y;
            scroll_v_y1 += diff_y;
            var y1 = scroll_v_y1 - 16;
            var h1 = y1 * scroll_height / bar_height;
            if (h1 < 0 || (h1 + frame_height) > scroll_height) return;
            pos_y = h1 + frame_height / 2 - scroll_height / 2;
            start_y = y;
        }
        draw(false, y > scroll_v_y1 && y < scroll_v_y2);
    } else {
        moving_h = false;
        moving_v = false;
        // マップエリア
        for (var i in tickets) {
            var ticket = tickets[i];
            if (x > ticket.x1 && x < ticket.x2 && y > ticket.y1 && y < ticket.y2) {
                pointer = true;
                break;
            }
        }
        if (pointer) {
            $element.addClass('cursor-pointer');
        } else {
            $element.removeClass('cursor-pointer');
        }
        if (selected) {
            var diff_x = x - start_x;
            var diff_y = y - start_y;
            selected.x += diff_x;
            selected.y -= diff_y;
            start_x = x;
            start_y = y;
        }
        draw(false, false);
    }
});

$('#cnvs').on('mouseup mouseleave touchend', function (e) {
    var $element = $(this);
    selected = null;
    moving_h = false;
    moving_v = false;
    $element.removeClass('cursor-pointer');
});
