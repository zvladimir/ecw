// Manifest of the voltage divider: a form page. The selector picks two or three resistors, and
// the schematic beside the fields follows the choice. The calculation is the backend's
// (`voltage_divider`).
import { html } from "../lib/preact.js";
import { SchematicDrawing } from "../components/schematic.js";
import { buildDividerSchematic } from "./divider_schematic.js";

export default {
  id: "voltage_divider",
  title: "Voltage Divider",
  category: "Basic",
  doc: "Vout = Vin · R_bottom / ΣR",
  page: "form",
  command: "voltage_divider",
  storeKey: "ecw.divider",
  selects: [{
    key: "mode", label: "Configuration", default: "two",
    options: [{ id: "two", name: "2 resistors" }, { id: "three", name: "3 resistors" }],
  }],
  side: ({ state }) => html`
    <div class="side-sch"><${SchematicDrawing} sch=${buildDividerSchematic(state.mode)} /></div>`,
};
