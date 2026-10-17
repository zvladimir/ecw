import { html } from "../lib/preact.js";
import { buildDividerSchematic } from "../plugins/divider_schematic.js";
import { buildLevelShifterSchematic } from "../plugins/level_shifter_schematic.js";
import { buildLevelShifterNiSchematic } from "../plugins/level_shifter_ni_schematic.js";
import { buildDifferentialSchematic } from "../plugins/opamp_differential_schematic.js";
import { buildOpampSchematic } from "../plugins/opamp_schematic.js";
import { Plot } from "./plot.js";
import { Schematic, SchematicDrawing } from "./schematic.js";

/** Figures for the help: `![caption](fig:name)` in Markdown picks `FIGURES[name]`. */

const schematic = (build) => () => html`<${SchematicDrawing} sch=${build()} />`;

function seriesParallel() {
  const s = new Schematic(600, 205);
  // Series pair.
  s.terminal(20, 70);
  s.line([[25, 70], [60, 70]]);
  s.resistor(60, 70);
  s.line([[150, 70], [170, 70]]);
  s.resistor(170, 70);
  s.line([[260, 70], [295, 70]]);
  s.terminal(300, 70);
  s.label(105, 46, "middle", "bottom", "R1");
  s.label(215, 46, "middle", "bottom", "R2");
  s.label(160, 190, "middle", "middle", "R = R1 + R2");
  // Parallel pair.
  s.terminal(340, 80);
  s.line([[345, 80], [375, 80]]);
  s.line([[375, 40], [375, 120]]);
  s.dot(375, 80);
  for (const y of [40, 120]) {
    s.line([[375, y], [400, y]]);
    s.resistor(400, y);
    s.line([[490, y], [525, y]]);
  }
  s.line([[525, 40], [525, 120]]);
  s.dot(525, 80);
  s.line([[525, 80], [555, 80]]);
  s.terminal(560, 80);
  s.label(445, 16, "middle", "bottom", "R1");
  s.label(445, 150, "middle", "top", "R2");
  s.label(450, 190, "middle", "middle", "R = R1·R2 / (R1+R2)");
  return s;
}

/** Standard series on a logarithmic scale: equal steps in the exponent, not in the value. */
const E6 = [1.0, 1.5, 2.2, 3.3, 4.7, 6.8];
const E12 = [1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2];
const E24 = [
  1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0,
  3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6, 6.2, 6.8, 7.5, 8.2, 9.1,
];

function ESeriesFigure() {
  const x0 = 70, width = 540;
  const px = (v) => x0 + Math.log10(v) * width;
  const rows = [["E6", E6, 40, true], ["E12", E12, 85, true], ["E24", E24, 130, false]];
  return html`
    <div class="sch fig-sch" style=${{ maxWidth: "680px" }}>
      <svg viewBox="0 0 640 175">
        ${rows.map(([name, values, y, labelled]) => html`
          <text x=${8} y=${y} dy="0.35em">${name}</text>
          <line x1=${x0} x2=${x0 + width} y1=${y} y2=${y} stroke="currentColor" stroke-width="1" opacity="0.4" />
          ${values.concat(10).map((v) => html`
            <circle cx=${px(v)} cy=${y} r=${4} fill="currentColor" />
            ${labelled && html`<text class="small" x=${px(v)} y=${y - 10} text-anchor="middle">${v}</text>`}`)}`)}
        <text class="small" x=${x0} y=${165} text-anchor="middle">1</text>
        <text class="small" x=${x0 + width} y=${165} text-anchor="middle">10</text>
        <text class="small" x=${x0 + width / 2} y=${165} text-anchor="middle">log scale</text>
      </svg>
    </div>`;
}

// ---------------------------------------------------------------- plots

const WAVE_PERIOD = 10; // ms
const tent = (t, width) => (t <= width / 2 ? (2 * t) / width : (2 * (width - t)) / width);

/** Example pulses: value at time `t` within one period, times to annotate, peaks to mark. */
const WAVES = {
  half_sine: {
    f: (t) => (t < 4 ? Math.sin((Math.PI * t) / 4) : 0),
    times: [["ton", 0, 4]],
    peaks: [["Ip", 2, 1]],
  },
  rect: {
    f: (t) => (t < 4 ? 1 : 0),
    times: [["ton", 0, 4]],
    peaks: [["Ip", 2, 1]],
  },
  trapezoid: {
    f: (t) => (t < 5 ? 0.4 + (0.6 * t) / 5 : 0),
    times: [["ton", 0, 5]],
    peaks: [["Ip1", 0.75, 0.4], ["Ip2", 4.25, 1]],
  },
  triangular: {
    f: (t) => (t < 3 ? t / 3 : t < 5 ? (5 - t) / 2 : 0),
    times: [["t1", 0, 3], ["t2", 3, 5], ["t3", 5, 10]],
    peaks: [["Imax", 4, 1]],
  },
  offset: {
    f: (t) => (t < 3 ? 0.3 + (0.7 * t) / 3 : t < 5 ? 0.3 + (0.7 * (5 - t)) / 2 : 0),
    times: [["t1", 0, 3], ["t2", 3, 5], ["t3", 5, 10]],
    peaks: [["Imin", 0.45, 0.3], ["Imax", 3, 1]],
  },
  bipolar: {
    f: (t) => (t < 3 ? -0.8 * tent(t, 3) : t < 6 ? tent(t - 3, 3) : 0),
    times: [["t1", 0, 3], ["t2", 3, 6], ["t3", 6, 10]],
    peaks: [["Imin", 1.5, -0.8], ["Imax", 4.5, 1]],
  },
};

function wavePlot({ f, times, peaks }) {
  const T = WAVE_PERIOD;
  const sample = (n, span) => Array.from({ length: n + 1 }, (_, i) => {
    const t = (span * i) / n;
    return [t, f(t % T)];
  });
  const one = sample(2000, T).map(([, y]) => y);
  const avg = one.reduce((a, b) => a + b, 0) / one.length;
  const rms = Math.sqrt(one.reduce((a, b) => a + b * b, 0) / one.length);

  const yMin = Math.min(0, ...one), yMax = Math.max(...one);
  const span = yMax - yMin;
  const yLo = yMin - 0.55 * span, yHi = yMax + 0.15 * span;
  const bottom = yLo + 0.05 * span;

  const guides = [];
  const marks = [];
  for (const x of new Set([0, T, ...times.flatMap(([, a, b]) => [a, b])])) {
    for (const xs of [x, x + T]) if (xs <= 2 * T) guides.push([xs, yMin, xs, bottom]);
  }
  for (const [, , y] of peaks) if (y !== 0) guides.push([0, y, 2 * T, y]);
  for (const [label, x, y] of peaks) marks.push({ x, y0: 0, y1: y, label });

  const tierY = (tier) => bottom + (tier === 0 ? 0.72 : 0.24) * (yMin - 0.05 * span - bottom);
  const dims = [
    ...times.map(([label, x0, x1]) => ({ x0, x1, y: tierY(0), label })),
    { x0: 0, x1: T, y: tierY(1), label: "T" },
  ];

  return {
    x: [0, 2 * T], y: [yLo, yHi],
    x_label: "t, ms", y_label: "i, A (example)", x_sym: "t", x_unit: "ms", y_unit: "A",
    lines: [{ kind: "accent", name: "i(t)", points: sample(800, 2 * T), width: 1.8 }],
    levels: [
      { kind: "irms", name: `Irms = ${rms.toFixed(3)} A`, y: rms },
      { kind: "iavg", name: `Iavg = ${avg.toFixed(3)} A`, y: avg },
    ],
    guides, dims, marks,
  };
}

function transferPlot() {
  const xs = Array.from({ length: 121 }, (_, i) => -3 + i * 0.05);
  return {
    x: [-3.3, 3.3], y: [-20, 20],
    x_label: "Vin, V", y_label: "Vout, V", x_sym: "Vin", x_unit: "V", y_unit: "V",
    lines: [
      { kind: "nominal", name: "Inverting, G = −5", points: xs.map((x) => [x, -5 * x]), width: 2 },
      { kind: "max", name: "Non-inverting, G = +6", points: xs.map((x) => [x, 6 * x]), width: 2 },
    ],
    levels: [], guides: [], dims: [], marks: [],
  };
}

/** Dashed outline of the window a level shifter maps onto: Vin range across, Vout range up. */
const windowGuides = ([x0, x1], [y0, y1]) => [[x0, y0, x1, y0], [x0, y1, x1, y1], [x0, y0, x0, y1], [x1, y0, x1, y1]];

/**
 * How a level shifter works: the gain alone gives the right slope in the wrong place, and the
 * reference moves the line by the offset into the window of the two ranges.
 */
function shiftPlot({ x, y, vin, vout, gain, offset, slopeName, shiftedName }) {
  const xs = Array.from({ length: 61 }, (_, i) => x[0] + ((x[1] - x[0]) * i) / 60);
  return {
    x, y,
    x_label: "Vin, V", y_label: "Vout, V", x_sym: "Vin", x_unit: "V", y_unit: "V",
    lines: [
      { kind: "max", name: slopeName, points: xs.map((v) => [v, gain * v]), width: 1.6 },
      { kind: "nominal", name: shiftedName, points: xs.map((v) => [v, gain * v + offset]), width: 2.4 },
    ],
    levels: [],
    guides: windowGuides(vin, vout),
    dims: [], marks: [],
  };
}

const plot = (make) => () => html`<${Plot} plot=${make()} />`;

export const FIGURES = {
  series_parallel: schematic(seriesParallel),
  eseries: ESeriesFigure,
  divider2: schematic(() => buildDividerSchematic("two")),
  divider3: schematic(() => buildDividerSchematic("three")),
  opamp_inv: schematic(() => buildOpampSchematic("inverting", { annotate: true })),
  opamp_noninv: schematic(() => buildOpampSchematic("non_inverting", { annotate: true })),
  differential: schematic(() => buildDifferentialSchematic().labelSlots({ r1: "R1", r3: "R3" })),
  shifter_inv: schematic(() => buildLevelShifterSchematic().labelSlots({ r1: "R1", r2: "R2", r3: "R3" })),
  shifter_ni: schematic(() => buildLevelShifterNiSchematic().labelSlots({ r1: "R1", r2: "R2", r3: "R3", r4: "R4" })),
  shift_inv: plot(() => shiftPlot({
    x: [-6, 0], y: [-1, 5.5], vin: [-5, -1], vout: [0.05, 3.3], gain: -0.8125, offset: -0.7625,
    slopeName: "−G·Vin (no reference)", shiftedName: "−G·Vin − Vref·R1/R3",
  })),
  shift_ni: plot(() => shiftPlot({
    x: [0, 6], y: [-4, 7], vin: [2, 5], vout: [0.05, 4.95], gain: 1.6333, offset: -3.2167,
    slopeName: "G·Vin (no reference)", shiftedName: "G·Vin − Vref·R1/R2",
  })),
  transfer: plot(transferPlot),
  wave_half_sine: plot(() => wavePlot(WAVES.half_sine)),
  wave_rect: plot(() => wavePlot(WAVES.rect)),
  wave_trapezoid: plot(() => wavePlot(WAVES.trapezoid)),
  wave_triangular: plot(() => wavePlot(WAVES.triangular)),
  wave_offset: plot(() => wavePlot(WAVES.offset)),
  wave_bipolar: plot(() => wavePlot(WAVES.bipolar)),
};
