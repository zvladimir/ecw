import { useEffect, useRef, useState } from "./preact.js";

export function loadJson(key, fallback) {
  try {
    const raw = localStorage.getItem(key);
    return raw === null ? fallback : JSON.parse(raw);
  } catch {
    return fallback;
  }
}

export function saveJson(key, value) {
  try {
    localStorage.setItem(key, JSON.stringify(value));
  } catch {
    /* storage unavailable: values just won't persist */
  }
}

const clearers = new Map(); // storage key -> function emptying the form that uses it

/** Empties every mounted form except the one saved under `keep` (the app settings). */
export function clearForms(keep) {
  for (const [key, clear] of clearers) if (key !== keep) clear();
}

/**
 * State object saved to localStorage; `update(patch)` merges a patch into it.
 * "Clear form" empties text values and lists; the `tab` and the keys in `keepOnClear`
 * (selectors, modes) stay.
 */
export function usePersisted(key, defaults, keepOnClear = []) {
  const [state, setState] = useState(() => ({ ...defaults, ...loadJson(key, {}) }));
  useEffect(() => saveJson(key, state), [key, state]);
  const keepRef = useRef(keepOnClear);
  keepRef.current = keepOnClear;
  useEffect(() => {
    const clear = () => setState((s) => Object.fromEntries(Object.entries(s).map(([k, v]) => [
      k,
      k === "tab" || keepRef.current.includes(k) ? v : typeof v === "string" ? "" : Array.isArray(v) ? [] : v,
    ])));
    clearers.set(key, clear);
    return () => clearers.delete(key);
  }, [key]);
  const update = (patch) => setState((s) => ({ ...s, ...(typeof patch === "function" ? patch(s) : patch) }));
  return [state, update];
}
