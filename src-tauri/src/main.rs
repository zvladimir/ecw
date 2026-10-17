//! The desktop shell of ECW: a Tauri window around the interface in `ui/`. It has two commands:
//! `solve` passes a JSON request to the calculators of `ecw-core`, `open_help` opens the help
//! window. The shell does no calculation and reads no files.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

const MAIN_WINDOW: &str = "main";
const HELP_WINDOW: &str = "help";

/// Runs the calculator `id`: the request and the response are JSON.
#[tauri::command]
fn solve(id: String, request: serde_json::Value) -> Result<serde_json::Value, String> {
    ecw_core::api::solve(&id, request)
}

/// Opens the help in its own window so it stays readable next to the calculators.
#[tauri::command]
async fn open_help(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(HELP_WINDOW) {
        return window.set_focus().map_err(|e| e.to_string());
    }
    WebviewWindowBuilder::new(&app, HELP_WINDOW, WebviewUrl::App("help.html".into()))
        .title("Help — ECW")
        .inner_size(680.0, 560.0)
        .min_inner_size(440.0, 320.0)
        .build()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![solve, open_help,])
        .on_window_event(|window, event| {
            // The app lives as long as its main window: closing it also closes the help window.
            if window.label() == MAIN_WINDOW && matches!(event, WindowEvent::CloseRequested { .. })
            {
                window.app_handle().exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run the application");
}
