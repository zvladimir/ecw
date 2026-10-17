import { RESISTOR_H, Schematic } from "../components/schematic.js";

/**
 * Thermistor and Rs in a divider read by an ADC. `top` puts the thermistor on the Vref side;
 * `rp` adds the linearising resistor in parallel with the thermistor.
 */
export function buildThermistorSchematic({ top = true, ntc = true, rp = false, bits = "" } = {}) {
  const sch = new Schematic(330, 400);
  const x = 70, lead = 20, body = 80, gap = 12;

  sch.hot("vref", [x - 12, 12, 90, 70], () => {
    sch.arrowUp(x, 20, 80);
    sch.label(x + 20, 30, "start", "middle", "Vref");
  });

  const element = (y, name, thermistor) => {
    sch.line([[x, y], [x, y + lead]]);
    sch.hot(thermistor ? "r0" : "rs", [x - 26, y + lead, 140, body], () => {
      if (thermistor) sch.thermistorVertical(x, y + lead, body);
      else sch.resistorVertical(x, y + lead, body);
      sch.label(x + RESISTOR_H / 2 + gap + (thermistor ? 14 : 0), y + lead + body / 2, "start", "middle", name);
    });
    sch.line([[x, y + lead + body], [x, y + lead + body + lead]]);
  };
  const first = top, thermistorLabel = ntc ? "NTC" : "PTC";
  const tap = 80 + 2 * lead + body;

  element(80, first ? thermistorLabel : "Rs", first);
  sch.dot(x, tap);
  element(tap, first ? "Rs" : thermistorLabel, !first);

  // Output to the ADC.
  sch.line([[x, tap], [235, tap]]);
  sch.hot("bits", [235, tap - 23, 80, 46], () => sch.box(235, tap, 80, 46, bits ? `ADC ${bits} bit` : "ADC"));
  sch.label(150, tap - 12, "middle", "bottom", "Vout");

  // Linearising resistor across the thermistor.
  if (rp) {
    const y0 = first ? 80 : tap, y1 = y0 + 2 * lead + body, xp = x + 100;
    sch.dot(x, y0);
    sch.dot(x, y1);
    sch.line([[x, y0], [xp, y0], [xp, y0 + lead]]);
    sch.hot("rp", [xp - 26, y0 + lead, 90, body], () => {
      sch.resistorVertical(xp, y0 + lead, body);
      sch.label(xp + RESISTOR_H / 2 + gap, y0 + lead + body / 2, "start", "middle", "Rp");
    });
    sch.line([[xp, y1 - lead], [xp, y1], [x, y1]]);
  }

  const end = tap + 2 * lead + body;
  sch.ground(x, end);
  return sch;
}
