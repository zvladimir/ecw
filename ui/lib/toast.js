import { html, useEffect, useState } from "./preact.js";

/** The function that shows a message; `Toasts` replaces it when it is mounted. */
let show = () => {};

/** Shows a short message at the bottom of the window, from anywhere. */
export const toast = (message) => show(message);

/** The place of the messages: mount it once, in the app. */
export function Toasts() {
  const [message, setMessage] = useState("");
  useEffect(() => {
    let timer;
    show = (m) => {
      setMessage(m);
      clearTimeout(timer);
      timer = setTimeout(() => setMessage(""), 1600);
    };
    return () => clearTimeout(timer);
  }, []);
  return html`<div id="toast" class=${message ? "show" : ""} role="status">${message}</div>`;
}
