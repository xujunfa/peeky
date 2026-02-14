mod commands;
mod db;

use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Configure the overlay NSWindow so it can appear above fullscreen apps.
///
/// Based on KeyClu's approach (reverse-engineered):
/// 1. collectionBehavior: moveToActiveSpace + stationary + ignoresCycle
///    + fullScreenAuxiliary + auxiliary
/// 2. window level: floating (3)
/// 3. Temporarily switch activation policy to Regular, activate the app,
///    bring window to front, then switch back to Accessory.
#[cfg(target_os = "macos")]
fn configure_overlay_for_fullscreen(window: &tauri::WebviewWindow) {
    let _ = window.with_webview(|webview| {
        use objc2::rc::Retained;
        use objc2::MainThreadMarker;
        use objc2_app_kit::{
            NSApplication, NSApplicationActivationPolicy, NSWindow,
            NSWindowCollectionBehavior,
        };

        unsafe {
            let ns_window_ptr = webview.ns_window();
            let ns_window: Retained<NSWindow> = Retained::retain(ns_window_ptr.cast()).unwrap();

            // floating level (3) — matches KeyClu's primary approach
            ns_window.setLevel(3);

            // Replicate KeyClu's collectionBehavior:
            //   moveToActiveSpace (1<<1) | stationary (1<<4) | ignoresCycle (1<<6)
            //   | fullScreenAuxiliary (1<<8) | auxiliary (1<<17)
            let behavior = NSWindowCollectionBehavior::MoveToActiveSpace
                | NSWindowCollectionBehavior::Stationary
                | NSWindowCollectionBehavior::IgnoresCycle
                | NSWindowCollectionBehavior::FullScreenAuxiliary
                | NSWindowCollectionBehavior::Auxiliary;
            ns_window.setCollectionBehavior(behavior);

            ns_window.setCanHide(false);

            // with_webview callback runs on the main thread
            let mtm = MainThreadMarker::new().unwrap();

            // Temporarily switch to Regular policy so the app can activate
            // and steal focus even over a fullscreen app, then revert to Accessory.
            let app = NSApplication::sharedApplication(mtm);
            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
            #[allow(deprecated)]
            app.activateIgnoringOtherApps(true);

            ns_window.orderFrontRegardless();
            ns_window.makeKeyAndOrderFront(None);

            // Switch back to Accessory so the Dock icon stays hidden.
            app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
        }
    });
}

/// Deactivate overlay: hide the window and ensure Accessory policy.
#[cfg(target_os = "macos")]
fn deactivate_overlay(window: &tauri::WebviewWindow) {
    let _ = window.hide();
    let _ = window.with_webview(|_webview| {
        use objc2::MainThreadMarker;
        use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

        let mtm = MainThreadMarker::new().unwrap();
        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
        app.deactivate();
    });
}

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
                .add_migrations("sqlite:peeky.db", db::migrations())
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin({
            let toggle_overlay =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyO);
            let toggle_main = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyL);
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return;
                    }
                    if shortcut == &toggle_overlay {
                        if let Some(w) = app.get_webview_window("overlay") {
                            if w.is_visible().unwrap_or(false) {
                                #[cfg(target_os = "macos")]
                                deactivate_overlay(&w);
                                #[cfg(not(target_os = "macos"))]
                                {
                                    let _ = w.hide();
                                }
                            } else {
                                let _ = w.show();
                                #[cfg(target_os = "macos")]
                                configure_overlay_for_fullscreen(&w);
                                #[cfg(not(target_os = "macos"))]
                                {
                                    let _ = w.set_visible_on_all_workspaces(true);
                                    let _ = w.set_always_on_top(true);
                                    let _ = w.set_focus();
                                }
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
            // Accessory policy (UIElement): hides Dock icon and allows overlay
            // windows to float above fullscreen apps on macOS 10.14+.
            // Peeky is a menu-bar app so no Dock icon is needed.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();
            let db_path = app_data_dir.join("peeky.db");
            let db_path_str = db_path.to_string_lossy().to_string();

            let pool = tauri::async_runtime::block_on(db::create_pool(&db_path_str))
                .expect("failed to create database pool");
            app.manage(pool);

            let tray_icon = app.default_window_icon().cloned().unwrap();
            TrayIconBuilder::with_id("main-tray")
                .icon(tray_icon)
                .icon_as_template(true)
                .tooltip("Peeky")
                .title("Peeky")
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

            if let Err(err) = app.global_shortcut().register(Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::ALT),
                Code::KeyO,
            )) {
                eprintln!("failed to register Ctrl+Option+O: {err}");
            }
            if let Err(err) = app.global_shortcut().register(Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::ALT),
                Code::KeyL,
            )) {
                eprintln!("failed to register Ctrl+Option+L: {err}");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::ping,
            commands::app::get_app_info,
            commands::settings::get_settings,
            commands::settings::set_settings,
            commands::categories::get_categories,
            commands::categories::create_category,
            commands::categories::update_category,
            commands::categories::delete_category,
            commands::categories::reorder_categories,
            commands::items::get_items,
            commands::items::get_all_items,
            commands::items::create_item,
            commands::items::update_item,
            commands::items::delete_item,
            update_tray_title,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod window_config_tests {
    use serde_json::Value;

    fn find_window<'a>(windows: &'a [Value], label: &str) -> &'a Value {
        windows
            .iter()
            .find(|window| window.get("label").and_then(Value::as_str) == Some(label))
            .unwrap_or_else(|| panic!("missing window config for label={label}"))
    }

    #[test]
    fn startup_window_visibility_and_overlay_mode_match_expected_behavior() {
        let config: Value = serde_json::from_str(include_str!("../tauri.conf.json"))
            .expect("valid tauri.conf.json");
        let windows = config
            .get("app")
            .and_then(|app| app.get("windows"))
            .and_then(Value::as_array)
            .expect("app.windows must be an array");

        let overlay = find_window(windows, "overlay");
        let main = find_window(windows, "main");

        assert_eq!(main.get("visible").and_then(Value::as_bool), Some(false));
        assert_eq!(overlay.get("visible").and_then(Value::as_bool), Some(false));
        assert_eq!(
            overlay.get("fullscreen").and_then(Value::as_bool),
            Some(false)
        );
        assert_eq!(
            overlay.get("transparent").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            overlay.get("center").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            overlay.get("alwaysOnTop").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            overlay
                .get("visibleOnAllWorkspaces")
                .and_then(Value::as_bool),
            Some(true)
        );
    }
}
