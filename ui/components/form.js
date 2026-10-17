import { html, useMemo } from "../lib/preact.js";
import { t } from "../lib/i18n.js";
import { copyTable } from "../lib/clipboard.js";

export const Section = ({ title }) => html`<h2 class="section">${t(title)}</h2>`;

let nextId = 0;
/** A document-unique id, stable across renders, to tie a label to its control. */
export const useFieldId = () => useMemo(() => `fld-${++nextId}`, []);

const NO_INFO = { status: "empty", text: "", computed: null };

/**
 * Text input with a clear button and parse feedback.
 * `info` is the backend's verdict for this field; `computed` there turns it into a read-only value.
 * Without `feedback` (compact fields on a schematic) only the error border and tooltip remain.
 * With `showComputed` off a computed field is just empty and locked: the value is shown in the results.
 */
export function Field({ label, hint, value, onInput, info = NO_INFO, feedback = true, fieldKey, hideLabel = false, showComputed = true }) {
  const isComputed = info.computed !== null && info.computed !== undefined;
  const isError = info.status === "error" && !isComputed;

  const id = useFieldId();
  let note = t(hint), noteClass = "fb hint";
  if (isComputed && info.computed) [note, noteClass] = [`→ ${t(info.computed)}`, "fb"];
  else if (isError) [note, noteClass] = [t(info.text), "fb err"];
  else if (info.status === "ok") [note, noteClass] = [t(info.text), "fb"];

  return html`
    <div class=${`field${isError ? " err" : ""}`} data-field=${fieldKey} title=${!feedback && isError ? t(info.text) : null}>
      <label class=${hideLabel ? "lbl sr-only" : "lbl"} for=${isComputed ? null : id}>${t(label)}</label>
      <div class="box">
        ${isComputed
          ? html`<span class="computed">${showComputed ? t(info.computed) : ""}</span>`
          : html`
            <input id=${id} type="text" spellcheck=${false} autocomplete="off" placeholder=${t(hint)}
              aria-invalid=${isError ? "true" : null} aria-describedby=${feedback ? `${id}-fb` : null}
              value=${value} onInput=${(e) => onInput(e.currentTarget.value)} />
            <button class="clear" type="button" title=${t("Clear")} aria-label=${t("Clear")}
              disabled=${value === ""} onClick=${() => onInput("")}>×</button>`}
      </div>
      ${feedback && html`<div id=${`${id}-fb`} class=${noteClass}>${note}</div>`}
    </div>`;
}

/**
 * Moves the focus to the input of the field `key` (a `Field` with `fieldKey`) and flashes the
 * field, which also shows a read-only (computed) one.
 */
export function focusField(key) {
  const field = document.querySelector(`.field[data-field="${key}"]`);
  if (!field) return;
  field.querySelector("input")?.select();
  field.classList.remove("flash");
  void field.offsetWidth; // restart the animation
  field.classList.add("flash");
  setTimeout(() => field.classList.remove("flash"), 1200);
}

/** Labelled drop-down; `options` is [{id, name}]. The current value is always listed. */
export function Select({ label, options, value, onChange }) {
  const list = options.some((o) => o.id === value) ? options : [{ id: value, name: value }, ...options];
  const id = useFieldId();
  return html`
    <div class="field">
      <label class="lbl" for=${id}>${t(label)}</label>
      <div class="box">
        <select id=${id} value=${value} onChange=${(e) => onChange(e.currentTarget.value)}>
          ${list.map((o) => html`<option key=${o.id} value=${o.id}>${t(o.name)}</option>`)}
        </select>
      </div>
    </div>`;
}

/**
 * Shows the backend outcome: the message for anything but a solved form, otherwise `children`.
 * Renders nothing before the first reply.
 */
export function Results({ resp, children }) {
  if (!resp) return null;
  const { kind, text } = resp.outcome;
  if (kind === "solved") return children;
  const cls = { error: "msg-err", fix_errors: "msg-err", pending: "msg-hint", too_many: "msg-warn" }[kind];
  return text ? html`<div class=${cls}>${t(text)}</div>` : null;
}

/** Table with an optional "Copy table" button. `renderCell(col, value, rowIndex)` may return custom content. */
export function ResultTable({ header = [], rows, strongFirst = false, renderCell, copyRows = rows, copy = true, hoverRows = false }) {
  return html`
    <div class="tbl-wrap">
      <table class=${`tbl${hoverRows ? " rows-hover" : ""}`}>
        ${header.length > 0 && html`<thead><tr>${header.map((h) => html`<th>${t(h)}</th>`)}</tr></thead>`}
        <tbody>
          ${rows.map((row, r) => html`
            <tr>${row.map((value, c) => html`
              <td class=${strongFirst && c === 0 ? "strong" : ""}>${renderCell?.(c, value, r) ?? t(value)}</td>`)}
            </tr>`)}
        </tbody>
      </table>
      ${copy && html`<button class="copy-btn" type="button"
        onClick=${() => copyTable(header.map((h) => t(h)), copyRows.map((row) => row.map((v) => t(v))))}>${t("Copy table")}</button>`}
    </div>`;
}
