mod commands;
mod db;

use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[tauri::command]
fn update_tray_title(app: tauri::AppHandle, title: String) {
    if let Some(tray) = app.tray_by_id("main-tray") {
        let _ = tray.set_title(Some(&title));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:starter.db", db::migrations())
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin({
            let toggle_timer = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyO);
            let toggle_main = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyL);
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return;
                    }
                    if shortcut == &toggle_timer {
                        if let Some(w) = app.get_webview_window("timer") {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    } else if shortcut == &toggle_main {
                        if let Some(w) = app.get_webview_window("main") {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    }
                })
                .build()
        })
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();
            let db_path = app_data_dir.join("starter.db");
            let db_path_str = db_path.to_string_lossy().to_string();

            let pool = tauri::async_runtime::block_on(db::create_pool(&db_path_str))
                .expect("failed to create database pool");
            app.manage(pool);

            if let Some(main_window) = app.get_webview_window("main") {
                main_window.hide().unwrap();
            }

            let tray_icon = app.default_window_icon().cloned().unwrap();
            TrayIconBuilder::with_id("main-tray")
                .icon(tray_icon)
                .icon_as_template(true)
                .tooltip("tauri-mac-starter")
                .title("tauri-mac-starter")
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            app.global_shortcut()
                .register(Shortcut::new(
                    Some(Modifiers::SUPER | Modifiers::SHIFT),
                    Code::KeyO,
                ))
                .expect("failed to register Cmd+Shift+O");
            app.global_shortcut()
                .register(Shortcut::new(
                    Some(Modifiers::SUPER | Modifiers::SHIFT),
                    Code::KeyL,
                ))
                .expect("failed to register Cmd+Shift+L");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::ping,
            commands::app::get_app_info,
            commands::settings::get_settings,
            commands::settings::set_settings,
            update_tray_title,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
