// Manifest of the difference amplifier: a design page whose fields sit on the schematic, with the
// signal ranges and the resistors in two groups. The numbers come from the backend
// (`opamp_differential`).
import { buildDifferentialSchematic } from "./opamp_differential_schematic.js";

export default {
  id: "opamp_differential",
  title: "Op-Amp (differential)",
  category: "Analog",
  doc: "Difference amplifier: Vo = G·(Vi2 - Vi1) + Vref",
  command: "opamp_differential",
  storeKey: "ecw.opamp.differential",
  schematic: buildDifferentialSchematic(),
  groups: { signal: "Signal ranges", circuit: "Resistors and reference" },
  plots: [{ key: "plot" }], // no heading, as for the single-ended amplifiers
  inputsHeading: false,
};
