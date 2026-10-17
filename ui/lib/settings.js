import { createContext, useContext } from "./preact.js";

export const DECIMALS = { min: 0, max: 9, default: 4 };
export const ZOOM = { min: 50, max: 150, default: 100 };
/** Interface languages: code (matches `help/<code>.md`) and native name. */
export const LANGUAGES = [{ id: "en", name: "English" }, { id: "ru", name: "Русский" }, { id: "eo", name: "Esperanto" }];
export const THEMES = [
  { id: "dark", name: "Dark" },
  { id: "light", name: "Light" },
  { id: "vscode", name: "VS Code" },
];

export const SETTINGS_KEY = "ecw.settings";

export const DEFAULT_SETTINGS = {
  theme: "dark",
  decimals: DECIMALS.default,
  zoom: ZOOM.default,
  lang: LANGUAGES[0].id,
  view: "ohms_law",
  closed: [],
  favorites: [],
};

export const clamp = (v, { min, max, default: fallback }) =>
  (Number.isFinite(v) ? Math.min(max, Math.max(min, Math.round(v))) : fallback);

export function applyPrefs({ theme, zoom }) {
  document.documentElement.dataset.theme = theme;
  document.documentElement.style.zoom = `${zoom}%`;
}

/** `{ settings }`: the current settings. */
export const SettingsContext = createContext({ settings: DEFAULT_SETTINGS });

export const useSettings = () => useContext(SettingsContext);
