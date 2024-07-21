import "./style.css";
import GanttFrame from "./ganttFrame";

const frame = new GanttFrame();

const main = async () => {
  await frame.readProject();
  await frame.readTreeList();
  frame.resetLines();
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

// 「完了済みを表示」が変更された
const showdone = document.querySelector<HTMLInputElement>(`#showdone`);
if (showdone) {
  showdone.addEventListener("change", function () {
    frame.setShowDone(showdone.checked);
  });
}

// 「進捗遅れを赤く表示」が変更された
const delayred = document.querySelector<HTMLInputElement>(`#delayred`);
if (delayred) {
  delayred.addEventListener("change", function () {
    frame.setDelayRed(delayred.checked);
  });
}
