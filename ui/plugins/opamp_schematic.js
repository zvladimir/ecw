import { RESISTOR_H, RESISTOR_LEN, Schematic } from "../components/schematic.js";

const FIELD_W = 140;
const FIELD_H = 54;
const FIELD_GAP = 6;
const GAIN_W = 104;
export const TOP_EXTRA = 26;
export const HEIGHT = 458; // the same for all three amplifiers (the difference one needs it)

/**
 * Inverting / non-inverting amplifier with slots for the input fields on the circuit.
 * With `annotate` the resistors get name labels instead (used by the help figures).
 */
export function buildOpampSchematic(topology, { annotate = false } = {}) {
  const inverting = topology === "inverting";
  const sch = new Schematic(880, HEIGHT, TOP_EXTRA);
  const put = (key, x, y, w = FIELD_W) => {
    // The signal ranges are named by the drawing itself (Vin, Vout): their fields carry no label.
    if (!annotate) return sch.place(key, x, y, w, key === "vin" || key === "vout");
    const names = { r1: "R1", r2: "R2" };
    if (names[key]) sch.label(x + FIELD_W / 2, y + FIELD_H - 4, "middle", "bottom", names[key]);
  };
  const aboveResistor = (rx, ry) => [
    rx + (RESISTOR_LEN - FIELD_W) / 2,
    ry - RESISTOR_H / 2 - FIELD_GAP - FIELD_H,
  ];

  const left = 400, tip = 620;
  const top = 110, bottom = 330, mid = 220;
  const upperY = 170, lowerY = 270; // inverting input / non-inverting input
  sch.hot("gain", [left, top, tip - left, bottom - top], () => {
    sch.line([[left, top], [tip, mid], [left, bottom], [left, top]]);
    sch.label(left + 8, upperY, "start", "middle", "−");
    sch.label(left + 8, lowerY, "start", "middle", "+");
  });
  const resistor = (key, x, y) => sch.hot(key, [x, y - RESISTOR_H, RESISTOR_LEN, 2 * RESISTOR_H], () => sch.resistor(x, y));
  const vinMark = (y, x = 30) => sch.hot("vin", [x - 26, y - 44, 90, 34], () => {
    sch.terminal(x, y);
    sch.label(x, y - 14, "middle", "bottom", "Vin");
  });
  put("gain", left + 40, mid - FIELD_H / 2, GAIN_W);

  const fbY = 50, outX = 700, nodeX = 340, termX = outX + 70;

  if (inverting) {
    const r1x = 130;
    vinMark(upperY);
    sch.line([[35, upperY], [r1x, upperY]]);
    resistor("r1", r1x, upperY);
    sch.line([[r1x + RESISTOR_LEN, upperY], [left, upperY]]);
    sch.dot(nodeX, upperY);
    put("r1", ...aboveResistor(r1x, upperY));
    put("vin", 10, upperY + 24);

    const r2x = 440;
    sch.line([[nodeX, upperY], [nodeX, fbY], [r2x, fbY]]);
    resistor("r2", r2x, fbY);
    sch.line([[r2x + RESISTOR_LEN, fbY], [outX, fbY], [outX, mid]]);
    put("r2", ...aboveResistor(r2x, fbY));

    sch.line([[left, lowerY], [nodeX, lowerY], [nodeX, 340]]);
    sch.ground(nodeX, 340);
  } else {
    const r2x = 210, r1x = 440;
    // In the help (no field under it) the input starts where the ground of R2 is, so both
    // inputs are drawn about as long; the page keeps it at the edge, above its field.
    const vinX = annotate ? r2x - 40 : 30;
    vinMark(lowerY, vinX);
    sch.line([[vinX + 5, lowerY], [left, lowerY]]);
    put("vin", 10, lowerY + 24);

    sch.line([[left, upperY], [nodeX, upperY]]);
    sch.dot(nodeX, upperY);
    sch.line([[nodeX, upperY], [r2x + RESISTOR_LEN, upperY]]);
    resistor("r2", r2x, upperY);
    sch.line([[r2x, upperY], [r2x - 40, upperY]]);
    sch.ground(r2x - 40, upperY);
    put("r2", ...aboveResistor(r2x, upperY));

    sch.line([[nodeX, upperY], [nodeX, fbY], [r1x, fbY]]);
    resistor("r1", r1x, fbY);
    sch.line([[r1x + RESISTOR_LEN, fbY], [outX, fbY], [outX, mid]]);
    put("r1", ...aboveResistor(r1x, fbY));
  }

  sch.line([[tip, mid], [termX - 5, mid]]);
  sch.dot(outX, mid);
  sch.hot("vout", [termX - 45, mid - 44, 90, 34], () => {
    sch.terminal(termX, mid);
    sch.label(termX, mid - 14, "middle", "bottom", "Vout");
  });
  const voutX = termX - FIELD_W / 2 - 10;
  put("vout", voutX, mid + 24);
  return sch;
}
