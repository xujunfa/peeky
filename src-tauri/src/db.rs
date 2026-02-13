use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_app_settings_table",
        sql: include_str!("../migrations/001_init.sql"),
        kind: MigrationKind::Up,
    }]
}

pub async fn create_pool(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let db_url = format!("sqlite:{}?mode=rwc", db_path);
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
