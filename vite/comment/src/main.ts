import { marked } from "marked";
import Comment from "./comment";

document.addEventListener("DOMContentLoaded", () => {
  new Comment();

  const markdown = document.querySelector<HTMLInputElement>(`#markdown`);
  if (markdown && markdown.value) {
    let html = marked.parse(markdown.value);
    const preview1 = document.querySelector<HTMLElement>(`#preview1`);
    if (html && preview1) preview1.innerHTML = `${html}`;
    const preview2 = document.querySelector<HTMLElement>(`#preview2`);
    if (html && preview2) preview2.innerHTML = `${html}`;
  }

  if (markdown) {
    markdown.addEventListener("keyup", () => {
      let html = marked.parse(markdown.value);
      const preview1 = document.querySelector<HTMLElement>(`#preview1`);
      if (html && preview1) preview1.innerHTML = `${html}`;
      const preview2 = document.querySelector<HTMLElement>(`#preview2`);
      if (html && preview2) preview2.innerHTML = `${html}`;
    });
  }

  const edit = document.querySelector<HTMLInputElement>(`#edit`);
  if (edit) {
    edit.addEventListener("change", () => {
      if (edit.checked) {
        const note1 = document.querySelector<HTMLElement>(`#note1`);
        if (note1) note1.classList.remove("d-none");
        const note2 = document.querySelector<HTMLElement>(`#note2`);
        if (note2) note2.classList.add("d-none");
      } else {
        const note2 = document.querySelector<HTMLElement>(`#note2`);
        if (note2) note2.classList.remove("d-none");
        const note1 = document.querySelector<HTMLElement>(`#note1`);
        if (note1) note1.classList.add("d-none");
      }
    });
  }
});
