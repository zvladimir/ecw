import { html } from "../lib/preact.js";
import { t } from "../lib/i18n.js";
import { usePersisted } from "../lib/storage.js";
import { useSolver } from "../lib/solver.js";
import { Field, ResultTable, Results, Section } from "../components/form.js";

// The last column, `n`, is an input: the number of parallel wires of the row.
const HEADERS = ["AWG", "D", "A", "R", "Vdrop", "P", "ΔT", "n"];
const FIELDS = [
  { key: "current", label: "I [A]", hint: "0.5, 500m" },
  { key: "length", label: "L [m]", hint: "1, 100m" },
  { key: "rth", label: "Rth [K/W]", hint: "10, 1.5" },
];

/** AWG Table: the current, the length and the thermal resistance in; a row for every wire size
 *  out. The count of parallel wires is kept per row and sent with the request. */
function WireAwg() {
  const [state, update] = usePersisted("ecw.awg", { current: "1", length: "1", rth: "50", n: [] });
  const resp = useSolver("wire_awg", state);

  // The count of row `i`, never below the least the backend allows.
  const nOf = (i) => Math.max(resp.min_n, state.n[i] ?? resp.default_n);
  const setN = (i, raw) => {
    const v = Math.round(Number(raw));
    const clamped = Number.isFinite(v) ? Math.min(resp.max_n, Math.max(resp.min_n, v)) : resp.default_n;
    update(() => {
      const n = Array.from({ length: resp.rows_total }, (_, k) => nOf(k));
      n[i] = clamped;
      return { n };
    });
  };

  const rows = resp?.rows ?? [];
  const cell = (c, v, r) => c === 7 ? html`
    <input type="number" min=${resp.min_n} max=${resp.max_n} step="1" value=${nOf(r)}
      onChange=${(e) => { setN(r, e.currentTarget.value); e.currentTarget.value = nOf(r); }} />` : v;

  return html`
    <${Section} title="Design Inputs" />
    <div class="fields">
      ${FIELDS.map((f) => html`
        <${Field} key=${f.key} label=${f.label} hint=${f.hint} info=${resp?.[f.key]}
          value=${state[f.key]} onInput=${(v) => update({ [f.key]: v })} />`)}
    </div>
    <${Section} title="Results" />
    <${Results} resp=${resp}>
      <${ResultTable} header=${HEADERS} strongFirst=${false} hoverRows
        rows=${rows.map((r) => [r.awg, r.d, r.a, r.r, r.vdrop, r.p, r.dt, ""])}
        copyRows=${rows.map((r, i) => [r.awg, r.d, r.a, r.r, r.vdrop, r.p, r.dt, String(nOf(i))])}
        renderCell=${cell} />
      <div class="footnote">${t(resp?.note)}</div>
    <//>`;
}

export default {
  id: "wire_awg",
  title: "AWG Table",
  category: "Basic",
  doc: "AWG wire table with drop, power, and ΔT",
  view: WireAwg,
};
