import * as bootstrap from "bootstrap";

export default class DeleteCommentModal {
  private id = "deleteCommentModal";
  private modal: bootstrap.Modal | null = null;

  constructor() {
    const memberModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (memberModal) {
      this.modal = new bootstrap.Modal(memberModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const btnDeleteComment =
      document.querySelector<HTMLButtonElement>(`#btnDeleteComment`);
    if (btnDeleteComment) {
      btnDeleteComment.addEventListener("click", () => {
        this.deleteComment();
      });
    }

    const btnDelete = document.querySelector<HTMLButtonElement>(`#btnDelete`);
    if (btnDelete) {
      btnDelete.addEventListener("click", () => {
        this.delete();
      });
    }
  }

  show() {
    if (this.modal) {
      this.modal.show();
    }
  }

  hide() {
    if (this.modal) {
      this.modal.hide();
    }
  }

  private deleteComment() {
    const selectedIndex =
      document.querySelector<HTMLInputElement>(`#selectedIndex`);
    if (selectedIndex) {
      const idx = Number(selectedIndex.value);
      const cid = document.querySelector<HTMLInputElement>(`#cid${idx}`);
      if (cid) {
        const commentId =
          document.querySelector<HTMLInputElement>(`#comment_id`);
        if (commentId) {
          commentId.value = cid.value;
        }

        this.show();
      }
    }
  }

  delete() {
    const action = document.querySelector<HTMLInputElement>(`#action`);
    if (action) {
      action.value = "Delete";
      const form = document.querySelector<HTMLFormElement>(`#post_comment`);
      if (form) {
        form.submit();
      }
    }
  }
}
