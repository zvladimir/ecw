import { HEIGHT, TOP_EXTRA } from "./opamp_schematic.js";
import { RESISTOR_H, RESISTOR_LEN, Schematic } from "../components/schematic.js";

const FIELD_W = 140;
const FIELD_H = 54;
const FIELD_GAP = 6;

/** Field slot centred over a horizontal resistor whose left lead is at (x, y). */
export const aboveResistor = (x, y) => [x + (RESISTOR_LEN - FIELD_W) / 2, y - RESISTOR_H / 2 - FIELD_GAP - FIELD_H];

/** The parts the two level shifters share: the amplifier with G, the feedback R1 and the output. */
export function amplifierParts(sch, { left, tip, top, bottom, mid, invY, nonInvY, nodeX, fbY, outX }) {
  sch.hot("gain", [left, top, tip - left, bottom - top], () => {
    sch.line([[left, top], [tip, mid], [left, bottom], [left, top]]);
    sch.label(left + 8, invY, "start", "middle", "−");
    sch.label(left + 8, nonInvY, "start", "middle", "+");
  });
  sch.place("gain", left + 40, mid - FIELD_H / 2, 104);

  // Feedback R1 from the output to the inverting node.
  const r1x = 440;
  sch.dot(nodeX, invY);
  sch.line([[nodeX, invY], [nodeX, fbY], [r1x, fbY]]);
  sch.hot("r1", [r1x, fbY - RESISTOR_H, RESISTOR_LEN, 2 * RESISTOR_H], () => sch.resistor(r1x, fbY));
  sch.line([[r1x + RESISTOR_LEN, fbY], [outX, fbY], [outX, mid]]);
  sch.place("r1", ...aboveResistor(r1x, fbY), FIELD_W);

  // Output.
  const termX = outX + 70;
  sch.line([[tip, mid], [termX - 5, mid]]);
  sch.dot(outX, mid);
  sch.terminal(termX, mid);
  sch.hot("vout", [termX - 45, mid - 44, 90, 34], () => {
    sch.label(termX, mid - 14, "middle", "bottom", "Vout");
  });
  sch.place("vout", termX - FIELD_W / 2 - 10, mid + 24, FIELD_W, true);
}

/** A signal source: a terminal at (30, y) named `name`, tied to the field `key`. */
export function source(sch, key, y, name) {
  sch.hot(key, [4, y - 44, 90, 34], () => {
    sch.terminal(30, y);
    sch.label(30, y - 14, "middle", "bottom", name);
  });
  sch.place(key, 10, y + 24, FIELD_W, true);
}

/** A horizontal resistor tied to the field `key`, with the field above it. */
export function resistorAbove(sch, key, x, y) {
  sch.hot(key, [x, y - RESISTOR_H, RESISTOR_LEN, 2 * RESISTOR_H], () => sch.resistor(x, y));
  sch.place(key, ...aboveResistor(x, y), FIELD_W);
}

/**
 * Inverting level shifter: Vin through R2 and Vref through R3 meet at the inverting input,
 * R1 is the feedback, the non-inverting input is grounded. The input fields sit on the circuit.
 */
export function buildLevelShifterSchematic() {
  const sch = new Schematic(880, HEIGHT, TOP_EXTRA);
  const geo = {
    left: 400, tip: 620, top: 110, bottom: 330, mid: 220,
    invY: 170, nonInvY: 270, nodeX: 340, fbY: 50, outX: 700,
  };
  const { left, invY, nonInvY } = geo;
  const vinY = 110, vrefY = 290, rx = 130, joinX = 300;

  // Vin - R2 and Vref - R3, joined at joinX and led to the inverting input.
  source(sch, "vin", vinY, "Vin");
  sch.line([[35, vinY], [rx, vinY]]);
  resistorAbove(sch, "r2", rx, vinY);
  source(sch, "vref", vrefY, "Vref");
  sch.line([[35, vrefY], [rx, vrefY]]);
  resistorAbove(sch, "r3", rx, vrefY);
  sch.line([[rx + RESISTOR_LEN, vinY], [joinX, vinY], [joinX, vrefY], [rx + RESISTOR_LEN, vrefY]]);
  sch.dot(joinX, invY);
  sch.line([[joinX, invY], [left, invY]]);

  amplifierParts(sch, geo);

  // Non-inverting input to ground.
  sch.line([[left, nonInvY], [375, nonInvY], [375, nonInvY + 20]]);
  sch.ground(375, nonInvY + 20);
  return sch;
}
