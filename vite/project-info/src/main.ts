import ProjectInfo from "./projectInfo";

const project = new ProjectInfo();

const main = async () => {
  project.load();
};

main().catch((e) => {
  console.error(e);
  window.alert(`エラーが発生しました。: ${e}`);
});
