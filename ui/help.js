import { html, render, useEffect, useState } from "./lib/preact.js";
import { closeWindow } from "./lib/backend.js";
import { DEFAULT_SETTINGS, SETTINGS_KEY, applyPrefs } from "./lib/settings.js";
import { loadJson } from "./lib/storage.js";
import { Markdown, parseMarkdown } from "./components/markdown.js";
import { setLanguage, t } from "./lib/i18n.js";

/** The title of the help: the logo, the name and the motto of the program. */
const Banner = () => html`
  <header class="help-banner">
    <img src="img/logo.svg" alt="" width="56" height="56" />
    <div>
      <div class="help-title">ECW</div>
      <div class="help-name">${t("Engineers' Circuit Workbench")}</div>
      <div class="help-motto">${t("Fast calculations. Practical engineering.")}</div>
    </div>
  </header>`;

const readSettings = () => ({ ...DEFAULT_SETTINGS, ...loadJson(SETTINGS_KEY, {}) });

/** Help window: renders `help/<lang>.md` and follows the main window's theme, zoom and language. */
function Help() {
  const [settings, setSettings] = useState(readSettings);
  const [source, setSource] = useState("");

  useEffect(() => applyPrefs(settings), [settings.theme, settings.zoom]);

  // The main window writes settings to localStorage; other windows get a `storage` event.
  useEffect(() => {
    const onStorage = (e) => e.key === SETTINGS_KEY && setSettings(readSettings());
    window.addEventListener("storage", onStorage);
    return () => window.removeEventListener("storage", onStorage);
  }, []);

  useEffect(() => {
    let live = true;
    (async () => {
      try {
        // A language without its own help falls back to English (a failed fetch counts too).
        const load = async (lang) => {
          const res = await fetch(`help/${lang}.md`);
          if (!res.ok) throw new Error(`help/${lang}.md: ${res.status}`);
          const text = await res.text();
          // Tauri answers a missing file with index.html (status 200): that is not a help text.
          if (/^\s*<(!doctype|html)/i.test(text)) throw new Error(`help/${lang}.md is missing`);
          return text;
        };
        const text = await load(settings.lang).catch(() => load("en"));
        if (live) setSource(text);
      } catch (e) {
        if (live) setSource(`# Help\n\nCould not load help: ${e}`);
      }
    })();
    return () => { live = false; };
  }, [settings.lang]);

  useEffect(() => {
    const onKey = (e) => e.key === "Escape" && closeWindow();
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, []);

  setLanguage(settings.lang); // the banner, the contents and the figures follow the language
  const blocks = parseMarkdown(source);
  const sections = blocks.map((b, n) => [b, n]).filter(([b]) => b.type === "heading" && b.level <= 2);

  return html`
    <${Banner} />
    <div class="toc">
      <strong>${t("Table of Contents")}</strong>
      ${sections.map(([b, n]) => html`
        <button key=${n} class=${`toc-item${b.level > 1 ? " l2" : ""}`} type="button"
          onClick=${() => document.getElementById(`md-${n}`)?.scrollIntoView({ behavior: "smooth", block: "start" })}>
          ${b.text}
        </button>`)}
    </div>
    <${Markdown} blocks=${blocks} />`;
}

document.addEventListener("contextmenu", (e) => e.preventDefault());
render(html`<${Help} />`, document.getElementById("help-root"));
