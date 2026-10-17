import { html, useEffect, useState } from "./lib/preact.js";
import { invoke, openUrl } from "./lib/backend.js";
import {
  DECIMALS, DEFAULT_SETTINGS, LANGUAGES, SETTINGS_KEY, SettingsContext, ZOOM,
  applyPrefs, clamp,
} from "./lib/settings.js";
import { setLanguage, t } from "./lib/i18n.js";
import { clearForms, usePersisted } from "./lib/storage.js";
import { Toasts, toast } from "./lib/toast.js";
import { SettingsDialog } from "./components/settings_dialog.js";
import { CATEGORIES, PLUGINS } from "./plugins/index.js";

const DONATE_URL = "https://boosty.to/desialsi";

const openHelp = () => invoke("open_help").catch((e) => toast(t("Cannot open help: {e}", { e })));

/** Settings as the UI uses them: persisted values may be stale or out of range. */
function sanitize(s) {
  return {
    ...s,
    decimals: clamp(s.decimals, DECIMALS),
    zoom: clamp(s.zoom, ZOOM),
    lang: LANGUAGES.some((l) => l.id === s.lang) ? s.lang : DEFAULT_SETTINGS.lang,
    view: PLUGINS.some((p) => p.id === s.view) ? s.view : DEFAULT_SETTINGS.view,
  };
}

function HeaderButton({ id, icon, label, title, onClick, className = "" }) {
  return html`
    <button id=${id} class=${`hbtn ${className}`} type="button" title=${title && t(title)} onClick=${onClick}>
      <span class="ico">${icon}</span><span>${t(label)}</span>
    </button>`;
}

function Header({ isFavorite, onFavorite, onSettings }) {
  return html`
    <header id="app-header">
      <div id="app-brand">
        <img id="app-logo" src="img/logo.svg" alt="" width="36" height="36" />
        <div id="app-name">
          <span id="app-title">ECW</span>
          <span id="app-subtitle">Engineers' Circuit Workbench</span>
        </div>
      </div>
      <div class="head-btns">
        <${HeaderButton} id="btn-fav" icon=${isFavorite ? "★" : "☆"} label="Favorite"
          className=${isFavorite ? "on" : ""} onClick=${onFavorite} />
        <${HeaderButton} id="btn-settings" icon="⚙" label="Settings" onClick=${onSettings} />
        <${HeaderButton} id="btn-help" icon="?" label="Help" title="Help (F1)" onClick=${openHelp} />
        <${HeaderButton} id="btn-donate" icon="♥" label="Donate"
          onClick=${() => openUrl(DONATE_URL).catch((e) => toast(t("Cannot open link: {e}", { e })))} />
      </div>
    </header>`;
}

function Sidebar({ settings, update, open }) {
  const [query, setQuery] = useState("");
  const q = query.trim().toLowerCase();
  const matches = (p) => !q || t(p.title).toLowerCase().includes(q);
  const byTitle = (a, b) => t(a.title).localeCompare(t(b.title));

  const item = (p, flat) => html`
    <button key=${p.id} type="button" class=${`item${p.id === settings.view ? " active" : ""}${flat ? " flat" : ""}`}
      onClick=${() => open(p.id)}>${t(p.title)}</button>`;

  const favorites = PLUGINS.filter((p) => settings.favorites.includes(p.id) && matches(p)).sort(byTitle);
  const toggleCategory = (c) => update({
    closed: settings.closed.includes(c) ? settings.closed.filter((x) => x !== c) : [...settings.closed, c],
  });

  return html`
    <aside id="sidebar">
      <input id="search" type="search" placeholder=${t("Search…")} autocomplete="off"
        value=${query} onInput=${(e) => setQuery(e.currentTarget.value)} />
      <nav id="tree">
        ${favorites.length > 0 && html`
          <div class="fav-head">★ ${t("Favorites")}</div>
          ${favorites.map((p) => item(p, true))}`}
        ${CATEGORIES.map((category) => {
          const items = PLUGINS.filter((p) => p.category === category && matches(p)).sort(byTitle);
          if (items.length === 0) return null;
          const closed = !q && settings.closed.includes(category);
          return html`
            <button key=${category} type="button" class=${`cat${closed ? " closed" : ""}`}
              onClick=${() => toggleCategory(category)}>${t(category)}</button>
            ${!closed && items.map((p) => item(p, false))}`;
        })}
      </nav>
    </aside>`;
}

export function App() {
  const [stored, update] = usePersisted(SETTINGS_KEY, DEFAULT_SETTINGS);
  const settings = sanitize(stored);
  setLanguage(settings.lang); // before the children render, so every `t()` below sees it
  const [showSettings, setShowSettings] = useState(false);

  useEffect(() => applyPrefs(settings), [settings.theme, settings.zoom]);

  useEffect(() => {
    const onKey = (e) => {
      if (e.key === "Escape") setShowSettings(false);
      if (e.key === "F1") { e.preventDefault(); openHelp(); }
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, []);

  const plugin = PLUGINS.find((p) => p.id === settings.view);
  const isFavorite = settings.favorites.includes(plugin.id);
  const toggleFavorite = () => update({
    favorites: isFavorite ? settings.favorites.filter((f) => f !== plugin.id) : [...settings.favorites, plugin.id],
  });

  const clearForm = () => clearForms(SETTINGS_KEY);

  return html`
    <${SettingsContext.Provider} value=${{ settings }}>
      <${Header} isFavorite=${isFavorite} onFavorite=${toggleFavorite} onSettings=${() => setShowSettings(true)} />
      <div id="body">
        <${Sidebar} settings=${settings} update=${update} open=${(view) => update({ view })} />
        <main id="main">
          <header id="title-bar">
            <div id="title-text">
              <h1 id="title">${t(plugin.title)}</h1>
              <p id="doc">${t(plugin.doc)}</p>
            </div>
            <button id="btn-clear-form" type="button" title=${t("Clear all fields")} onClick=${clearForm}>⌫</button>
          </header>
          <section id="content"><${plugin.view} key=${plugin.id} /></section>
        </main>
      </div>
      ${showSettings && html`<${SettingsDialog} settings=${settings} update=${update} onClose=${() => setShowSettings(false)} />`}
      <${Toasts} />
    <//>`;
}
