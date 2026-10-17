import { html } from "../lib/preact.js";
import { t } from "../lib/i18n.js";
import { usePersisted } from "../lib/storage.js";
import { useSolver } from "../lib/solver.js";
import { Field, ResultTable, Results, Section, Select, useFieldId } from "../components/form.js";

/** Resistor Solver: the target, the E-series and the user's own resistors in; the table of the
 *  closest single resistors and pairs out. The search is the backend's (`resistor_solver`). */
function ResistorSolver() {
  const poolId = useFieldId();
  const [state, update] = usePersisted("ecw.resistor", {
    target: "10.12k", pool: "10k, 2k, 100", series: "E24",
  }, ["series"]);
  const resp = useSolver("resistor_solver", state);

  const series = (resp?.series_options ?? [state.series]).map((n) => ({ id: n, name: n }));
  const cells = resp?.rows ?? [];
  const flat = cells.map((r) => [
    t(r.label), ...r.cells.map((c) => (c ? `${c.text} (${c.deviation})` : "—")),
  ]);
  const renderCell = (c, _value, r) => {
    if (c === 0) return null;
    const cell = cells[r].cells[c - 1];
    if (!cell) return "—";
    return html`
      <div class="mono">${cell.text}</div>
      <div class=${`dev ${cell.in_tolerance ? "ok" : "warn"}`}>${cell.deviation}</div>`;
  };

  return html`
    <${Section} title="Design Inputs" />
    <div class="fields">
      <${Field} label="R [Ω]" hint=${resp?.target_hint ?? ""} info=${resp?.target}
        value=${state.target} onInput=${(target) => update({ target })} />
      <${Select} label="Series" options=${series} value=${state.series}
        onChange=${(s) => update({ series: s })} />
    </div>
    <div class="field pool">
      <label class="lbl" for=${poolId}>${t("Custom pool (Ω)")}</label>
      <div class="box">
        <textarea id=${poolId} rows="6" spellcheck=${false} value=${state.pool}
          onInput=${(e) => update({ pool: e.currentTarget.value })}></textarea>
      </div>
    </div>
    <${Section} title="Results" />
    <${Results} resp=${resp}>
      <${ResultTable} header=${resp?.header} rows=${flat} strongFirst renderCell=${renderCell} />
    <//>`;
}

export default {
  id: "resistor_solver",
  title: "Resistor Solver",
  category: "Basic",
  doc: "Pick R from E6…E96 or a custom pool",
  view: ResistorSolver,
};
