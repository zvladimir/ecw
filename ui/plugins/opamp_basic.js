import { buildOpampSchematic } from "./opamp_schematic.js";

/** The two single-ended amplifiers differ in the topology sent to the backend and the drawing. */
const amplifier = (topology, fields) => ({
  category: "Analog",
  command: "opamp",
  request: { topology },
  storeKey: `ecw.opamp.${topology}`,
  schematic: buildOpampSchematic(topology),
  plots: [{ key: "plot" }],
  inputsHeading: false,
  ...fields,
});

export const opampInverting = amplifier("inverting", {
  id: "opamp_basic",
  title: "Op-Amp (inverting)",
  doc: "Inverting amplifier: G = -R2/R1, worst-case tolerances",
});

export const opampNonInverting = amplifier("non_inverting", {
  id: "opamp_non_inverting",
  title: "Op-Amp (non-inverting)",
  doc: "Non-inverting amplifier: G = 1 + R1/R2, worst-case tolerances",
});
