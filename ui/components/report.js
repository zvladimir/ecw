import { html } from "../lib/preact.js";
import { usePersisted } from "../lib/storage.js";
import { t } from "../lib/i18n.js";
import { useSolver } from "../lib/solver.js";
import { Field, ResultTable, Results, Section, Select } from "./form.js";
import { Plot } from "./plot.js";
import { SchematicDrawing, SchematicView } from "./schematic.js";

/**
 * Parts shared by the design pages (level shifters, difference amplifier):
 * every page asks the backend for a report with fields, steps, checks, notes and graphs.
 */

const TABS = [{ id: "schematic", name: "Schematic" }, { id: "plot", name: "Plot" }];
const MARKS = { ok: "✓", fail: "✗", info: "ℹ" };


export function Tabs({ tab, onChange }) {
  return html`
    <div class="tabs">${TABS.map((tab_) => html`
      <button key=${tab_.id} type="button" class=${`tab${tab === tab_.id ? " active" : ""}`}
        onClick=${() => onChange(tab_.id)}>${t(tab_.name)}</button>`)}
    </div>`;
}

/** Input fields of the report, in the labelled groups the backend assigned them to. */
export function GroupedFields({ fields = [], groups, values, onInput }) {
  const byGroup = {};
  for (const field of fields) (byGroup[field.group] ??= []).push(field);
  return Object.entries(byGroup).map(([group, list]) => html`
    <h3 class="group" key=${group}>${t(groups[group])}</h3>
    <div class="field-grid">
      ${list.map((f) => html`
        <${Field} key=${f.key} fieldKey=${f.key} label=${f.label} hint=${f.hint} info=${f}
          value=${values[f.key] ?? ""} onInput=${(v) => onInput({ [f.key]: v })} />`)}
    </div>`);
}

/** The list of checks (✓ ✗ ℹ) and the recommendations under it. */
export function ChecksAndNotes({ resp }) {
  return html`
    ${resp?.checks?.length > 0 && html`
      <h3 class="group">${t("Checks")}</h3>
      <ul class="checks">
        ${resp.checks.map((c, i) => html`
          <li key=${i} class=${`check ${c.status}`}><span class="mark">${MARKS[c.status]}</span>${t(c.text)}</li>`)}
      </ul>`}
    ${resp?.notes?.length > 0 && html`
      <h3 class="group">${t("Notes and recommendations")}</h3>
      <ul class="notes">${resp.notes.map((n, i) => html`<li key=${i}>${t(n)}</li>`)}</ul>`}`;
}

/** Graphs of the report: `plots` is [{ key, title }], `key` names the field of the reply. */
export function Plots({ resp, plots }) {
  return plots.map((p) => resp?.[p.key] && html`
    ${p.title && html`<h3 class="group">${t(p.title)}</h3>`}
    <${Plot} key=${p.key} plot=${resp[p.key]} />`);
}

/** What a solved form shows by default: the "no data" hint or the table, then checks and notes. */
export const defaultResults = (resp) => {
  const table = resp?.steps ?? resp?.table;
  return html`
    ${resp?.no_data
      ? html`<div class="msg-hint">${t(resp.no_data)}</div>`
      : table && html`<${ResultTable} ...${table} strongFirst />`}
    <${ChecksAndNotes} resp=${resp} />`;
};

/**
 * Mistakes in a calculator's wiring, for its developer: a slot of the schematic that the backend
 * sends no field for, or a field with neither a slot nor a heading for its group.
 */
function wiringProblems(sch, fields, groups) {
  const keys = new Set(fields.map((f) => f.key));
  const slots = new Set(sch.slots.map((slot) => slot.key));
  const problems = [
    ...[...slots].filter((key) => !keys.has(key)).map((key) => `the schematic has a slot "${key}" that the backend sends no field for`),
    ...fields.filter((f) => !slots.has(f.key) && !groups[f.group])
      .map((f) => `the field "${f.key}" has neither a slot on the schematic nor a heading for its group "${f.group}"`),
  ];
  return problems.length > 0 ? problems.join("; ") : null;
}

/**
 * A complete design page: schematic, selectors, grouped fields, results with the table,
 * checks and notes, and the graphs on the second tab. A calculator is described by these options:
 *
 * - `storeKey`, `command`: where the form is saved and which backend calculator answers;
 * - `request`: fixed values added to every request (for example the topology);
 * - `defaults`: starting values of fields (empty ones need no entry);
 * - `schematic`: a drawing, or a function of the form state that returns one;
 * - `selects`: [{ key, label, options, default, patch }] drop-downs whose values live in the saved
 *   state and go to the backend; `patch(value)` returns more values to set with the choice;
 * - `groups`: the backend's group names mapped to headings;
 * - `plots`: [{ key, title }] graphs of the second tab, `key` names the field of the reply;
 * - `inputsHeading: false` drops the "Design Inputs" heading under the schematic;
 * - `sideSelects`: the selects sit beside a smaller schematic instead of under a large one;
 * - `fixState(stored)`: repairs a stale saved state;
 * - `renderResults(resp)`: replaces the default results.
 *
 * Fields whose key is a slot of the schematic are edited on the drawing instead of in the groups.
 * A slot without a field, or a field with neither a slot nor a group heading, is shown as an error.
 */
export function DesignPage({
  storeKey, command, request = {}, defaults = {}, schematic, groups = {}, selects = [], plots = [],
  inputsHeading = true, sideSelects = false, fixState = (s) => s, renderResults = defaultResults,
}) {
  const start = { tab: "schematic", ...Object.fromEntries(selects.filter((s) => s.default !== undefined).map((s) => [s.key, s.default])), ...defaults };
  const [stored, update] = usePersisted(storeKey, start, selects.map((s) => s.key));
  const { tab, ...state } = fixState(stored);
  const resp = useSolver(command, { ...request, ...state });
  const sch = typeof schematic === "function" ? schematic(state) : schematic;

  const onSchematic = new Set(sch.slots.map((slot) => slot.key));
  const byKey = Object.fromEntries((resp?.fields ?? []).map((f) => [f.key, f]));
  const renderSlot = (key) => html`
    <${Field} fieldKey=${key} label=${byKey[key]?.label ?? key} hint=${byKey[key]?.hint ?? ""} feedback=${false} showComputed=${false}
      hideLabel=${sch.slots.find((slot) => slot.key === key)?.hideLabel}
      info=${byKey[key]} value=${state[key] ?? ""} onInput=${(v) => update({ [key]: v })} />`;

  const wiring = resp?.fields && wiringProblems(sch, resp.fields, groups);
  const tabs = html`
    ${wiring && html`<div class="msg-err">Calculator wiring: ${wiring}</div>`}
    <${Tabs} tab=${tab} onChange=${(id) => update({ tab: id })} />`;
  if (tab === "plot") {
    // A backend that always sends the plot (empty axes at first) shows it whatever the outcome.
    const always = plots.some((p) => resp?.[p.key]);
    return html`${tabs}${always ? html`<${Plots} resp=${resp} plots=${plots} />` : html`<${Results} resp=${resp}><${Plots} resp=${resp} plots=${plots} /><//>`}`;
  }

  const selectors = selects.map((s) => html`
    <${Select} key=${s.key} label=${s.label} options=${s.options} value=${state[s.key]}
      onChange=${(v) => update({ [s.key]: v, ...s.patch?.(v) })} />`);
  const heading = inputsHeading && html`<${Section} title="Design Inputs" />`;
  const fields = html`
    <${GroupedFields} fields=${resp?.fields?.filter((f) => !onSchematic.has(f.key))} groups=${groups} values=${state} onInput=${update} />`;
  const results = html`<${Section} title="Results" /><${Results} resp=${resp}>${renderResults(resp)}<//>`;

  if (sideSelects) {
    return html`
      ${tabs}${heading}
      <div class="split">
        <div class="form-col wide">${selectors}</div>
        <div class="side-sch wide"><${SchematicDrawing} sch=${sch} /></div>
      </div>
      ${fields}${results}`;
  }
  return html`
    ${tabs}
    ${onSchematic.size > 0
      ? html`<${SchematicView} sch=${sch} renderSlot=${renderSlot} />`
      : html`<${SchematicDrawing} sch=${sch} />`}
    ${heading}
    ${selects.length > 0 && html`<div class="fields wrap">${selectors}</div>`}
    ${fields}${results}`;
}
