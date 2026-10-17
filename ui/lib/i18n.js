import { MESSAGES } from "../locales/en.js";
import { RU } from "../locales/ru.js";
import { EO } from "../locales/eo.js";

/**
 * Interface strings. The English text is the key: `t("Design Inputs")` returns it unchanged
 * unless the chosen language has a translation. Placeholders look like `{name}`.
 *
 * The backend does not speak any language: its replies carry texts as messages `{ id, args }`
 * (`t(message)` renders one) or as plain strings that need no translation (numbers, symbols).
 * The English text of every message id is in `locales/en.js`; a language overrides it.
 *
 * To add a language, put `{ "Design Inputs": "…", "error.solve.ordered": "…", … }` into
 * `DICTIONARIES` under its code (the same code as `LANGUAGES` and `help/<code>.md`). Missing
 * entries stay English.
 */
export const DICTIONARIES = { ru: RU, eo: EO };

let dictionary = {};

export function setLanguage(code) {
  dictionary = DICTIONARIES[code] ?? {};
}

const fill = (s, vars) => s.replace(/\{(\w+)\}/g, (_, k) => vars[k] ?? "");

export function t(text, vars) {
  if (text !== null && typeof text === "object") {
    const { id, args = {} } = text;
    const rendered = Object.fromEntries(Object.entries(args).map(([k, v]) => [k, t(v)]));
    return fill(dictionary[id] ?? MESSAGES[id] ?? id, rendered);
  }
  const s = dictionary[text] ?? text;
  return vars ? fill(s, vars) : s;
}
