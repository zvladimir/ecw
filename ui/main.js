import { html, render } from "./lib/preact.js";
import { App } from "./app.js";

// The interface is not a document: no text selection and no browser menu, except in the inputs
// and in the results, which can be selected and copied (keep the list in step with style.css).
const SELECTABLE = "input, textarea, .tbl-wrap, .checks, .notes, .msg-err, .msg-hint, .msg-warn, .footnote, .computed, .fb";
// `selectstart` is aimed at a text node, which has no `closest`.
const element = (e) => (e.target.nodeType === Node.TEXT_NODE ? e.target.parentElement : e.target);
const selectable = (e) => element(e)?.closest(SELECTABLE);
document.addEventListener("contextmenu", (e) => {
  if (!selectable(e) || !(element(e).closest("input, textarea") || window.getSelection().toString())) e.preventDefault();
});
document.addEventListener("selectstart", (e) => {
  if (!selectable(e)) e.preventDefault();
});

// The scale is a setting of the app (Settings → Zoom), not something the webview changes on its own.
document.addEventListener("wheel", (e) => e.ctrlKey && e.preventDefault(), { passive: false });
document.addEventListener("keydown", (e) => {
  if ((e.ctrlKey || e.metaKey) && ["+", "=", "-", "_", "0"].includes(e.key)) e.preventDefault();
});
for (const type of ["gesturestart", "gesturechange"]) {
  document.addEventListener(type, (e) => e.preventDefault());
}

render(html`<${App} />`, document.getElementById("root"));
