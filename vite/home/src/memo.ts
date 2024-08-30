import MemoModal from "./memoModal";

export default class Memo {
  memoModal = new MemoModal();

  constructor() {
    this.handler();
  }

  // イベントハンドラを登録する
  private handler() {
    this.memoModal.handler();
  }
}
