import { HEIGHT, TOP_EXTRA } from "./opamp_schematic.js";
import { RESISTOR_H, Schematic } from "../components/schematic.js";
import { amplifierParts, resistorAbove, source } from "./level_shifter_schematic.js";

const FIELD_W = 140;
const FIELD_H = 54;
const FIELD_GAP = 6;

/**
 * Non-inverting level shifter: Vref through R2 to the inverting input with R1 as the feedback,
 * Vin through the divider R3/R4 to the non-inverting input. The input fields sit on the circuit.
 */
export function buildLevelShifterNiSchematic() {
  const sch = new Schematic(880, HEIGHT, TOP_EXTRA);
  const geo = {
    left: 400, tip: 620, top: 110, bottom: 330, mid: 220,
    invY: 170, nonInvY: 270, nodeX: 340, fbY: 50, outX: 700,
  };
  const { left, invY, nonInvY } = geo;

  // Vref - R2 - inverting input.
  source(sch, "vref", invY, "Vref");
  sch.line([[35, invY], [130, invY]]);
  resistorAbove(sch, "r2", 130, invY);
  sch.line([[220, invY], [left, invY]]);

  // Vin - R3 - non-inverting input, R4 from the node to ground.
  const r3x = 210, node = 360, r4Top = nonInvY + 20, r4Len = 70;
  source(sch, "vin", nonInvY, "Vin");
  sch.line([[35, nonInvY], [r3x, nonInvY]]);
  resistorAbove(sch, "r3", r3x, nonInvY);
  sch.line([[r3x + 90, nonInvY], [left, nonInvY]]);
  sch.dot(node, nonInvY);
  sch.line([[node, nonInvY], [node, r4Top]]);
  sch.hot("r4", [node - RESISTOR_H / 2 - 4, r4Top, RESISTOR_H + 8, r4Len], () => sch.resistorVertical(node, r4Top, r4Len));
  sch.place("r4", node - RESISTOR_H / 2 - FIELD_GAP - FIELD_W, r4Top + r4Len / 2 - FIELD_H / 2, FIELD_W);
  sch.line([[node, r4Top + r4Len], [node, r4Top + r4Len + 20]]);
  sch.ground(node, r4Top + r4Len + 20);

  amplifierParts(sch, geo);
  return sch;
}
