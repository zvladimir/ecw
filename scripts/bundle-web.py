#!/usr/bin/env python3
"""Packs the browser version of ECW (target/web/, made by build-web.sh) into ONE .html file.

The file opens by a double click from the disk: no server, no installation. ES modules and
`fetch` do not work from file://, so everything is inlined:
  * the modules go into an import map as data: URLs (relative imports become `ecw:<path>`),
  * the wasm bytes, the help texts, the images and the KaTeX fonts are embedded as base64,
  * the help window is the same file opened with `#help`.
Usage: bundle-web.py [target/web [output.html]]
"""
import base64
import json
import posixpath
import re
import sys
from pathlib import Path

root = Path(sys.argv[1] if len(sys.argv) > 1 else "target/web")
out = Path(sys.argv[2] if len(sys.argv) > 2 else "target/web/ecw.html")


def b64(data: bytes) -> str:
    return base64.b64encode(data).decode()


def data_uri(path: Path, mime: str) -> str:
    return f"data:{mime};base64,{b64(path.read_bytes())}"


# ------------------------------------------------------------------ images
IMAGES = {
    f"img/{p.name}": data_uri(p, {".svg": "image/svg+xml", ".png": "image/png"}[p.suffix])
    for p in sorted((root / "img").iterdir())
}

# ------------------------------------------------------------------ modules
IMPORT = re.compile(r"""(\bfrom\s*|\bimport\s*\(\s*)(["'])(\.{1,2}/[^"']+)\2""")
modules: dict[str, str] = {}  # `ecw:<path>` -> source with rewritten imports


def add_module(path: str) -> str:
    name = f"ecw:{path}"
    if name in modules:
        return name
    modules[name] = ""  # a cycle finds the name already taken
    source = (root / path).read_text(encoding="utf-8")

    def rewrite(m: re.Match) -> str:
        target = posixpath.normpath(posixpath.join(posixpath.dirname(path), m.group(3)))
        if not (root / target).is_file():
            return m.group(0)
        return f"{m.group(1)}{m.group(2)}{add_module(target)}{m.group(2)}"

    source = IMPORT.sub(rewrite, source)
    for image, uri in IMAGES.items():
        source = source.replace(image, uri)
    modules[name] = source
    return name


for entry in ("main.js", "help.js", "wasm/ecw_wasm.js"):
    add_module(entry)

import_map = {
    name: "data:text/javascript;base64," + b64(source.encode()) for name, source in modules.items()
}

# ------------------------------------------------------------------ styles
katex_css = (root / "vendor/katex/katex.min.css").read_text(encoding="utf-8")
katex_css = re.sub(
    r"url\(fonts/([^)]+\.woff2)\)",
    lambda m: f"url({data_uri(root / 'vendor/katex/fonts' / m.group(1), 'font/woff2')})",
    katex_css,
)
# Only woff2 is shipped; the woff/ttf fallbacks point at files that are not there.
katex_css = re.sub(r",url\(fonts/[^)]+\) format\(\"(?:woff|truetype)\"\)", "", katex_css)
assert "url(fonts/" not in katex_css, "KaTeX fonts left outside the file"
style = (root / "style.css").read_text(encoding="utf-8")

# ------------------------------------------------------------------ the page
files = {f"help/{p.name}": p.read_text(encoding="utf-8") for p in sorted((root / "help").glob("*.md"))}
wasm = b64((root / "wasm/ecw_wasm_bg.wasm").read_bytes())
katex_js = (root / "vendor/katex/katex.min.js").read_text(encoding="utf-8")


def script_safe(text: str) -> str:
    return text.replace("</script", "<\\/script").replace("<!--", "<\\!--")


boot = f"""
const files = {json.dumps(files)};
const realFetch = window.fetch.bind(window);
window.fetch = (url, ...rest) =>
  String(url) in files ? Promise.resolve(new Response(files[String(url)])) : realFetch(url, ...rest);
globalThis.ECW_WASM = Uint8Array.from(atob("{wasm}"), (c) => c.charCodeAt(0));
globalThis.ECW_HELP_URL = location.href.split("#")[0] + "#help";
"""

start = """
try {
  const mount = (id) => document.body.prepend(Object.assign(document.createElement("div"), { id }));
  if (location.hash === "#help") {
    mount("help-root");
    document.body.classList.add("help-page");
    document.title = "Help — ECW";
    await import("ecw:help.js");
  } else {
    mount("root");
    await import("ecw:main.js");
  }
} catch (e) {
  const box = document.createElement("pre");
  box.style.cssText = "white-space:pre-wrap;padding:16px;color:#c33";
  box.textContent = "ECW could not start. A current Firefox, Chrome, Edge or Safari is needed.\\n\\n" + (e && e.stack || e);
  document.body.prepend(box);
}
"""

page = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>ECW — Engineers' Circuit Workbench</title>
<style>{katex_css}</style>
<style>{style}</style>
</head>
<body>
<script>{script_safe(katex_js)}</script>
<script>{script_safe(boot)}</script>
<script type="importmap">{script_safe(json.dumps({"imports": import_map}))}</script>
<script type="module">{start}</script>
</body>
</html>
"""
out.write_text(page, encoding="utf-8")
print(f"single file: {out} ({out.stat().st_size / 1e6:.1f} MB, {len(modules)} modules)")
