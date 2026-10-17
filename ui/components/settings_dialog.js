import { html } from "../lib/preact.js";
import { DECIMALS, DEFAULT_SETTINGS, LANGUAGES, THEMES, ZOOM, clamp } from "../lib/settings.js";
import { t } from "../lib/i18n.js";

/** A modal window: closed by the × button, by a click on the backdrop, or by Esc (the app). */
function Dialog({ title, onClose, children }) {
  return html`
    <div class="overlay" onClick=${(e) => e.target === e.currentTarget && onClose()}>
      <div class="dialog">
        <div class="dialog-head">
          <strong>${t(title)}</strong>
          <button class="clear" type="button" title=${t("Close (Esc)")} onClick=${onClose}>×</button>
        </div>
        <div class="dialog-body">${children}</div>
      </div>
    </div>`;
}

/** The settings: language, theme and zoom of the interface, and the decimals of the results.
 *  A change applies at once and is saved by `update`. */
export function SettingsDialog({ settings, update, onClose }) {
  const row = (label, input) => html`<label class="set-row"><span>${t(label)}</span>${input}</label>`;
  const number = (key, range, step) => html`
    <input type="number" min=${range.min} max=${range.max} step=${step} value=${settings[key]}
      onChange=${(e) => update({ [key]: clamp(Number(e.currentTarget.value), range) })} />`;
  const choice = (key, options) => html`
    <select value=${settings[key]} onChange=${(e) => update({ [key]: e.currentTarget.value })}>
      ${options.map((o) => html`<option key=${o.id} value=${o.id}>${t(o.name)}</option>`)}
    </select>`;

  return html`
    <${Dialog} title="Settings" onClose=${onClose}>
      <h3>${t("Appearance")}</h3>
      ${row("Language", choice("lang", LANGUAGES))}
      ${row("Theme", choice("theme", THEMES))}
      ${row("Zoom, %", number("zoom", ZOOM, 10))}
      <h3>${t("Output")}</h3>
      ${row("Decimal places", number("decimals", DECIMALS, 1))}
      <p class="msg-hint">${t("Settings are saved automatically in this app's storage.")}</p>
      <button class="copy-btn" type="button" onClick=${() => update({
        lang: DEFAULT_SETTINGS.lang, theme: DEFAULT_SETTINGS.theme,
        zoom: DEFAULT_SETTINGS.zoom, decimals: DEFAULT_SETTINGS.decimals,
      })}>${t("Reset to defaults")}</button>
    <//>`;
}
