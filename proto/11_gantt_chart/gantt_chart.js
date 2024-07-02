let cols = [
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

const SCROLL_BAR_WIDTH = 16;

// 水平スクロールバー
class ScrollBar {
    x1 = 0;
    y1 = 0;
    x2 = 0;
    y2 = 0;

    generate(frag) {
        //console.log(`x1=${this.x1} y1=${this.y1} x2=${this.x2} y2=${this.y2}`);
        let bar = document.createElement('div');
        bar.className = "scroll-bar";
        bar.style = `top: ${this.y1}px; left: ${this.x1 + 1}px; width: ${this.x2 - this.x1}px; height: ${this.y2 - this.y1}px;`;
        frag.append(bar);
    }
}

class Frame {
    width = 0;
    height = 0;
    scrollBarH = new ScrollBar();
    scrollBarV = new ScrollBar();

    generate() {
        const chart = document.querySelector("#chart");
        const lineHeight = 26;
        if (!chart) {
            console.error("Failed to get #chart!");
            return;
        }
        this.width = chart.offsetWidth;
        this.height = chart.offsetHeight;
        //console.log(`width: ${chart.offsetWidth} height: ${chart.offsetHeight}`);
        let frag = document.createDocumentFragment();
        let header = document.createElement('div');
        header.className = "header";
        header.style = "top: 0px; left: 0px; width: 100%; height: 78px;"
        frag.append(header);

        let line = document.createElement('div');
        line.className = "line";
        line.style = "top: 78px; left: 0px; width: 100%; height: 1px;"
        frag.append(line);

        let x = 0;
        for (let col of cols) {
            let label = document.createElement('div');
            label.className = "label";
            label.style = `top: 55px; left: ${x}px; width: ${col.width}px;`;
            label.textContent = col.name;
            header.append(label);
            x += col.width;

            line = document.createElement('div');
            line.className = "line";
            line.style = `top: 0px; left: ${x}px; width: 1px; height: 100%;`;
            frag.append(line);
        }

        x += 2;
        line = document.createElement('div');
        line.className = "line";
        line.style = `top: 0px; left: ${x}px; width: 1px; height: 100%;`;
        frag.append(line);

        line = document.createElement('div');
        line.className = "line";
        line.style = `top: ${lineHeight}px; left: ${x}px; width: ${chart.offsetWidth - x}px; height: 1px;`;
        frag.append(line);

        line = document.createElement('div');
        line.className = "line";
        line.style = `top: ${lineHeight + lineHeight}px; left: ${x}px; width: ${chart.offsetWidth - x}px; height: 1px;`;
        frag.append(line);

        this.scrollBarH.x1 = x;
        this.scrollBarH.y1 = this.height - SCROLL_BAR_WIDTH;
        this.scrollBarH.x2 = this.width - SCROLL_BAR_WIDTH;
        this.scrollBarH.y2 = this.height;
        this.scrollBarH.generate(frag);

        this.scrollBarV.x1 = this.width - SCROLL_BAR_WIDTH;
        this.scrollBarV.y1 = lineHeight * 3 + 1;
        this.scrollBarV.x2 = this.width;
        this.scrollBarV.y2 = this.height - SCROLL_BAR_WIDTH;
        this.scrollBarV.generate(frag);

        let corner = document.createElement('div');
        corner.className = "scroll-corner";
        corner.style = `top: ${this.height - SCROLL_BAR_WIDTH}px; left: ${this.width - SCROLL_BAR_WIDTH}px; width: ${SCROLL_BAR_WIDTH}px; height: ${SCROLL_BAR_WIDTH}px;`;
        frag.append(corner);

        chart.append(frag);
    }
}

let f = new Frame();
f.generate();
