use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub identifier: String,
    pub platform: String,
}

#[tauri::command]
pub fn ping() -> String {
    "pong".to_string()
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: "tauri-mac-starter".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        identifier: "com.tauri-mac-starter.app".to_string(),
        platform: std::env::consts::OS.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_app_info, ping, AppInfo};

    #[test]
    fn ping_returns_pong() {
        assert_eq!(ping(), "pong");
    }

    #[test]
    fn app_info_contains_required_fields() {
        let info: AppInfo = get_app_info();
        assert_eq!(info.name, "tauri-mac-starter");
        assert_eq!(info.version, "0.1.0");
        assert_eq!(info.identifier, "com.tauri-mac-starter.app");
        assert!(!info.platform.is_empty());
    }
}
