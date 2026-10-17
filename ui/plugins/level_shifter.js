// Manifest of the inverting level shifter: a design page whose fields sit on the schematic. The
// page is laid out by components/report.js, the numbers come from the backend (`level_shifter`).
import { buildLevelShifterSchematic } from "./level_shifter_schematic.js";

export default {
  id: "level_shifter",
  title: "Level Shifter (inverting)",
  category: "Analog",
  doc: "Level shifter on an inverting amplifier: Vo = −Vi·R1/R2 − Vref·R1/R3",
  command: "level_shifter",
  storeKey: "ecw.levelshifter.v2", // the fields changed: the old saved form does not fit
  schematic: buildLevelShifterSchematic(),
  plots: [{ key: "plot" }], // no heading, as for the op-amps
  inputsHeading: false,
};
