import { marked } from "marked";
import CommentModal from "./commentModal";
import DeleteCommentModal from "./deleteCommentModal";

export default class Comment {
  commentModal = new CommentModal();
  deleteCommentModal = new DeleteCommentModal();

  constructor() {
    this.handler();

    for (let i = 0; i < 1000; i++) {
      const mkd = document.querySelector<HTMLInputElement>(`#mkd${i}`);
      if (mkd) {
        let html = marked.parse(mkd.value);
        const pre = document.querySelector<HTMLElement>(`#pre${i}`);
        if (pre) pre.innerHTML = `${html}`;
      } else {
        break;
      }
    }
  }

  // イベントハンドラを登録する
  private handler() {
    this.commentModal.handler();
    this.deleteCommentModal.handler();
  }
}
