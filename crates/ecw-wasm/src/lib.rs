//! The browser twin of the Tauri `solve` command: the same calculators, JSON in and JSON out.

use wasm_bindgen::prelude::wasm_bindgen;

/// Runs the calculator `id` on a JSON request and returns its JSON response; an error is
/// the message of the failure, like the `Err(String)` of the desktop command.
#[wasm_bindgen]
pub fn solve(id: &str, request: &str) -> Result<String, String> {
    let request = serde_json::from_str(request).map_err(|e| e.to_string())?;
    let response = ecw_core::api::solve(id, request)?;
    serde_json::to_string(&response).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn unknown_calculator_is_an_error() {
        assert!(solve("nope", "{}").is_err());
    }

    #[test]
    fn broken_json_is_an_error() {
        assert!(solve("ohms_law", "{").is_err());
    }

    #[test]
    fn known_calculator_answers_with_json() {
        let response = solve("ohms_law", r#"{"decimals": 3}"#).unwrap();
        assert!(serde_json::from_str::<serde_json::Value>(&response).is_ok());
    }
}
