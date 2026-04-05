// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod apps;

use apps::AppSuggestion;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn search_apps(query: String) -> Result<Vec<AppSuggestion>, String> {
    apps::search_apps(query)
}

#[tauri::command]
fn launch_app(desktop_file_path: String) -> Result<(), String> {
    apps::launch_desktop_file(desktop_file_path)
}

#[tauri::command]
fn hide_main_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;
    window.hide().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "linux")]
            {
                use tauri_plugin_global_shortcut::{Builder, ShortcutState};

                app.handle().plugin(
                    Builder::new()
                        .with_shortcuts(["ctrl+space"])?
                        .with_handler(|app, _shortcut, event| {
                            if event.state != ShortcutState::Pressed {
                                return;
                            }
                            let Some(window) = app.get_webview_window("main") else {
                                return;
                            };
                            let show = match window.is_visible() {
                                Ok(v) => !v,
                                Err(_) => true,
                            };
                            if show {
                                let _ = window.unminimize();
                                let _ = window.center();
                                let _ = window.show();
                                let _ = window.set_focus();
                            } else {
                                let _ = window.hide();
                            }
                        })
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            hide_main_window,
            search_apps,
            launch_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
