import { HEIGHT, TOP_EXTRA } from "./opamp_schematic.js";
import { RESISTOR_H, RESISTOR_LEN, Schematic } from "../components/schematic.js";

const FIELD_W = 140;
const FIELD_H = 54;
const FIELD_GAP = 6;

/**
 * Difference amplifier: Vi1 - R1 - "−" with R3 feedback; Vi2 - R2 - "+" with R4 to Vref.
 * The input fields sit on the circuit; R2 = R1 and R3 = R4 = G·R1 are only named.
 */
export function buildDifferentialSchematic() {
  const sch = new Schematic(880, HEIGHT, TOP_EXTRA);
  const left = 400, tip = 620, top = 110, bottom = 330, mid = 220;
  const invY = 170, nonInvY = 270, nodeX = 340, r1x = 130, fbY = 50, outX = 700;

  sch.hot("gain", [left, top, tip - left, bottom - top], () => {
    sch.line([[left, top], [tip, mid], [left, bottom], [left, top]]);
    sch.label(left + 8, invY, "start", "middle", "−");
    sch.label(left + 8, nonInvY, "start", "middle", "+");
  });

  // Vi1 - R1 - inverting node, R3 from the output back to it.
  sch.terminal(30, invY);
  sch.line([[35, invY], [r1x, invY]]);
  sch.hot("r1", [r1x, invY - RESISTOR_H, RESISTOR_LEN, 2 * RESISTOR_H], () => sch.resistor(r1x, invY));
  sch.place("r1", r1x + (RESISTOR_LEN - FIELD_W) / 2, invY - RESISTOR_H / 2 - FIELD_GAP - FIELD_H, FIELD_W);
  sch.line([[r1x + RESISTOR_LEN, invY], [left, invY]]);
  sch.dot(nodeX, invY);
  sch.line([[nodeX, invY], [nodeX, fbY], [440, fbY]]);
  sch.hot("r3", [440, fbY - RESISTOR_H, RESISTOR_LEN, 2 * RESISTOR_H], () => sch.resistor(440, fbY));
  sch.line([[440 + RESISTOR_LEN, fbY], [outX, fbY], [outX, mid]]);
  sch.place("r3", 440 + (RESISTOR_LEN - FIELD_W) / 2, fbY - RESISTOR_H / 2 - FIELD_GAP - FIELD_H, FIELD_W);

  // Vi2 - R2 - non-inverting node, R4 from it to Vref.
  sch.terminal(30, nonInvY);
  sch.hot("vdiff", [4, invY + 26, 90, nonInvY - invY - 52], () => {
    sch.label(44, (invY + nonInvY) / 2, "middle", "middle", "Vin diff");
  });
  sch.line([[35, nonInvY], [r1x, nonInvY]]);
  sch.hot("r1", [r1x, nonInvY - 54, RESISTOR_LEN, 54 + RESISTOR_H / 2], () => {
    sch.resistor(r1x, nonInvY);
    sch.label(r1x + RESISTOR_LEN / 2, nonInvY - 26, "middle", "bottom", "R2");
  });
  sch.line([[r1x + RESISTOR_LEN, nonInvY], [left, nonInvY]]);
  sch.dot(nodeX, nonInvY);
  sch.line([[nodeX, nonInvY], [nodeX, nonInvY + 20]]);
  sch.hot("r3", [nodeX - RESISTOR_H / 2 - 4, nonInvY + 20, RESISTOR_H + 70, 70], () => {
    sch.resistorVertical(nodeX, nonInvY + 20, 70);
    sch.label(nodeX + RESISTOR_H / 2 + 12, nonInvY + 55, "start", "middle", "R4");
  });
  sch.line([[nodeX, nonInvY + 90], [nodeX, nonInvY + 112]]);
  sch.hot("vref", [nodeX - 12, nonInvY + 103, 94, 28], () => {
    sch.terminal(nodeX, nonInvY + 117);
    sch.label(nodeX + 20, nonInvY + 117, "start", "middle", "Vref");
  });
  sch.place("vref", nodeX + 20, nonInvY + 117 + 14, FIELD_W, true);

  sch.place("gain", left + 40, mid - FIELD_H / 2, 104);

  // Vdiff = Vi2 − Vi1 under the inputs.
  sch.place("vdiff", 10, nonInvY + 24, FIELD_W, true);

  // Output.
  sch.line([[tip, mid], [765, mid]]);
  sch.dot(outX, mid);
  sch.terminal(770, mid);
  sch.hot("vo", [725, mid - 44, 90, 34], () => {
    sch.label(770, mid - 14, "middle", "bottom", "Vout");
  });
  sch.place("vo", 770 - FIELD_W / 2 - 10, mid + 24, FIELD_W, true);
  return sch;
}
