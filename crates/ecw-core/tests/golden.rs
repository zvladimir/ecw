//! Golden tests of the calculator API: a fixed set of requests per calculator, the serialized
//! response of each compared with a file in `tests/golden/`. A refactor that keeps behaviour
//! leaves the files untouched; to accept an intended change run with `UPDATE_GOLDEN=1` and
//! review the diff.

use std::path::PathBuf;

use serde_json::{Map, Value, json};

/// A request of the given texts; a field left out reads as empty.
fn request(texts: Value) -> Value {
    let mut map = Map::new();
    map.insert("decimals".to_owned(), json!(4));
    for (key, value) in texts.as_object().expect("texts are an object") {
        map.insert(key.clone(), value.clone());
    }
    Value::Object(map)
}

fn check(calc: &str, case: &str, request: Value) {
    let response =
        ecw_core::api::solve(calc, request).unwrap_or_else(|e| panic!("{calc}/{case}: {e}"));
    let mut actual = serde_json::to_string_pretty(&response).expect("response serializes");
    actual.push('\n');

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(format!("{calc}__{case}.json"));
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        std::fs::create_dir_all(path.parent().expect("has a parent")).expect("create golden dir");
        std::fs::write(&path, &actual).expect("write golden file");
        return;
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "no golden file {}; run with UPDATE_GOLDEN=1",
            path.display()
        )
    });
    let expected_json: Value = serde_json::from_str(&expected)
        .unwrap_or_else(|e| panic!("golden file {} is not JSON: {e}", path.display()));
    let actual_json: Value = serde_json::from_str(&actual).expect("response is JSON");
    assert!(
        same(&expected_json, &actual_json),
        "{calc}/{case} differs from {}",
        path.display()
    );
}

/// Replies equal as JSON, with numbers compared to a relative 1e-12: the system `sin` of
/// Windows and of glibc may differ in the last bit, and the plots hold raw values of it.
fn same(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(a), Some(b)) => a == b || (a - b).abs() <= 1e-12 * a.abs().max(b.abs()),
            _ => a == b,
        },
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(key, a)| b.get(key).is_some_and(|b| same(a, b)))
        }
        _ => expected == actual,
    }
}

#[test]
fn ohms_law() {
    let case = |name, o| check("ohms_law", name, request(o));
    case("empty", json!({}));
    case("u_and_i", json!({"u": "12", "i": "2m"}));
    case("tolerance", json!({"u": "5 1%", "r": "1k 5%"}));
    case("u_and_r", json!({"u": "12", "r": "4"}));
    case("u_and_p", json!({"u": "12", "p": "36"}));
    case("i_and_r", json!({"i": "3", "r": "4"}));
    case("i_and_p", json!({"i": "3", "p": "36"}));
    case("r_and_p", json!({"r": "4", "p": "36"}));
    case("r_and_p_tolerance", json!({"r": "4 5%", "p": "36 2%"}));
    case("u_and_p_tolerance", json!({"u": "12 1%", "p": "36 2%"}));
    case("i_and_p_tolerance", json!({"i": "3 1%", "p": "36 2%"}));
    case("negative", json!({"u": "-5", "i": "1"}));
    case(
        "overdetermined",
        json!({"u": "1", "i": "1", "r": "1", "p": "1"}),
    );
    case("bad_input", json!({"u": "12 V"}));
}

#[test]
fn voltage_divider() {
    let case = |name, o| check("voltage_divider", name, request(o));
    case("empty", json!({"mode": "two"}));
    case(
        "two_from_ratio",
        json!({"mode": "two", "vin": "12", "vout1": "3.3", "r1": "10k"}),
    );
    case(
        "two_with_tolerance",
        json!({"mode": "two", "vin": "5", "r1": "10k 1%", "r2": "10k 1%"}),
    );
    case(
        "three",
        json!({"mode": "three", "vin": "12", "vout1": "8", "vout2": "4", "r1": "1k"}),
    );
    case("bad_input", json!({"mode": "two", "vin": "x"}));
    case(
        "two_vout_from_r",
        json!({"mode": "two", "vin": "12", "r1": "1k", "r2": "2k"}),
    );
    case(
        "two_r1_from_vout",
        json!({"mode": "two", "vin": "12", "vout1": "4", "r2": "2k"}),
    );
    case(
        "two_vin_from_vout",
        json!({"mode": "two", "vout1": "4", "r1": "1k", "r2": "2k"}),
    );
    case(
        "two_vout_above_vin",
        json!({"mode": "two", "vin": "5", "vout1": "6", "r1": "1k"}),
    );
    case(
        "two_tolerance_vout",
        json!({"mode": "two", "vin": "12 1%", "vout1": "4", "r1": "1k 1%"}),
    );
    // Vin = 12, R1 = 1k, R2 = 2k, R3 = 3k give Vout1 = 10 and Vout2 = 6.
    let all = [
        ("vin", "12"),
        ("vout1", "10"),
        ("vout2", "6"),
        ("r1", "1k"),
        ("r2", "2k"),
        ("r3", "3k"),
    ];
    let combos: [[usize; 4]; 15] = [
        [0, 1, 2, 3],
        [0, 1, 2, 4],
        [0, 1, 2, 5],
        [0, 1, 3, 4],
        [0, 1, 3, 5],
        [0, 1, 4, 5],
        [0, 2, 3, 4],
        [0, 2, 3, 5],
        [0, 2, 4, 5],
        [0, 3, 4, 5],
        [1, 3, 4, 5],
        [2, 3, 4, 5],
        [1, 2, 3, 4],
        [1, 2, 3, 5],
        [1, 2, 4, 5],
    ];
    for combo in combos {
        let mut o = serde_json::Map::new();
        o.insert("mode".into(), json!("three"));
        let mut name = String::from("three");
        for k in combo {
            o.insert(all[k].0.into(), json!(all[k].1));
            name.push('_');
            name.push_str(all[k].0);
        }
        check("voltage_divider", &name, request(Value::Object(o)));
    }
    case(
        "three_tolerance",
        json!({"mode": "three", "vin": "12 1%", "vout1": "10", "vout2": "6", "r1": "1k 1%"}),
    );
    case(
        "three_tolerance_resistors",
        json!({"mode": "three", "vin": "12", "r1": "1k 1%", "r2": "2k 1%", "r3": "3k 1%"}),
    );
    case(
        "three_vout2_above_vout1",
        json!({"mode": "three", "vin": "12", "vout1": "6", "vout2": "10", "r1": "1k"}),
    );
}

#[test]
fn opamp() {
    let case = |name, o| check("opamp", name, request(o));
    case("inverting_empty", json!({"topology": "inverting"}));
    case(
        "inverting_gain",
        json!({"topology": "inverting", "vin": "-1; 1", "r1": "1k", "r2": "5k"}),
    );
    case(
        "inverting_tolerance",
        json!({"topology": "inverting", "vin": "1", "gain": "-5", "r1": "1k 5%"}),
    );
    case(
        "non_inverting",
        json!({"topology": "non_inverting", "vin": "500m", "r1": "9k", "r2": "1k"}),
    );
    case(
        "non_inverting_tolerance",
        json!({"topology": "non_inverting", "vin": "1", "r1": "9k 1%", "r2": "1k 1%"}),
    );
    case(
        "positive_inverting_gain",
        json!({"topology": "inverting", "gain": "5"}),
    );
}

#[test]
fn opamp_differential() {
    let case = |name, o| check("opamp_differential", name, request(o));
    case("empty", json!({}));
    case(
        "two_ranges",
        json!({"vdiff": "-1.25; 1.25", "vo": "-2.5; 2.5", "vref": "0", "r1": "10k"}),
    );
    case(
        "tolerances",
        json!({"vdiff": "-1; 1", "vo": "-2; 2", "vref": "0 1%", "r1": "10k 1%", "r3": "20k 1%"}),
    );
    case(
        "range_and_resistors",
        json!({"vdiff": "1; 2", "vref": "0", "r1": "10k", "r3": "5k"}),
    );
    case(
        "r1_from_r3",
        json!({"vdiff": "-1; 1", "vo": "-2; 2", "vref": "0", "r3": "20k"}),
    );
    case(
        "one_number_ranges",
        json!({"vdiff": "1.25", "vo": "5", "vref": "0", "r1": "10k"}),
    );
    case(
        "gain_negative",
        json!({"vdiff": "-1; 1", "gain": "-2", "vref": "0", "r1": "10k"}),
    );
    case(
        "ranges_reversed",
        json!({"vdiff": "2; 1", "vo": "-2; 2", "vref": "0", "r1": "10k"}),
    );
    case(
        "r1_negative",
        json!({"vdiff": "-1; 1", "vo": "-2; 2", "vref": "0", "r1": "-5"}),
    );
    case(
        "r3_negative",
        json!({"vdiff": "-1; 1", "vo": "-2; 2", "vref": "0", "r1": "10k", "r3": "-5"}),
    );
    case(
        "gain_range_vref",
        json!({"vdiff": "-1; 1", "gain": "2", "vref": "1", "r1": "10k"}),
    );
}

/// A request of `fields` built from `base` with `extra` on top.
fn with(base: &Value, extra: Value) -> Value {
    let mut o = base.clone();
    for (k, v) in extra.as_object().expect("object") {
        o[k] = v.clone();
    }
    request(o)
}

#[test]
fn level_shifter() {
    let typical = json!({"vin": "-5; -1", "vout": "0.05; 3.3", "vref": "5", "r1": "845"});
    let case = |name, extra| check("level_shifter", name, with(&typical, extra));
    check("level_shifter", "empty", request(json!({})));
    case("typical", json!({}));
    case("standard_resistors", json!({"r2": "1.04k", "r3": "5.56k"}));
    case(
        "tolerances",
        json!({"vref": "5 0.5%", "r1": "845 1%", "r2": "1.04k 1%", "r3": "5.56k 1%"}),
    );
    case("r2_anchor", json!({"r1": "", "r2": "1.04k"}));
    case(
        "vout_from_gain",
        json!({"vout": "", "gain": "-0.8125", "r3": "5.541k"}),
    );
    case(
        "vin_from_resistors",
        json!({"vin": "", "r2": "1.04k", "r3": "5.56k"}),
    );
    case("one_number_ranges", json!({"vin": "-5", "vout": "3.3"}));
    case("no_r3", json!({"vin": "-2; -1", "vout": "2; 4"}));
    case("no_resistor", json!({"r1": ""}));
    case("not_enough", json!({"vout": ""}));
    case("vref_recommended", json!({"vref": ""}));
    case(
        "vref_recommended_negative",
        json!({"vin": "1", "vout": "3", "vref": ""}),
    );
    case("vref_required", json!({"vref": "", "r3": "5.541k"}));
    case(
        "vref_not_needed",
        json!({"vin": "-2; -1", "vout": "2; 4", "vref": ""}),
    );
    case("no_vref", json!({"vref": "", "vout": ""}));
    case("r1_negative", json!({"r1": "-1"}));
    case("gain_positive", json!({"gain": "2"}));
    case("empty_span", json!({"vin": "-1; -5"}));
    case("no_reference", json!({"vref": "0"}));
    case("wrong_reference_sign", json!({"vref": "-5"}));
}

#[test]
fn level_shifter_ni() {
    let typical =
        json!({"vin": "2; 5", "vout": "0.05; 4.95", "vref": "2.5", "r1": "1k", "r4": "1k"});
    let case = |name, extra| check("level_shifter_ni", name, with(&typical, extra));
    check("level_shifter_ni", "empty", request(json!({})));
    case("typical", json!({}));
    case("standard_resistors", json!({"r2": "777", "r3": "402"}));
    case(
        "tolerances",
        json!({"vref": "2.5 0.5%", "r1": "1k 1%", "r2": "777 1%", "r3": "402 1%", "r4": "1k 1%"}),
    );
    case(
        "r2_r3_anchors",
        json!({"r1": "", "r2": "777", "r3": "402", "r4": ""}),
    );
    case(
        "vout_from_resistors",
        json!({"vout": "", "r2": "777", "r3": "402"}),
    );
    case(
        "vin_from_gain",
        json!({"vin": "", "gain": "1.6333", "r2": "777.2"}),
    );
    case("no_divider_resistor", json!({"r4": ""}));
    case("not_enough", json!({"vout": ""}));
    case("r4_negative", json!({"r4": "-1"}));
    case("vref_recommended", json!({"vref": ""}));
    case("vref_required", json!({"vref": "", "r2": "777.2"}));
    case(
        "vref_no_offset",
        json!({"vin": "1", "vout": "3", "vref": ""}),
    );
    case("no_vref", json!({"vref": "", "vin": ""}));
    case(
        "gain_negative",
        json!({"vin": "", "gain": "-2", "r2": "777"}),
    );
    case("no_offset", json!({"vin": "1; 2", "vout": "1; 2"}));
    case("reference_sign", json!({"vref": "-2.5"}));
    case("r3_not_positive", json!({"vin": "2; 2.5"}));
    case("empty_span", json!({"vin": "5; 2"}));
    case("no_reference", json!({"vref": "0"}));
}

#[test]
fn current_shape() {
    let case = |name, o| check("current_shape", name, request(o));
    case("empty", json!({"kind": "rectangular"}));
    case(
        "rectangular",
        json!({"kind": "rectangular", "ip": "2", "ton": "1m", "period": "10m"}),
    );
    case(
        "half_sine",
        json!({"kind": "half_sine", "ip": "5", "ton": "2m", "period": "10m"}),
    );
    case(
        "trapezoidal",
        json!({"kind": "trapezoidal", "ip": "3", "ip2": "1", "ton": "4m", "period": "10m"}),
    );
    case(
        "triangular",
        json!({"kind": "triangular", "ip": "4", "period": "10m", "t1": "4m", "t2": "2m"}),
    );
    case(
        "bipolar_triangular",
        json!({"kind": "bipolar_triangular", "imin": "-2", "ip": "4", "period": "10m", "t1": "3m", "t2": "3m"}),
    );
}

#[test]
fn resistor_solver() {
    let case = |name, o| check("resistor_solver", name, request(o));
    case("empty", json!({"series": "E24"}));
    case("target", json!({"series": "E24", "target": "4.7k"}));
    case(
        "target_with_pool",
        json!({"series": "E24", "target": "4.7k", "pool": "1k; 10k"}),
    );
    case(
        "target_with_tolerance",
        json!({"series": "E96", "target": "3.3k 1%"}),
    );
}

#[test]
fn wire_awg() {
    check(
        "wire_awg",
        "basic",
        request(json!({"current": "2", "length": "1", "rth": "10"})),
    );
    check("wire_awg", "empty", request(json!({})));
}

/// Every calculator of the API has golden cases (a file `id__case.json`).
#[test]
fn every_calculator_has_golden_cases() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden");
    let files: Vec<String> = std::fs::read_dir(&dir)
        .expect("golden dir")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    for id in ecw_core::api::ids() {
        assert!(
            files.iter().any(|f| f.starts_with(&format!("{id}__"))),
            "no golden cases for {id}"
        );
    }
}
