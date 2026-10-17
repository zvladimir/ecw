// The backend of the UI. In the desktop app it is the Tauri API (`withGlobalTauri` exposes it
// as window.__TAURI__); in a plain browser the same calculators run as WebAssembly (`wasm/`,
// built from crates/ecw-wasm by scripts/build-web.sh). The single-file build (scripts/bundle-web.py)
// embeds the wasm bytes in `globalThis.ECW_WASM` and the help address in `ECW_HELP_URL`.
const tauri = () => window.__TAURI__;

let wasm;
const loadWasm = () =>
  (wasm ??= import("../wasm/ecw_wasm.js").then(async (m) => {
    await m.default(globalThis.ECW_WASM && { module_or_path: globalThis.ECW_WASM });
    return m;
  }));

const webInvoke = async (command, args) => {
  if (command === "solve") {
    const m = await loadWasm();
    return JSON.parse(m.solve(args.id, JSON.stringify(args.request)));
  }
  if (command === "open_help") {
    window.open(globalThis.ECW_HELP_URL ?? "help.html", "ecw-help", "popup,width=680,height=560");
    return;
  }
  throw new Error(`unknown command: ${command}`);
};

export const invoke = (command, args) =>
  tauri() ? tauri().core.invoke(command, args) : webInvoke(command, args);

export const openUrl = async (url) => {
  if (tauri()) return tauri().opener.openUrl(url);
  window.open(url, "_blank", "noopener,noreferrer");
};

export const closeWindow = async () =>
  tauri() ? tauri().window.getCurrentWindow().close() : window.close();
