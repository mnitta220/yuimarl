import "./style.css";
import GanttFrame from "./ganttFrame";

const frame = new GanttFrame();
frame.draw();

window.addEventListener("resize", function () {
  frame.resize();
  frame.draw();
});
