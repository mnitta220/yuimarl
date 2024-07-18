import "./style.css";
import GanttFrame from "./ganttFrame";

const frame = new GanttFrame();

const main = async () => {
  await frame.readProject();
  await frame.readTreeList();
  frame.draw();
};

main().catch((e) => {
  console.error(e);
  window.alert(`エラーが発生しました。: ${e}`);
});

window.addEventListener("resize", function () {
  frame.resize();
  frame.draw();
});

const showdone = document.querySelector<HTMLInputElement>(`#showdone`);
if (showdone) {
  showdone.addEventListener("change", function () {
    frame.setShowDone(showdone.checked);
  });
}
