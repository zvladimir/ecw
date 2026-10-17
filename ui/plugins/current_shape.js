import { Plot } from "../components/plot.js";
import { html } from "../lib/preact.js";

/** Rows of the form for waveforms whose fields read better side by side (the rest: one per row). */
const ROWS = {
  half_sine: [["ip"], ["ton", "period"]],
  rectangular: [["ip"], ["ton", "period"]],
  trapezoidal: [["ip", "ip2"], ["ton", "period"]],
  triangular: [["ip"], ["t1", "t2"], ["t3", "period"]],
  triangular_offset: [["imin", "ip"], ["t1", "t2"], ["t3", "period"]],
  bipolar_triangular: [["imin", "ip"], ["t1", "t2"], ["t3", "period"]],
};

export default {
  id: "current_shape",
  title: "Current Shape",
  category: "Basic",
  doc: "RMS / average current for a periodic pulse",
  page: "form",
  command: "current_shape",
  storeKey: "ecw.shape",
  splitClass: "split wide",
  formClass: "form-col",
  selects: [{ key: "kind", label: "Waveform", default: "half_sine", options: (resp) => resp?.kinds ?? [] }],
  rows: (state) => ROWS[state.kind],
  side: ({ resp }) => resp && html`<${Plot} plot=${resp.plot} />`,
};
