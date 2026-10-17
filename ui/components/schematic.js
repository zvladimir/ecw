import { html } from "../lib/preact.js";
import { focusField } from "./form.js";

export const RESISTOR_LEN = 90;
export const RESISTOR_H = 28;
const DOT_R = 4;
const TERMINAL_R = 5;
const GROUND_LEAD = 12;
const GROUND_BARS = [18, 11, 4];
const GROUND_STEP = 7;
const STROKE = { fill: "none", stroke: "currentColor", "stroke-width": 2 };

/**
 * Circuit schematic in a logical coordinate space. Drawing calls collect SVG shapes;
 * `place()` reserves a spot for an input field. `<SchematicView>` scales both together.
 */
export class Schematic {
  constructor(width, height, topExtra = 0) {
    this.w = width;
    this.h = height;
    this.top = topExtra;
    this.items = [];
    this.slots = [];
  }

  line(points) {
    const pts = points.map(([x, y]) => `${x},${y}`).join(" ");
    this.items.push(html`<polyline points=${pts} ...${STROKE} stroke-linejoin="round" />`);
  }

  dot(x, y) {
    this.items.push(html`<circle cx=${x} cy=${y} r=${DOT_R} fill="currentColor" />`);
  }

  terminal(x, y) {
    this.items.push(html`<circle cx=${x} cy=${y} r=${TERMINAL_R} ...${STROKE} />`);
  }

  /** `anchor`: "start" | "middle"; `base`: "middle" | "bottom" | "top". */
  label(x, y, anchor, base, text) {
    const dy = { middle: "0.35em", bottom: "-0.2em", top: "0.8em" }[base];
    this.items.push(html`<text x=${x} y=${y} text-anchor=${anchor} dy=${dy}>${text}</text>`);
  }

  /** Horizontal resistor body with its left lead at (x, y). */
  resistor(x, y) {
    this.items.push(html`<rect x=${x} y=${y - RESISTOR_H / 2} width=${RESISTOR_LEN} height=${RESISTOR_H} ...${STROKE} />`);
  }

  /** Vertical resistor body of `len` with its top lead at (x, y). */
  resistorVertical(x, y, len) {
    this.items.push(html`<rect x=${x - RESISTOR_H / 2} y=${y} width=${RESISTOR_H} height=${len} ...${STROKE} />`);
  }

  /**
   * Vertical thermistor with its top lead at (x, y): a resistor body crossed by a slanted line
   * that ends in a foot, the usual symbol of a temperature-dependent resistor.
   */
  thermistorVertical(x, y, len) {
    this.resistorVertical(x, y, len);
    this.line([[x - 22, y + len - 8], [x - 8, y + len - 8], [x + 20, y + 8]]);
  }

  /** Box centred vertically on `y` with its left edge at `x`, for example the ADC input. */
  box(x, y, width, height, text) {
    this.items.push(html`<rect x=${x} y=${y - height / 2} width=${width} height=${height} ...${STROKE} />`);
    this.label(x + width / 2, y, "middle", "middle", text);
  }

  /** Line from (x, tailY) up to an arrow head at (x, tipY). */
  arrowUp(x, tipY, tailY) {
    this.line([[x, tailY], [x, tipY]]);
    this.line([[x - 7, tipY + 14], [x, tipY], [x + 7, tipY + 14]]);
  }

  ground(x, y) {
    this.line([[x, y], [x, y + GROUND_LEAD]]);
    GROUND_BARS.forEach((half, i) => {
      const yy = y + GROUND_LEAD + i * GROUND_STEP;
      this.line([[x - half, yy], [x + half, yy]]);
    });
  }

  /**
   * Draws what `build()` adds as one clickable part tied to the input field `key`:
   * clicking it selects the field (see `focusField`). `hit` is
   * [x, y, width, height], an invisible area that makes thin lines easy to hit.
   */
  hot(key, hit, build) {
    const start = this.items.length;
    build();
    const parts = this.items.splice(start);
    const [x, y, w, h] = hit;
    this.items.push(html`
      <g class="hot" data-field=${key}>
        <rect class="hit" x=${x} y=${y} width=${w} height=${h} fill="transparent" stroke="none" />
        ${parts}
      </g>`);
  }

  /**
   * Replaces the field slots by name labels, for a drawing without fields (the help):
   * `names` maps a field key to its label; slots without a name are dropped.
   */
  labelSlots(names) {
    for (const s of this.slots) {
      if (names[s.key]) this.label(s.x + s.width / 2, s.y + 50, "middle", "bottom", names[s.key]);
    }
    this.slots = [];
    return this;
  }

  /** Reserves a field slot with its top-left corner at (x, y). */
  place(key, x, y, width, hideLabel = false) {
    this.slots.push({ key, x, y, width, hideLabel });
  }
}

/** Click on a clickable part of a drawing (see `hot`): selects its field. */
export function pickField(e) {
  const key = e.target.closest("svg [data-field]")?.dataset.field;
  if (key) focusField(key);
}

/** Renders the schematic; `renderSlot(key)` supplies the field for each reserved slot. */
export function SchematicView({ sch, renderSlot }) {
  const total = sch.h + sch.top;
  // Tab goes along the signal flow, left to right and top to bottom, not in the order of `place()`.
  const slots = [...sch.slots].sort((a, b) => a.x - b.x || a.y - b.y);
  return html`
    <div class="sch" style=${{ "--w": sch.w, maxWidth: `${sch.w}px` }} onClick=${pickField}>
      <svg viewBox=${`0 ${-sch.top} ${sch.w} ${total}`}>${sch.items}</svg>
      ${slots.map((s) => html`
        <div key=${s.key} class="sch-slot" style=${{
          left: `${(s.x / sch.w) * 100}%`,
          top: `${((s.y + sch.top) / total) * 100}%`,
          width: `${(s.width / sch.w) * 100}%`,
        }}>${renderSlot(s.key)}</div>`)}
    </div>`;
}

/** The whole schematic as a figure, without field slots (reports and the help). */
export function SchematicDrawing({ sch }) {
  return html`
    <div class="sch fig-sch" style=${{ "--w": sch.w, width: `${sch.w}px`, maxWidth: "100%" }} onClick=${pickField}>
      <svg viewBox=${`0 ${-sch.top} ${sch.w} ${sch.h + sch.top}`}>${sch.items}</svg>
    </div>`;
}
