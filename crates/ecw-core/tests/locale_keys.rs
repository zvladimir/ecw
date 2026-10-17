//! Every message id written in the sources must exist in the UI's English dictionary
//! (`ui/locales/en.js`): a missing id would show up on screen as the id itself.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// The ids of the dictionary: the keys of its `"id": "text",` lines.
fn dictionary_ids(text: &str) -> HashSet<String> {
    text.lines()
        .filter_map(|line| line.trim_start().strip_prefix('"'))
        .filter_map(|rest| rest.split_once("\": ").map(|(id, _)| id.to_owned()))
        .collect()
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("readable dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn is_id(text: &str) -> bool {
    text.contains('.')
        && text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_')
}

/// String literals that look like an id (`"a.b"`, or `"t:a.b"` for a label) and start with the
/// first part of some id of the dictionary, so a typo in one cannot hide.
fn used_ids(source: &str, families: &HashSet<String>) -> Vec<String> {
    source
        .split('"')
        .skip(1)
        .step_by(2)
        .map(|literal| literal.strip_prefix("t:").unwrap_or(literal))
        .filter(|literal| {
            is_id(literal) && families.contains(literal.split('.').next().unwrap_or(""))
        })
        .map(str::to_owned)
        .collect()
}

#[test]
fn every_id_used_in_the_sources_is_in_the_dictionary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dictionary = dictionary_ids(
        &std::fs::read_to_string(root.join("../../ui/locales/en.js")).expect("en.js"),
    );
    assert!(
        dictionary.len() > 50,
        "the dictionary was not read: {} ids",
        dictionary.len()
    );
    let families: HashSet<String> = dictionary
        .iter()
        .filter_map(|id| id.split('.').next())
        .map(str::to_owned)
        .collect();
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);

    let mut missing = Vec::new();
    for file in files {
        let source = std::fs::read_to_string(&file).expect("source");
        // The unit tests at the end of a file may name ids by their prefix.
        let source = source.split("#[cfg(test)]").next().unwrap_or_default();
        for id in used_ids(source, &families) {
            if !dictionary.contains(&id) {
                missing.push(format!("{}: {id}", file.display()));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "ids missing from ui/locales/en.js:\n{}",
        missing.join("\n")
    );
}

/// Every message of the English dictionary is translated in each other language of the UI.
#[test]
fn every_message_is_translated() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../ui/locales");
    let english = dictionary_ids(&std::fs::read_to_string(root.join("en.js")).expect("en.js"));
    for language in ["ru.js", "eo.js"] {
        let translated =
            dictionary_ids(&std::fs::read_to_string(root.join(language)).expect(language));
        let mut missing: Vec<&String> = english.difference(&translated).collect();
        missing.sort();
        assert!(missing.is_empty(), "{language} lacks: {missing:?}");
    }
}
