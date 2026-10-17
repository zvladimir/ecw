import { t } from "../lib/i18n.js";
import { html, useLayoutEffect, useRef, useState } from "../lib/preact.js";

const COLORS = {
  nominal: "var(--nominal)", max: "var(--upper)", min: "var(--lower)",
  accent: "var(--accent)", irms: "var(--nominal)", iavg: "var(--iavg)",
};

/** How close, in picture units, the pointer must be to a line to stick to it. */
const SNAP = 28;
const W = 900, H = 520, L = 64, R = 18, T = 14, B = 46;

function niceTicks(lo, hi, target = 8) {
  const span = hi - lo;
  if (!(span > 0)) return [lo];
  const raw = span / target;
  const mag = 10 ** Math.floor(Math.log10(raw));
  const step = [1, 2, 5, 10].map((m) => m * mag).find((s) => s >= raw) ?? 10 * mag;
  const ticks = [];
  for (let v = Math.ceil(lo / step) * step; v <= hi + step * 1e-9; v += step) {
    ticks.push(Math.abs(v) < step * 1e-9 ? 0 : v);
  }
  return ticks;
}

const fmtTick = (v) => String(Number(v.toPrecision(6)));

/** y of a polyline at `x`, or null outside its x range. */
function yAt(points, x) {
  for (let i = 1; i < points.length; i++) {
    const [ax, ay] = points[i - 1], [bx, by] = points[i];
    if (x >= Math.min(ax, bx) && x <= Math.max(ax, bx)) {
      return bx === ax ? ay : ay + ((x - ax) / (bx - ax)) * (by - ay);
    }
  }
  return null;
}

/** Triangle with its tip at (x, y) pointing along the unit vector (dx, dy). */
function ArrowHead({ x, y, dx, dy }) {
  const len = 9, half = 3.5;
  const bx = x - dx * len, by = y - dy * len;
  const pts = `${x},${y} ${bx - dy * half},${by + dx * half} ${bx + dy * half},${by - dx * half}`;
  return html`<polygon points=${pts} fill="var(--weak)" />`;
}

/** The plot with its texts (legend names, axis labels, dimension labels) rendered. */
function localized(plot) {
  const named = (item) => ({ ...item, name: t(item.name) });
  const labelled = (item) => ({ ...item, label: t(item.label) });
  return {
    ...plot,
    x_label: t(plot.x_label), y_label: t(plot.y_label), x_sym: t(plot.x_sym),
    lines: plot.lines.map(named), levels: plot.levels.map(named),
    dims: plot.dims.map(labelled), marks: plot.marks.map(labelled),
  };
}

/**
 * Plot described by the backend: curves, horizontal levels, dashed guides,
 * dimension arrows (along x) and amplitude markers (along y).
 */
export function Plot({ plot: raw }) {
  const plot = localized(raw);
  const [tip, setTip] = useState(null);
  const [hidden, setHidden] = useState(() => new Set()); // names of the lines switched off in the legend
  const toggle = (name) => setHidden((h) => {
    const next = new Set(h);
    next.has(name) ? next.delete(name) : next.add(name);
    return next;
  });
  const tipRef = useRef(null);
  // The tip follows the pointer, but flips to the other side of it near the edge of the window.
  useLayoutEffect(() => {
    const el = tipRef.current;
    if (!tip || !el) return;
    const { width, height } = el.getBoundingClientRect();
    const gap = 14, margin = 6;
    const left = tip.left + width + margin > window.innerWidth ? tip.x - gap - width : tip.left;
    const top = tip.top + height + margin > window.innerHeight ? tip.y - gap - height : tip.top;
    el.style.left = `${Math.max(margin, left)}px`;
    el.style.top = `${Math.max(margin, top)}px`;
  }, [tip]);
  const clipId = useRef(`plot-clip-${Math.random().toString(36).slice(2)}`).current;
  const [x0, x1] = plot.x, [y0, y1] = plot.y;
  const sx = (x) => L + ((x - x0) / (x1 - x0)) * (W - L - R);
  const sy = (y) => H - B - ((y - y0) / (y1 - y0)) * (H - T - B);
  
  const onMove = (e) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const px = ((e.clientX - rect.left) / rect.width) * W;
    const py = ((e.clientY - rect.top) / rect.height) * H;
    if (px < L || px > W - R || py < T || py > H - B) return setTip(null);
    const x = x0 + ((px - L) / (W - L - R)) * (x1 - x0);
    // The pointer sticks to the nearest visible line or level within reach.
    const candidates = [
      ...plot.lines.filter((l) => !hidden.has(l.name)).map((l) => ({ ...l, y: yAt(l.points, x) })),
      ...plot.levels.filter((l) => !hidden.has(l.name)),
    ].filter((c) => c.y !== null && Math.abs(sy(c.y) - py) <= SNAP);
    const snap = candidates.sort((a, b) => Math.abs(sy(a.y) - py) - Math.abs(sy(b.y) - py))[0];
    const lines = [`${plot.x_sym} = ${x.toFixed(4)} ${plot.x_unit}`];
    if (snap) lines.push(`${snap.name} = ${snap.y.toFixed(4)} ${plot.y_unit}`);
    setTip({
      x: e.clientX, y: e.clientY, left: e.clientX + 14, top: e.clientY + 14, text: lines.join("\n"),
      snap: snap && { x, y: snap.y, kind: snap.kind },
    });
  };

  const legend = [...plot.lines, ...plot.levels];
  const midY = (T + H - B) / 2;

  return html`
    <div class="plot-wrap">
      <svg viewBox=${`0 0 ${W} ${H}`} onMouseMove=${onMove} onMouseLeave=${() => setTip(null)}>
        <defs><clipPath id=${clipId}><rect x=${L} y=${T} width=${W - L - R} height=${H - T - B} /></clipPath></defs>
        ${niceTicks(x0, x1).map((x) => html`
          <line class="grid" x1=${sx(x)} x2=${sx(x)} y1=${T} y2=${H - B} />
          <text x=${sx(x)} y=${H - B + 16} text-anchor="middle">${fmtTick(x)}</text>`)}
        ${niceTicks(y0, y1).map((y) => html`
          <line class="grid" x1=${L} x2=${W - R} y1=${sy(y)} y2=${sy(y)} />
          <text x=${L - 8} y=${sy(y)} text-anchor="end" dy="0.35em">${fmtTick(y)}</text>`)}
        <rect class="axis" x=${L} y=${T} width=${W - L - R} height=${H - T - B} fill="none" />
        <text x=${(L + W - R) / 2} y=${H - 8} text-anchor="middle">${plot.x_label}</text>
        <text x=${14} y=${midY} text-anchor="middle" transform=${`rotate(-90 14 ${midY})`}>${plot.y_label}</text>
        ${x0 < 0 && x1 > 0 && html`<line class="axis" x1=${sx(0)} x2=${sx(0)} y1=${T} y2=${H - B} />`}
        ${y0 < 0 && y1 > 0 && html`<line class="axis" x1=${L} x2=${W - R} y1=${sy(0)} y2=${sy(0)} />`}
        <g clip-path=${`url(#${clipId})`}>
          ${plot.guides.map(([ax, ay, bx, by]) => html`
            <line class="guide" x1=${sx(ax)} y1=${sy(ay)} x2=${sx(bx)} y2=${sy(by)} />`)}
          ${plot.levels.filter((lv) => !hidden.has(lv.name)).map((lv) => html`
            <line x1=${L} x2=${W - R} y1=${sy(lv.y)} y2=${sy(lv.y)} stroke=${COLORS[lv.kind]} stroke-width="1.4" />`)}
          ${plot.lines.filter((line) => !hidden.has(line.name)).map((line) => html`
            <polyline fill="none" stroke-linejoin="round" stroke=${COLORS[line.kind]} stroke-width=${line.width}
              points=${line.points.map(([x, y]) => `${sx(x)},${sy(y)}`).join(" ")} />`)}
          ${tip?.snap && html`
            <line class="guide" x1=${sx(tip.snap.x)} x2=${sx(tip.snap.x)} y1=${T} y2=${H - B} />
            <circle cx=${sx(tip.snap.x)} cy=${sy(tip.snap.y)} r="5.5" fill=${COLORS[tip.snap.kind]}
              stroke="var(--window)" stroke-width="2" />`}
          ${plot.dims.map((d) => html`
            <line class="shaft" x1=${sx(d.x0)} x2=${sx(d.x1)} y1=${sy(d.y)} y2=${sy(d.y)} />
            <${ArrowHead} x=${sx(d.x0)} y=${sy(d.y)} dx=${-1} dy=${0} />
            <${ArrowHead} x=${sx(d.x1)} y=${sy(d.y)} dx=${1} dy=${0} />
            <text x=${(sx(d.x0) + sx(d.x1)) / 2} y=${sy(d.y) + 4} text-anchor="middle"
              dominant-baseline="hanging">${d.label}</text>`)}
          ${plot.marks.map((m) => {
            const dir = Math.sign(sy(m.y1) - sy(m.y0)) || 1;
            return html`
              <line class="shaft" x1=${sx(m.x)} x2=${sx(m.x)} y1=${sy(m.y0)} y2=${sy(m.y1)} />
              <${ArrowHead} x=${sx(m.x)} y=${sy(m.y0)} dx=${0} dy=${-dir} />
              <${ArrowHead} x=${sx(m.x)} y=${sy(m.y1)} dx=${0} dy=${dir} />
              <text x=${sx(m.x) + 8} y=${(sy(m.y0) + sy(m.y1)) / 2} dy="0.35em">${m.label}</text>`;
          })}
        </g>
      </svg>
      ${legend.length > 1 && html`
        <div class="legend">${legend.map((l) => html`
          <button key=${l.name} type="button" class=${`legend-item${hidden.has(l.name) ? " off" : ""}`}
            aria-pressed=${!hidden.has(l.name)} title=${t("Show or hide the line")}
            onClick=${() => toggle(l.name)}><i style=${{ background: COLORS[l.kind] }}></i>${l.name}</button>`)}</div>`}
      ${tip && html`<div class="plot-tip" ref=${tipRef} style=${{ left: `${tip.left}px`, top: `${tip.top}px` }}>${tip.text}</div>`}
    </div>`;
}
