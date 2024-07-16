import "./style.css";
import GanttFrame from "./ganttFrame";

const frame = new GanttFrame();

const main = async () => {
  await frame.readTreeList();
  frame.draw();
};

main().catch((e) => {
  console.error(e);
});

window.addEventListener("resize", function () {
  frame.resize();
  frame.draw();
});
