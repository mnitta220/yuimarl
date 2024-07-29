import TicketInfo from "./ticketInfo";

const info = new TicketInfo();

const main = async () => {
  info.load();
};

main().catch((e) => {
  console.error(e);
  window.alert(`エラーが発生しました。: ${e}`);
});
