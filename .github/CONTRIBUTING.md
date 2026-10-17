# Contributing to ECW

ECW is a Tauri 2 app: the calculations are in Rust (`crates/ecw-core`), the interface is plain
ES modules with Preact (`ui/`, no Node and no bundler), and `src-tauri` only passes requests from
one to the other.

```
cargo run -p ecw          # run the app
cargo test -p ecw-core    # unit, golden and contract tests
```

`ui/` is embedded into the binary at build time: rebuild after changing it.

## Adding a calculator

A calculator is a backend module that turns field texts into a reply, and a manifest that
describes its page. Take the closest existing one as a template: `opamp_differential` for a
page with fields on a schematic, `ohms_law` for a plain form.

1. **The physics** — `crates/ecw-core/src/<name>/`. Calculate on `uom` quantities
   (`ElectricPotential`, `ElectricalResistance`, `Ratio`, …), never on bare `f64`: the compiler
   then checks the units of every formula. Numbers become `f64` only at the edges (parsing,
   formatting, plot points).
   - "Any N of M given, find the rest" is `domain::infer`: declare each quantity as a typed
     `Var`, list the ways it follows from others (`rule2(U, I, R, |i, r| Ok(i * r))`; `when2`
     for a formula that may not apply, such as a division by zero) and run `infer`. The order
     of the rules is their priority. See `ohms_law/calc.rs` and `level_shifter/inverting.rs`.
   - Values with a tolerance are `Interval`s; the worst case of a result that uses an input
     twice is found at the corners of the inputs (`report::corner`, `report::extremes`).
2. **The API** — `crates/ecw-core/src/api/<name>.rs` with `pub fn solve(req: &Form) -> Response`,
   and the module name added to `calculators!(…)` in `api/mod.rs`. That name is the id the UI
   uses. Design pages (fields on a schematic, results table, graph) build on `api/report.rs`:
   `DesignForm` parses the fields, `param_row`/`range_row`/`vout_rows` make the table and
   `transfer_plot` the graph.
3. **Texts** — the backend knows no languages. Every message is `msg!("<name>.some.id", arg = …)`
   in Rust and an entry `"<name>.some.id": "Text with {arg}"` in `ui/locales/en.js` and its
   translation in `ui/locales/ru.js`. Interface strings (titles, descriptions, labels) are English
   in the code and translated in `ru.js` under the English text. Symbols and units (`"R1 [Ω]"`)
   stay plain strings. The help is `ui/help/en.md` and `ui/help/ru.md`.
4. **The page** — `ui/plugins/<name>.js` exports a manifest (`id`, `title`, `category`, `doc`,
   `command: "<name>"`, …; see the comment on `DesignPage` in `ui/components/report.js`) and is
   added to `MANIFESTS` in `ui/plugins/index.js`. A schematic is drawn with
   `ui/components/schematic.js`; `sch.place("<field key>", …)` puts a field on it.
5. **Tests** — unit tests next to the code, and golden cases in `crates/ecw-core/tests/golden.rs`
   (`check("<name>", "case", request(json!({…})))`). Create their files with
   `UPDATE_GOLDEN=1 cargo test -p ecw-core --test golden` and review them before committing.

### What the tests check for you

- `golden` — the replies of every calculator are unchanged, and every calculator has cases.
- `locale_keys` — every message id in the Rust sources exists in `ui/locales/en.js`, and every
  message of `en.js` is translated in `ru.js`.
- `ui_contract` — every calculator a page asks for exists in the API and every API calculator
  has a page; page ids are distinct and every manifest is registered in `index.js`.
- In the app, a design page whose schematic slots and backend fields do not match shows the
  mismatch in red at the top.

## Style

- Comments explain why, in plain English; names say what.
- Keep calculations on types, conversions at the edges.
- A change that alters results should alter golden files only where intended: look at the diff.
