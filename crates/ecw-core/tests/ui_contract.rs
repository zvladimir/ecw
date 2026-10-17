//! The UI and the backend agree on the calculators: every backend id that a manifest in
//! `ui/plugins` asks for (`command: "id"` or `useSolver("id", …)`) exists in `api::ids()`, every
//! calculator of the API is used by some page, manifests have distinct ids, and every manifest
//! file is registered in `ui/plugins/index.js`.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

/// The string literals that follow `prefix` in `source`: `command: "x"` gives `x`.
fn literals_after<'a>(source: &'a str, prefix: &str) -> Vec<&'a str> {
    source
        .match_indices(prefix)
        .filter_map(|(at, _)| {
            let rest = &source[at + prefix.len()..];
            let end = rest.find('"')?;
            Some(&rest[..end])
        })
        .collect()
}

fn plugins() -> Vec<(String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../ui/plugins");
    let mut files: Vec<(String, String)> = std::fs::read_dir(&dir)
        .expect("ui/plugins")
        .map(|e| e.expect("entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "js"))
        .map(|p| {
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            (name, std::fs::read_to_string(&p).expect("plugin source"))
        })
        .collect();
    files.sort();
    files
}

#[test]
fn ui_and_backend_agree_on_the_calculators() {
    let backend: BTreeSet<&str> = ecw_core::api::ids().collect();
    let files = plugins();
    let mut used = BTreeSet::new();
    for (file, source) in &files {
        for id in literals_after(source, "command: \"")
            .into_iter()
            .chain(literals_after(source, "useSolver(\""))
        {
            assert!(
                backend.contains(id),
                "{file} asks for the calculator {id:?}, which the API does not have"
            );
            used.insert(id);
        }
    }
    let unused: Vec<_> = backend.difference(&used).collect();
    assert!(unused.is_empty(), "calculators without a page: {unused:?}");
}

#[test]
fn manifests_are_registered_and_have_distinct_ids() {
    let files = plugins();
    let index = &files
        .iter()
        .find(|(name, _)| name == "index.js")
        .expect("index.js")
        .1;
    let mut seen: HashMap<&str, &str> = HashMap::new();
    for (file, source) in &files {
        // A manifest is an object with `id: "…"` at the start of a line.
        let ids: Vec<&str> = source
            .lines()
            .filter_map(|line| line.trim_start().strip_prefix("id: \""))
            .filter_map(|rest| rest.split('"').next())
            .collect();
        if ids.is_empty() {
            continue;
        }
        assert!(
            index.contains(&format!("from \"./{file}\"")),
            "{file} is not imported in ui/plugins/index.js"
        );
        for id in ids {
            if let Some(other) = seen.insert(id, file) {
                panic!("the page id {id:?} is used in {other} and in {file}");
            }
        }
    }
}
