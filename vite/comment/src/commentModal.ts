import { marked } from "marked";
import * as bootstrap from "bootstrap";

export default class CommentModal {
  private id = "commentModal";
  private modal: bootstrap.Modal | null = null;

  constructor(/*private info: Comment*/) {
    const memberModal = document.querySelector<HTMLDivElement>(`#${this.id}`);
    if (memberModal) {
      this.modal = new bootstrap.Modal(memberModal);
    }
  }

  // イベントハンドラを登録する
  handler() {
    const icnAddComment =
      document.querySelector<HTMLButtonElement>(`#icnAddComment`);
    if (icnAddComment) {
      icnAddComment.addEventListener("click", () => {
        this.clickComment();
      });
    }

    const btnAdd = document.querySelector<HTMLButtonElement>(`#btnAdd`);
    if (btnAdd) {
      btnAdd.addEventListener("click", () => {
        this.add();
      });
    }

    const btnEditComment =
      document.querySelector<HTMLButtonElement>(`#btnEditComment`);
    if (btnEditComment) {
      btnEditComment.addEventListener("click", () => {
        this.editComment();
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

  add() {
    const action = document.querySelector<HTMLInputElement>(`#action`);
    if (action) {
      action.value = "Create";
      const markdown = document.querySelector<HTMLInputElement>(`#markdown`);
      if (markdown) {
        const comment = document.querySelector<HTMLInputElement>(`#comment`);
        if (comment) {
          comment.value = markdown.value;
          const form = document.querySelector<HTMLFormElement>(`#post_comment`);
          if (form) {
            form.submit();
          }
        }
      }
    }
  }

  clickComment() {
    const commentId = document.querySelector<HTMLInputElement>(`#comment_id`);
    if (commentId) {
      commentId.value = "";
    }
    const commentModalLabel =
      document.querySelector<HTMLElement>(`#commentModalLabel`);
    if (commentModalLabel) {
      commentModalLabel.innerHTML = "コメントを追加";
    }
    const btnUpdate = document.querySelector<HTMLElement>(`#btnUpdate`);
    if (btnUpdate) {
      btnUpdate.classList.add("d-none");
    }

    const btnAdd = document.querySelector<HTMLElement>(`#btnAdd`);
    if (btnAdd) {
      btnAdd.classList.remove("d-none");
    }
    const markdown = document.querySelector<HTMLInputElement>(`#markdown`);
    if (markdown) {
      markdown.value = "";
    }
    const preview1 = document.querySelector<HTMLElement>(`#preview1`);
    if (preview1) {
      preview1.innerHTML = "";
    }

    this.show();
  }

  private editComment() {
    const selectedIndex =
      document.querySelector<HTMLInputElement>(`#selectedIndex`);
    if (selectedIndex) {
      const idx = Number(selectedIndex.value);
      const commentModalLabel =
        document.querySelector<HTMLElement>(`#commentModalLabel`);
      if (commentModalLabel) {
        commentModalLabel.innerHTML = "コメントを編集";
      }
      const btnAdd = document.querySelector<HTMLElement>(`#btnAdd`);
      if (btnAdd) {
        btnAdd.classList.add("d-none");
      }
      const btnUpdate = document.querySelector<HTMLElement>(`#btnUpdate`);
      if (btnUpdate) {
        btnUpdate.classList.remove("d-none");
      }
      const cid = document.querySelector<HTMLInputElement>(`#cid${idx}`);
      const mid = document.querySelector<HTMLInputElement>(`#mkd${idx}`);
      if (cid && mid) {
        const commentId =
          document.querySelector<HTMLInputElement>(`#comment_id`);
        if (commentId) {
          commentId.value = cid.value;
        }
        const markdown = document.querySelector<HTMLInputElement>(`#markdown`);
        if (markdown) {
          markdown.value = mid.value;
        }
        let html = marked.parse(mid.value);
        const preview1 = document.querySelector<HTMLElement>(`#preview1`);
        if (preview1) {
          preview1.innerHTML = `${html}`;
        }

        this.show();
      }
    }
  }
}
