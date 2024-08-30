import { marked } from "marked";
import Memo from "./memo";

document.addEventListener("DOMContentLoaded", () => {
  new Memo();

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
      const preview2 = document.querySelector<HTMLElement>(`#preview2`);
      if (html && preview2) preview2.innerHTML = `${html}`;
    });
  }
});
