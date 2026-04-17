// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod apps;
mod calculator;

use serde::Serialize;
use tauri::Manager;
use apps::{search_apps, AppSuggestion};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn hide_launcher(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?
        .hide()
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LauncherSearchResult {
    apps: Vec<AppSuggestion>,
    calculator: Option<String>,
}

#[tauri::command]
fn launcher_search(query: &str) -> Result<LauncherSearchResult, String> {
    Ok(LauncherSearchResult {
        apps: search_apps(query.to_string())?,
        calculator: calculator::try_eval(query),
    })
}

#[tauri::command]
fn launch_desktop_file(path: String) -> Result<(), String> {
    apps::launch_desktop_file(path)
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn global_shortcut_plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_global_shortcut::{Builder as ShortcutBuilder, ShortcutState};

    ShortcutBuilder::new()
        .with_shortcut("ctrl+space")
        .expect("register ctrl+space shortcut")
        .with_handler(|app, _shortcut, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        })
        .build()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder.plugin(global_shortcut_plugin());
    }

    builder
        .invoke_handler(tauri::generate_handler![greet, hide_launcher, launcher_search, launch_desktop_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
