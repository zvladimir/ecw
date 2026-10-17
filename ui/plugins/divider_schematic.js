import { RESISTOR_H, Schematic } from "../components/schematic.js";

/** Voltage divider with two or three resistors: Vin on top, taps to the right, ground at the bottom. */
export function buildDividerSchematic(mode) {
  const three = mode === "three";
  const sch = new Schematic(260, three ? 510 : 380);
  const x = 60, outEnd = 210, lead = 40, nodeLead = 20, resLen = 70, gap = 10;

  sch.hot("vin", [x - 14, 8, 100, 80], () => {
    sch.arrowUp(x, 20, 80);
    sch.label(x + gap * 2, 30, "start", "middle", "Vin");
  });

  let y = 80;
  const resistor = (name, key) => {
    sch.line([[x, y], [x, y + nodeLead]]);
    y += nodeLead;
    sch.hot(key, [x - RESISTOR_H / 2 - 4, y, RESISTOR_H + gap * 2 + 70, resLen], () => {
      sch.resistorVertical(x, y, resLen);
      sch.label(x + RESISTOR_H / 2 + gap, y + resLen / 2, "start", "middle", name);
    });
    y += resLen;
  };
  const tap = (name, key) => {
    sch.line([[x, y], [x, y + lead]]);
    y += lead;
    sch.dot(x, y);
    sch.hot(key, [x + 4, y - 30, outEnd - x + 12, 40], () => {
      sch.line([[x, y], [outEnd - 5, y]]);
      sch.terminal(outEnd, y);
      sch.label((x + outEnd) / 2 + gap, y - gap, "middle", "bottom", name);
    });
  };

  resistor("R1", "r1");
  tap(three ? "Vout1" : "Vout", "vout1");
  resistor("R2", "r2");
  if (three) { tap("Vout2", "vout2"); resistor("R3", "r3"); }

  sch.line([[x, y], [x, y + lead]]);
  sch.ground(x, y + lead);
  return sch;
}
