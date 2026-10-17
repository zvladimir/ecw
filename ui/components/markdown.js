import { html, useEffect, useRef } from "../lib/preact.js";
import { openUrl } from "../lib/backend.js";
import { FIGURES } from "./figures.js";

/** KaTeX formula (global `katex` from vendor/katex). */
function Formula({ expr, display }) {
  const ref = useRef(null);
  useEffect(() => {
    try {
      katex.render(expr, ref.current, { displayMode: display, throwOnError: false });
    } catch {
      ref.current.textContent = expr;
    }
  }, [expr, display]);
  return display
    ? html`<div class="math-block" ref=${ref}></div>`
    : html`<span class="math" ref=${ref}></span>`;
}

const INLINE = new RegExp([
  String.raw`\*\*(?<bold>[^*]+)\*\*`,
  String.raw`\*(?<em>[^*\s][^*]*)\*`,
  String.raw`\`(?<code>[^\`]+)\``,
  String.raw`\[(?<label>[^\]]+)\]\((?<url>[^)]+)\)`,
  String.raw`!\[[^\]]*\]\([^)]*\)`,
  String.raw`\$\$(?<display>.+?)\$\$`,
  String.raw`\$(?<math>[^$]+)\$`,
].join("|"), "g");

function inline(text) {
  const out = [];
  let last = 0;
  // `matchAll` iterates over a copy of the regex: `inline` is re-entered for bold and
  // italic text, and a shared `lastIndex` would send the outer loop back to the start forever.
  for (const m of text.matchAll(INLINE)) {
    const g = m.groups;
    if (m.index > last) out.push(text.slice(last, m.index));
    if (g.bold !== undefined) out.push(html`<strong>${inline(g.bold)}</strong>`);
    else if (g.em !== undefined) out.push(html`<em>${inline(g.em)}</em>`);
    else if (g.code !== undefined) out.push(html`<code>${g.code}</code>`);
    else if (g.label !== undefined) out.push(html`<a class="link" href=${g.url} title=${g.url} onClick=${openLink}>${inline(g.label)}</a>`);
    else if (g.display !== undefined) out.push(html`<${Formula} expr=${g.display} display=${true} />`);
    else if (g.math !== undefined) out.push(html`<${Formula} expr=${g.math} display=${false} />`);
    last = m.index + m[0].length;
  }
  if (last < text.length) out.push(text.slice(last));
  return out;
}

/** Links open in the system browser (the allowed sites are listed in src-tauri/capabilities). */
function openLink(e) {
  e.preventDefault();
  openUrl(e.currentTarget.getAttribute("href")).catch(() => {});
}

const LIST_ITEM = /^\s*([-*]|\d+\.)\s+/;
const HEADING = /^(#{1,6})\s+(.*)$/;
const isTable = (line) => line.trim().startsWith("|");

/** Markdown subset → blocks: headings, lists, tables, paragraphs, **bold**, `code`, links, $math$. */
export function parseMarkdown(source) {
  const lines = source.split("\n");
  const blocks = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];
    if (!line.trim()) { i++; continue; }

    const heading = HEADING.exec(line);
    if (heading) {
      blocks.push({ type: "heading", level: heading[1].length, text: heading[2] });
      i++;
    } else if (isTable(line)) {
      const rows = [];
      while (i < lines.length && isTable(lines[i])) {
        const cells = lines[i].trim().replace(/^\||\|$/g, "").split("|").map((c) => c.trim());
        if (!cells.every((c) => /^:?-+:?$/.test(c))) rows.push(cells);
        i++;
      }
      blocks.push({ type: "table", rows });
    } else if (line.startsWith(">")) {
      const quote = [];
      while (i < lines.length && lines[i].startsWith(">")) {
        quote.push(lines[i].replace(/^>\s?/, "").trim());
        i++;
      }
      blocks.push({ type: "quote", text: quote.join(" ") });
    } else if (LIST_ITEM.test(line)) {
      const ordered = /^\s*\d+\./.test(line);
      // A numbered list may go on after a formula or a paragraph: it keeps its first number.
      const start = ordered ? parseInt(line.trim(), 10) : 1;
      const items = [];
      while (i < lines.length && LIST_ITEM.test(lines[i])) {
        items.push(lines[i].replace(LIST_ITEM, ""));
        i++;
      }
      blocks.push({ type: "list", ordered, start, items });
    } else {
      const para = [];
      while (i < lines.length && lines[i].trim() && !HEADING.test(lines[i]) && !LIST_ITEM.test(lines[i]) && !isTable(lines[i])) {
        para.push(lines[i].trim());
        i++;
      }
      const text = para.join(" ");
      const figure = /^!\[([^\]]*)\]\(fig:([a-z0-9_]+)\)$/.exec(text);
      if (figure) blocks.push({ type: "figure", caption: figure[1], name: figure[2] });
      else if (!/^!\[[^\]]*\]\([^)]*\)$/.test(text)) blocks.push({ type: "paragraph", text });
    }
  }
  return blocks;
}

export function Markdown({ blocks }) {
  return html`<div class="md">${blocks.map((b, n) => {
    switch (b.type) {
      case "heading": {
        const Tag = `h${b.level}`;
        return html`<${Tag} id=${`md-${n}`}>${inline(b.text)}<//>`;
      }
      case "table": {
        const [head, ...body] = b.rows;
        return html`<table class="tbl">
          <thead><tr>${head.map((c) => html`<th>${inline(c)}</th>`)}</tr></thead>
          <tbody>${body.map((r) => html`<tr>${r.map((c) => html`<td>${inline(c)}</td>`)}</tr>`)}</tbody>
        </table>`;
      }
      case "list": {
        const Tag = b.ordered ? "ol" : "ul";
        return html`<${Tag} start=${b.ordered && b.start !== 1 ? b.start : undefined}>${b.items.map((t) => html`<li>${inline(t)}</li>`)}<//>`;
      }
      case "quote":
        return html`<blockquote>${inline(b.text)}</blockquote>`;
      case "figure": {
        const Figure = FIGURES[b.name];
        return html`
          <figure class="figure">
            ${Figure ? html`<${Figure} />` : html`<div class="msg-err">Missing figure: ${b.name}</div>`}
            <figcaption>${inline(b.caption)}</figcaption>
          </figure>`;
      }
      default:
        return html`<p>${inline(b.text)}</p>`;
    }
  })}</div>`;
}
