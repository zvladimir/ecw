// Manifest of the non-inverting level shifter: a design page whose fields sit on the schematic.
// The page is laid out by components/report.js, the numbers come from the backend
// (`level_shifter_ni`).
import { buildLevelShifterNiSchematic } from "./level_shifter_ni_schematic.js";

export default {
  id: "level_shifter_non_inverting",
  title: "Level Shifter (non-inverting)",
  category: "Analog",
  doc: "Level shifter on a non-inverting amplifier: Vo = Vi·R4/(R3+R4)·(1 + R1/R2) − Vref·R1/R2",
  command: "level_shifter_ni",
  storeKey: "ecw.levelshifter.ni.v2", // the fields changed: the old saved form does not fit
  schematic: buildLevelShifterNiSchematic(),
  plots: [{ key: "plot" }], // no heading, as for the op-amps
  inputsHeading: false,
};
