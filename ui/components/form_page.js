import { html, useRef } from "../lib/preact.js";
import { usePersisted } from "../lib/storage.js";
import { useSolver } from "../lib/solver.js";
import { Field, Results, Section, Select } from "./form.js";
import { defaultResults } from "./report.js";

/**
 * A calculator whose inputs are a plain list of fields (with selects above them), results below
 * and, optionally, something beside the form. Options of a manifest:
 *
 * - `storeKey`, `command`, `defaults`: as for `DesignPage`;
 * - `selects`: [{ key, label, options, default }]; `options` is a list or a function of the reply;
 * - `rows(state)`: field keys grouped into rows, for a form that reads better side by side
 *   (fields it does not name get no place); without it every field has a row of its own;
 * - `side({ state, resp, formRef })`: what stands beside the form;
 * - `splitClass`, `formClass`: the layout classes of the form and the area around it.
 */
export function FormPage({
  storeKey, command, defaults = {}, selects = [], rows, side,
  splitClass = "split", formClass = "form-col grow",
}) {
  const start = { ...Object.fromEntries(selects.filter((s) => s.default !== undefined).map((s) => [s.key, s.default])), ...defaults };
  const [state, update] = usePersisted(storeKey, start, selects.map((s) => s.key));
  const resp = useSolver(command, state);
  const formRef = useRef(null);

  const fields = resp?.fields ?? [];
  // Before the first reply there are no fields yet, so the empty rows are dropped.
  const grid = (rows?.(state)?.map((keys) => keys.map((k) => fields.find((f) => f.key === k)).filter(Boolean))
    ?? fields.map((f) => [f])).filter((row) => row.length > 0);
  const field = (f) => html`
    <${Field} key=${f.key} fieldKey=${f.key} label=${f.label} hint=${f.hint} info=${f}
      value=${state[f.key] ?? ""} onInput=${(v) => update({ [f.key]: v })} />`;

  return html`
    <${Section} title="Design Inputs" />
    <div class=${splitClass}>
      <div class=${formClass} ref=${formRef}>
        ${selects.map((s) => html`
          <${Select} key=${s.key} label=${s.label} options=${typeof s.options === "function" ? s.options(resp) : s.options}
            value=${state[s.key]} onChange=${(v) => update({ [s.key]: v })} />`)}
        <div class="field-list">
          ${grid.map((row) => row.length > 1
            ? html`<div class="field-row" key=${row.map((f) => f.key).join()}>${row.map(field)}</div>`
            : field(row[0]))}
        </div>
      </div>
      ${side?.({ state, resp, formRef })}
    </div>
    <${Section} title="Results" />
    <${Results} resp=${resp}>${defaultResults(resp)}<//>`;
}
