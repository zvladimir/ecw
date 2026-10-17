import { html } from "../lib/preact.js";
import { FormPage } from "../components/form_page.js";
import { DesignPage } from "../components/report.js";
import currentShape from "./current_shape.js";
import levelShifter from "./level_shifter.js";
import levelShifterNonInverting from "./level_shifter_ni.js";
import ohmsLaw from "./ohms_law.js";
import { opampInverting, opampNonInverting } from "./opamp_basic.js";
import opampDifferential from "./opamp_differential.js";
import resistorSolver from "./resistor_solver.js";
import voltageDivider from "./voltage_divider.js";
import wireAwg from "./wire_awg.js";

/**
 * A calculator is a manifest: `id`, `title`, `category`, `doc` and either a `view` component or the
 * options of a page: `DesignPage` (components/report.js) or, with `page: "form"`, `FormPage`
 * (components/form_page.js).
 */
const MANIFESTS = [
  ohmsLaw,
  wireAwg,
  voltageDivider,
  resistorSolver,
  currentShape,
  opampInverting,
  opampNonInverting,
  levelShifter,
  levelShifterNonInverting,
  opampDifferential,
];

const PAGES = { design: DesignPage, form: FormPage };

/** Calculator registry: the sidebar, header and routing are built from it. */
export const PLUGINS = MANIFESTS.map((m) => ({
  ...m,
  view: m.view ?? (() => html`<${PAGES[m.page ?? "design"]} ...${m} />`),
}));

export const CATEGORIES = [...new Set(PLUGINS.map((p) => p.category))].sort();
