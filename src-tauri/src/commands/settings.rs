use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppSettings {
    pub locale: String,
    pub launch_on_login: bool,
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            locale: "en-US".to_string(),
            launch_on_login: false,
            theme: "system".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSettingsInput {
    pub locale: Option<String>,
    pub launch_on_login: Option<bool>,
    pub theme: Option<String>,
}

pub async fn get_settings_by_pool(db: &SqlitePool) -> Result<AppSettings, String> {
    let settings = sqlx::query_as::<_, AppSettings>(
        "SELECT locale, launch_on_login, theme FROM app_settings WHERE id = 1",
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(settings.unwrap_or_default())
}

pub async fn set_settings_by_pool(
    db: &SqlitePool,
    input: SetSettingsInput,
) -> Result<AppSettings, String> {
    let current = get_settings_by_pool(db).await?;
    let merged = AppSettings {
        locale: input.locale.unwrap_or(current.locale),
        launch_on_login: input.launch_on_login.unwrap_or(current.launch_on_login),
        theme: input.theme.unwrap_or(current.theme),
    };

    sqlx::query(
        "INSERT INTO app_settings (id, locale, launch_on_login, theme) VALUES (1, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET locale = excluded.locale, launch_on_login = excluded.launch_on_login, theme = excluded.theme",
    )
    .bind(&merged.locale)
    .bind(merged.launch_on_login)
    .bind(&merged.theme)
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(merged)
}

#[tauri::command]
pub async fn get_settings(db: State<'_, SqlitePool>) -> Result<AppSettings, String> {
    get_settings_by_pool(db.inner()).await
}

#[tauri::command]
pub async fn set_settings(
    db: State<'_, SqlitePool>,
    input: SetSettingsInput,
) -> Result<AppSettings, String> {
    set_settings_by_pool(db.inner(), input).await
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::{get_settings_by_pool, set_settings_by_pool, AppSettings, SetSettingsInput};

    async fn setup_db() -> SqlitePool {
        let db = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("connect sqlite memory");
        sqlx::query(
            "CREATE TABLE app_settings (id INTEGER PRIMARY KEY CHECK (id = 1), locale TEXT NOT NULL DEFAULT 'en-US', launch_on_login INTEGER NOT NULL DEFAULT 0, theme TEXT NOT NULL DEFAULT 'system')",
        )
        .execute(&db)
        .await
        .expect("create app_settings table");
        db
    }

    #[tokio::test]
    async fn get_settings_returns_defaults_when_missing() {
        let db = setup_db().await;

        let settings: AppSettings = get_settings_by_pool(&db).await.expect("get settings");

        assert_eq!(settings.locale, "en-US");
        assert!(!settings.launch_on_login);
        assert_eq!(settings.theme, "system");
    }

    #[tokio::test]
    async fn set_settings_updates_only_provided_fields() {
        let db = setup_db().await;

        let updated = set_settings_by_pool(
            &db,
            SetSettingsInput {
                locale: Some("zh-CN".to_string()),
                launch_on_login: Some(true),
                theme: None,
            },
        )
        .await
        .expect("set settings");

        assert_eq!(updated.locale, "zh-CN");
        assert!(updated.launch_on_login);
        assert_eq!(updated.theme, "system");
    }
}
