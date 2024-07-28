import ProjectInfo from "./projectInfo";

const project = new ProjectInfo();

const main = async () => {
  console.log("main");
  project.load();
  for (const m of project.members) {
    console.log(m);
  }
};

main().catch((e) => {
  console.error(e);
  window.alert(`エラーが発生しました。: ${e}`);
});
